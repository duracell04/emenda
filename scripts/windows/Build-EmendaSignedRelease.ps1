[CmdletBinding()]
param(
  [string] $CertificateThumbprint = $env:EMENDA_WINDOWS_SIGNING_THUMBPRINT,

  [switch] $PreflightOnly
)

$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'EmendaSigning.Common.ps1')

function Get-EmendaRustTarget {
  $rustVersion = @(& rustc -vV)
  if ($LASTEXITCODE -ne 0) {
    throw 'rustc -vV failed.'
  }

  $hostLine = $rustVersion | Where-Object { $_ -match '^host:\s+' } | Select-Object -First 1
  if ($null -eq $hostLine) {
    throw 'rustc did not report its host target.'
  }
  $hostTarget = ($hostLine -replace '^host:\s+', '').Trim()
  if ($hostTarget -cne 'x86_64-pc-windows-msvc') {
    throw "The Surface V0.1 signed build requires x86_64-pc-windows-msvc; found '$hostTarget'."
  }

  [pscustomobject]@{
    Host = $hostTarget
    BundleArchitecture = 'x64'
  }
}

function Get-SingleFreshEmendaArtifact {
  param(
    [Parameter(Mandatory = $true)]
    [string] $Directory,

    [Parameter(Mandatory = $true)]
    [string] $Filter,

    [Parameter(Mandatory = $true)]
    [DateTime] $NotOlderThanUtc,

    [Parameter(Mandatory = $true)]
    [string] $Label
  )

  $artifacts = @(
    Get-ChildItem -LiteralPath $Directory -Filter $Filter -File -ErrorAction Stop |
      Where-Object { $_.LastWriteTimeUtc -ge $NotOlderThanUtc }
  )
  if ($artifacts.Count -ne 1) {
    throw "Expected one fresh $Label artifact matching '$Filter' in '$Directory'; found $($artifacts.Count)."
  }

  $artifacts[0].FullName
}

function Invoke-EmendaTamperTest {
  param(
    [Parameter(Mandatory = $true)]
    [string] $SignToolPath,

    [Parameter(Mandatory = $true)]
    [string] $SourcePath,

    [Parameter(Mandatory = $true)]
    [string] $TamperedPath
  )

  [System.IO.File]::Copy($SourcePath, $TamperedPath, $false)
  $stream = [System.IO.File]::Open(
    $TamperedPath,
    [System.IO.FileMode]::Open,
    [System.IO.FileAccess]::ReadWrite,
    [System.IO.FileShare]::None
  )
  try {
    if ($stream.Length -le 8192) {
      throw 'The release executable is unexpectedly small for the deterministic tamper test.'
    }
    [void] $stream.Seek(4096, [System.IO.SeekOrigin]::Begin)
    $originalByte = $stream.ReadByte()
    if ($originalByte -lt 0) {
      throw 'The tamper test could not read the release executable.'
    }
    [void] $stream.Seek(4096, [System.IO.SeekOrigin]::Begin)
    $stream.WriteByte($originalByte -bxor 1)
  } finally {
    $stream.Dispose()
  }

  $tamperedSignature = Get-AuthenticodeSignature -LiteralPath $TamperedPath
  if ($tamperedSignature.Status -ne [System.Management.Automation.SignatureStatus]::HashMismatch) {
    throw "The tampered release executable reported '$($tamperedSignature.Status)' instead of HashMismatch."
  }

  $verificationOutput = (& $SignToolPath verify /pa /all /v $TamperedPath 2>&1 | Out-String)
  $tamperExitCode = $LASTEXITCODE
  Write-Verbose $verificationOutput
  if ($tamperExitCode -eq 0) {
    throw 'The tampered release executable unexpectedly passed Authenticode verification.'
  }
}

Assert-EmendaWindowsHost
Assert-EmendaNonElevatedProcess
if ([string]::IsNullOrWhiteSpace($CertificateThumbprint)) {
  throw 'Set EMENDA_WINDOWS_SIGNING_THUMBPRINT to the local test certificate thumbprint before running the signed build.'
}

$normalizedThumbprint = ConvertTo-EmendaCertificateThumbprint -Thumbprint $CertificateThumbprint
$certificate = Get-EmendaSigningCertificate -Thumbprint $normalizedThumbprint
$signToolPath = Get-EmendaSignToolPath
Assert-EmendaMicrosoftSignTool -Path $signToolPath
$certUtilPath = Get-EmendaCertUtilPath
$rustTarget = Get-EmendaRustTarget
$trustPaths = Assert-EmendaTemporaryTrustAbsent -Thumbprint $normalizedThumbprint
$rootTrustPath = $trustPaths.Root
$publisherTrustPath = $trustPaths.TrustedPublisher

$repositoryRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..\..')).Path
$worktreeStatus = @(& git -C $repositoryRoot status --porcelain=v1 --untracked-files=all)
if ($LASTEXITCODE -ne 0) {
  throw 'git status failed while checking the signed-build source tree.'
}
if ($worktreeStatus.Count -ne 0) {
  throw 'The signed release requires a clean Git worktree. Commit or remove every tracked and untracked source change first.'
}
$tauriCommand = Join-Path $repositoryRoot 'node_modules\.bin\tauri.cmd'
if (-not (Test-Path -LiteralPath $tauriCommand -PathType Leaf)) {
  throw 'The local Tauri CLI is missing. Run npm install before the signed build.'
}
$tauriSignerPath = Join-Path $PSScriptRoot 'Invoke-EmendaTauriSign.ps1'
$windowsPowerShellPath = Join-Path $env:SystemRoot 'System32\WindowsPowerShell\v1.0\powershell.exe'
if (-not (Test-Path -LiteralPath $tauriSignerPath -PathType Leaf)) {
  throw "The audited Tauri signing helper is missing: $tauriSignerPath"
}
if (-not (Test-Path -LiteralPath $windowsPowerShellPath -PathType Leaf)) {
  throw "Windows PowerShell 5.1 is missing: $windowsPowerShellPath"
}

if ($PreflightOnly) {
  Write-Host "Signing preflight passed for $($rustTarget.Host), certificate $normalizedThumbprint, and '$signToolPath'."
  return
}

$tauriConfiguration = Get-Content -Raw -LiteralPath (Join-Path $repositoryRoot 'src-tauri\tauri.conf.json') | ConvertFrom-Json
$productName = [string] $tauriConfiguration.productName
$productVersion = [string] $tauriConfiguration.version
if ([string]::IsNullOrWhiteSpace($productName) -or [string]::IsNullOrWhiteSpace($productVersion)) {
  throw 'The Tauri configuration must define productName and version.'
}

$temporaryIdentifier = "emenda-signing-$PID-$([Guid]::NewGuid().ToString('N'))"
$temporaryConfigurationPath = Join-Path ([System.IO.Path]::GetTempPath()) "$temporaryIdentifier.json"
$temporaryCertificatePath = Join-Path ([System.IO.Path]::GetTempPath()) "$temporaryIdentifier.cer"
$temporaryTamperedPath = Join-Path ([System.IO.Path]::GetTempPath()) "$temporaryIdentifier-tampered.exe"
$temporaryNsisPayloadPath = Join-Path ([System.IO.Path]::GetTempPath()) "$temporaryIdentifier-nsis-payload.exe"
$rootTrustImportAttempted = $false
$publisherTrustImportAttempted = $false
$previousSignToolPath = [Environment]::GetEnvironmentVariable('TAURI_WINDOWS_SIGNTOOL_PATH', 'Process')
$artifacts = @()
$releaseDirectory = Join-Path $repositoryRoot 'src-tauri\target\release'
$releaseExecutable = Join-Path $releaseDirectory 'emenda.exe'
$manifestPath = Join-Path $releaseDirectory 'bundle\emenda-test-signing-manifest.json'
$manifestContent = $null
$manifestArtifacts = @()
$operationError = $null
$cleanupErrors = New-Object 'System.Collections.Generic.List[string]'
$timestampUrl = 'http://timestamp.digicert.com'

try {
  # A manifest describes one completely verified run. Invalidate an earlier run
  # before Tauri can replace any of the artifacts that it described.
  if (Test-Path -LiteralPath $manifestPath -PathType Leaf -ErrorAction Stop) {
    Remove-Item -LiteralPath $manifestPath -Force -ErrorAction Stop
  }

  $temporaryConfiguration = [ordered]@{
    bundle = [ordered]@{
      windows = [ordered]@{
        certificateThumbprint = $normalizedThumbprint
        digestAlgorithm = 'sha256'
        timestampUrl = $timestampUrl
        tsp = $true
        signCommand = [ordered]@{
          cmd = $windowsPowerShellPath
          args = @(
            '-NoLogo',
            '-NoProfile',
            '-NonInteractive',
            '-ExecutionPolicy',
            'Bypass',
            '-File',
            $tauriSignerPath,
            '-SignToolPath',
            $signToolPath,
            '-CertificateThumbprint',
            $normalizedThumbprint,
            '-TimestampUrl',
            $timestampUrl,
            '-ProductName',
            $productName,
            '-ExpectedMainExecutable',
            $releaseExecutable,
            '-NsisPayloadCapturePath',
            $temporaryNsisPayloadPath,
            '-ArtifactPath',
            '%1'
          )
        }
      }
    }
  }
  Write-EmendaUtf8File -Path $temporaryConfigurationPath -Content ($temporaryConfiguration | ConvertTo-Json -Depth 4)

  [Environment]::SetEnvironmentVariable('TAURI_WINDOWS_SIGNTOOL_PATH', $signToolPath, 'Process')
  $buildStartedUtc = [DateTime]::UtcNow.AddSeconds(-5)
  Push-Location $repositoryRoot
  try {
    & $tauriCommand build --bundles 'nsis,msi' --config $temporaryConfigurationPath --ci
    if ($LASTEXITCODE -ne 0) {
      throw "The signed Tauri build failed with exit code $LASTEXITCODE."
    }
  } finally {
    Pop-Location
  }

  if (-not (Test-Path -LiteralPath $releaseExecutable -PathType Leaf)) {
    throw "The release executable was not produced: $releaseExecutable"
  }
  if ((Get-Item -LiteralPath $releaseExecutable).LastWriteTimeUtc -lt $buildStartedUtc) {
    throw "The release executable is stale: $releaseExecutable"
  }

  if (-not (Test-Path -LiteralPath $temporaryNsisPayloadPath -PathType Leaf)) {
    throw 'The custom signer did not capture the exact executable embedded in the NSIS installer.'
  }
  if ((Get-Item -LiteralPath $temporaryNsisPayloadPath).LastWriteTimeUtc -lt $buildStartedUtc) {
    throw "The captured NSIS executable payload is stale: $temporaryNsisPayloadPath"
  }

  # Tauri restores its unsigned, unpatched base executable after each installer.
  # Publish the signed NSIS-stage copy as release-exe so its manifest hash is the
  # exact byte stream installed by the authoritative current-user installer.
  [IO.File]::Copy($temporaryNsisPayloadPath, $releaseExecutable, $true)
  $capturedPayloadHash = (Get-FileHash -LiteralPath $temporaryNsisPayloadPath -Algorithm SHA256).Hash
  $releaseExecutableHash = (Get-FileHash -LiteralPath $releaseExecutable -Algorithm SHA256).Hash
  if ($capturedPayloadHash -cne $releaseExecutableHash) {
    throw 'The published release executable does not match the captured NSIS payload.'
  }

  $nsisPath = Get-SingleFreshEmendaArtifact `
    -Directory (Join-Path $releaseDirectory 'bundle\nsis') `
    -Filter "${productName}_${productVersion}_$($rustTarget.BundleArchitecture)-setup.exe" `
    -NotOlderThanUtc $buildStartedUtc `
    -Label 'NSIS installer'
  $msiPath = Get-SingleFreshEmendaArtifact `
    -Directory (Join-Path $releaseDirectory 'bundle\msi') `
    -Filter "${productName}_${productVersion}_$($rustTarget.BundleArchitecture)_*.msi" `
    -NotOlderThanUtc $buildStartedUtc `
    -Label 'MSI installer'

  $artifacts = @(
    [pscustomobject]@{ Label = 'release-exe'; Path = $releaseExecutable },
    [pscustomobject]@{ Label = 'nsis'; Path = $nsisPath },
    [pscustomobject]@{ Label = 'msi'; Path = $msiPath }
  )
  foreach ($artifact in $artifacts) {
    [void] (Get-EmendaArtifactSignature -Path $artifact.Path -ExpectedThumbprint $normalizedThumbprint)
  }

  [void] (Assert-EmendaTemporaryTrustAbsent -Thumbprint $normalizedThumbprint)
  Export-Certificate -Cert $certificate -FilePath $temporaryCertificatePath -Type CERT | Out-Null
  if (Test-Path -LiteralPath $rootTrustPath -PathType Leaf) {
    throw 'The exact Emenda certificate appeared in CurrentUser\Root before this script could import it.'
  }
  $rootTrustImportAttempted = $true
  [void] (Import-EmendaCurrentUserTrust `
    -CertUtilPath $certUtilPath `
    -CertificatePath $temporaryCertificatePath `
    -StoreName 'Root' `
    -ExpectedThumbprint $normalizedThumbprint)
  if (-not (Test-Path -LiteralPath $rootTrustPath -PathType Leaf)) {
    throw 'The exact Emenda public certificate is absent from CurrentUser\Root after import.'
  }
  if (Test-Path -LiteralPath $publisherTrustPath -PathType Leaf) {
    throw 'The exact Emenda certificate appeared in CurrentUser\TrustedPublisher before this script could import it.'
  }
  $publisherTrustImportAttempted = $true
  [void] (Import-EmendaCurrentUserTrust `
    -CertUtilPath $certUtilPath `
    -CertificatePath $temporaryCertificatePath `
    -StoreName 'TrustedPublisher' `
    -ExpectedThumbprint $normalizedThumbprint)
  if (-not (Test-Path -LiteralPath $publisherTrustPath -PathType Leaf)) {
    throw 'The exact Emenda public certificate is absent from CurrentUser\TrustedPublisher after import.'
  }

  foreach ($artifact in $artifacts) {
    [void] (Get-EmendaArtifactSignature `
      -Path $artifact.Path `
      -ExpectedThumbprint $normalizedThumbprint `
      -RequireTrusted)
    Invoke-EmendaSignToolVerification -SignToolPath $signToolPath -ArtifactPath $artifact.Path
  }

  Invoke-EmendaTamperTest `
    -SignToolPath $signToolPath `
    -SourcePath $releaseExecutable `
    -TamperedPath $temporaryTamperedPath

  $manifestArtifacts = @(
    foreach ($artifact in $artifacts) {
      $signature = Get-EmendaArtifactSignature `
        -Path $artifact.Path `
        -ExpectedThumbprint $normalizedThumbprint `
        -RequireTrusted
      [ordered]@{
        label = $artifact.Label
        path = $artifact.Path.Substring($repositoryRoot.Length).TrimStart('\')
        length = (Get-Item -LiteralPath $artifact.Path).Length
        sha256 = (Get-FileHash -LiteralPath $artifact.Path -Algorithm SHA256).Hash
        signerSubject = $signature.SignerCertificate.Subject
        signerThumbprint = $signature.SignerCertificate.Thumbprint
        timestampSubject = $signature.TimeStamperCertificate.Subject
        timestampThumbprint = $signature.TimeStamperCertificate.Thumbprint
      }
    }
  )
  $manifest = [ordered]@{
    schemaVersion = 1
    generatedAtUtc = [DateTime]::UtcNow.ToString('o')
    testSigningOnly = $true
    rustTarget = $rustTarget.Host
    artifacts = $manifestArtifacts
  }
  $manifestContent = $manifest | ConvertTo-Json -Depth 6
} catch {
  $operationError = $_
} finally {
  try {
    if ($publisherTrustImportAttempted) {
      try {
        if (Test-Path -LiteralPath $publisherTrustPath -PathType Leaf -ErrorAction Stop) {
          Remove-Item -LiteralPath $publisherTrustPath -Force -ErrorAction Stop
        }
      } catch {
        $cleanupErrors.Add("Could not remove temporary TrustedPublisher entry: $($_.Exception.Message)")
      }
    }

    if ($rootTrustImportAttempted) {
      try {
        if (Test-Path -LiteralPath $rootTrustPath -PathType Leaf -ErrorAction Stop) {
          Remove-Item -LiteralPath $rootTrustPath -Force -ErrorAction Stop
        }
      } catch {
        $cleanupErrors.Add("Could not remove temporary Root entry: $($_.Exception.Message)")
      }
    }

    foreach ($temporaryPath in @(
        $temporaryTamperedPath,
        $temporaryNsisPayloadPath,
        $temporaryCertificatePath,
        $temporaryConfigurationPath
      )) {
      try {
        if (Test-Path -LiteralPath $temporaryPath -PathType Leaf -ErrorAction Stop) {
          Remove-Item -LiteralPath $temporaryPath -Force -ErrorAction Stop
        }
      } catch {
        $cleanupErrors.Add("Could not remove temporary file '$temporaryPath': $($_.Exception.Message)")
      }
    }
  } finally {
    try {
      [Environment]::SetEnvironmentVariable('TAURI_WINDOWS_SIGNTOOL_PATH', $previousSignToolPath, 'Process')
    } catch {
      $cleanupErrors.Add("Could not restore TAURI_WINDOWS_SIGNTOOL_PATH: $($_.Exception.Message)")
    }
  }
}

if ($null -ne $operationError) {
  if ($cleanupErrors.Count -gt 0) {
    $combinedMessage = @(
      "Signed build failed: $($operationError.Exception.Message)"
      'Cleanup also failed:'
      $cleanupErrors
    ) -join "`n"
    throw [System.InvalidOperationException]::new($combinedMessage, $operationError.Exception)
  }
  throw $operationError
}

if ($cleanupErrors.Count -gt 0) {
  throw "Signed-build cleanup failed:`n$($cleanupErrors -join "`n")"
}

if (Test-Path -LiteralPath $rootTrustPath -PathType Leaf) {
  throw 'The temporary CurrentUser Root trust entry remained after cleanup.'
}
if (Test-Path -LiteralPath $publisherTrustPath -PathType Leaf) {
  throw 'The temporary CurrentUser TrustedPublisher entry remained after cleanup.'
}

foreach ($artifact in $artifacts) {
  # After removing local trust, chain status may correctly become untrusted. The
  # signer identity, embedded timestamp, and intact hash must still be present.
  [void] (Get-EmendaArtifactSignature -Path $artifact.Path -ExpectedThumbprint $normalizedThumbprint)
}

# The manifest content was captured before temporary trust cleanup. Ensure that
# every artifact still has exactly the captured payload before publishing that
# manifest as the success marker.
foreach ($artifact in $artifacts) {
  $capturedArtifacts = @(
    $manifestArtifacts | Where-Object { $_['label'] -ceq $artifact.Label }
  )
  if ($capturedArtifacts.Count -ne 1) {
    throw "The captured manifest does not contain exactly one '$($artifact.Label)' artifact."
  }
  $capturedArtifact = $capturedArtifacts[0]
  $currentLength = (Get-Item -LiteralPath $artifact.Path).Length
  $currentHash = (Get-FileHash -LiteralPath $artifact.Path -Algorithm SHA256).Hash.ToUpperInvariant()
  if ($currentLength -ne [long] $capturedArtifact['length'] -or
      $currentHash -cne ([string] $capturedArtifact['sha256']).ToUpperInvariant()) {
    throw "Artifact '$($artifact.Label)' changed after its signed-build manifest metadata was captured."
  }
}

if ([string]::IsNullOrWhiteSpace($manifestContent)) {
  throw 'The signed-build manifest was not generated.'
}

# Publish the success marker atomically only after trust cleanup and post-cleanup
# signature checks pass. Any failed run therefore leaves no success manifest.
$temporaryManifestPath = "$manifestPath.$PID-$([Guid]::NewGuid().ToString('N')).tmp"
try {
  Write-EmendaUtf8File -Path $temporaryManifestPath -Content $manifestContent
  [System.IO.File]::Move($temporaryManifestPath, $manifestPath)
} finally {
  if (Test-Path -LiteralPath $temporaryManifestPath -PathType Leaf) {
    Remove-Item -LiteralPath $temporaryManifestPath -Force -ErrorAction SilentlyContinue
  }
}

Write-Host 'Temporary CurrentUser Root and TrustedPublisher trust was removed.'
Write-Host 'Post-cleanup Windows chain status may be untrusted by design; signer identity and timestamp remain embedded.'
Write-Host "Signed test-release manifest: $manifestPath"
