Set-StrictMode -Version Latest

$script:EmendaSigningSubject = 'CN=Emenda V0.1 Local Test Signing'
$script:EmendaSigningFriendlyName = 'Emenda V0.1 Local Test Signing'
$script:EmendaCodeSigningEku = '1.3.6.1.5.5.7.3.3'
$script:EmendaTimeStampingEku = '1.3.6.1.5.5.7.3.8'
$script:EmendaSha256WithRsa = '1.2.840.113549.1.1.11'
$script:EmendaSigningProvider = 'Microsoft Software Key Storage Provider'

function Assert-EmendaWindowsHost {
  if ([Environment]::OSVersion.Platform -ne [PlatformID]::Win32NT) {
    throw 'Emenda Windows signing must run on Windows.'
  }
}

function Assert-EmendaNonElevatedProcess {
  $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
  try {
    $principal = New-Object Security.Principal.WindowsPrincipal($identity)
    if ($principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
      throw 'Emenda local test signing and certification must run from a non-elevated PowerShell session.'
    }
  } finally {
    $identity.Dispose()
  }
}

function ConvertTo-EmendaCertificateThumbprint {
  param(
    [Parameter(Mandatory = $true)]
    [string] $Thumbprint
  )

  $normalized = ($Thumbprint -replace '\s', '').ToUpperInvariant()
  if ($normalized -notmatch '^[0-9A-F]{40}$') {
    throw 'The signing certificate thumbprint must contain exactly 40 hexadecimal characters.'
  }

  $normalized
}

function Assert-EmendaSigningCertificate {
  param(
    [Parameter(Mandatory = $true)]
    [System.Security.Cryptography.X509Certificates.X509Certificate2] $Certificate,

    [int] $MinimumRemainingDays = 30
  )

  $normalizedThumbprint = ConvertTo-EmendaCertificateThumbprint -Thumbprint $Certificate.Thumbprint
  if ($Certificate.Subject -cne $script:EmendaSigningSubject) {
    throw "Certificate $normalizedThumbprint has unexpected subject '$($Certificate.Subject)'."
  }
  if ($Certificate.Issuer -cne $script:EmendaSigningSubject) {
    throw "Certificate $normalizedThumbprint is not the expected self-signed test certificate."
  }
  if ($Certificate.FriendlyName -cne $script:EmendaSigningFriendlyName) {
    throw "Certificate $normalizedThumbprint has unexpected friendly name '$($Certificate.FriendlyName)'."
  }
  if (-not $Certificate.HasPrivateKey) {
    throw "Certificate $normalizedThumbprint does not have an accessible private key."
  }

  $now = [DateTime]::UtcNow
  if ($Certificate.NotBefore.ToUniversalTime() -gt $now.AddMinutes(5)) {
    throw "Certificate $normalizedThumbprint is not valid yet."
  }
  if ($Certificate.NotAfter.ToUniversalTime() -lt $now.AddDays($MinimumRemainingDays)) {
    throw "Certificate $normalizedThumbprint expires too soon; at least $MinimumRemainingDays days must remain."
  }
  $validityDuration = $Certificate.NotAfter.ToUniversalTime() - $Certificate.NotBefore.ToUniversalTime()
  $minimumOneYearWindow = [TimeSpan]::FromDays(365) - [TimeSpan]::FromMinutes(10)
  $maximumOneYearWindow = [TimeSpan]::FromDays(365) + [TimeSpan]::FromMinutes(10)
  if ($validityDuration -lt $minimumOneYearWindow -or $validityDuration -gt $maximumOneYearWindow) {
    throw "Certificate $normalizedThumbprint does not use the required one-year validity window."
  }
  if ($Certificate.SignatureAlgorithm.Value -ne $script:EmendaSha256WithRsa) {
    throw "Certificate $normalizedThumbprint was not self-signed with RSA/SHA-256."
  }

  $ekuExtension = @(
    $Certificate.Extensions |
      Where-Object { $_.Oid.Value -eq '2.5.29.37' }
  ) | Select-Object -First 1
  if ($null -eq $ekuExtension) {
    throw "Certificate $normalizedThumbprint has no enhanced key usage extension."
  }

  $hasCodeSigningEku = $false
  foreach ($eku in $ekuExtension.EnhancedKeyUsages) {
    if ($eku.Value -eq $script:EmendaCodeSigningEku) {
      $hasCodeSigningEku = $true
      break
    }
  }
  if (-not $hasCodeSigningEku) {
    throw "Certificate $normalizedThumbprint is missing the Code Signing enhanced key usage."
  }

  $keyUsageExtension = @(
    $Certificate.Extensions |
      Where-Object { $_.Oid.Value -eq '2.5.29.15' }
  ) | Select-Object -First 1
  if ($null -eq $keyUsageExtension) {
    throw "Certificate $normalizedThumbprint has no key usage extension."
  }
  $digitalSignature = [System.Security.Cryptography.X509Certificates.X509KeyUsageFlags]::DigitalSignature
  if (($keyUsageExtension.KeyUsages -band $digitalSignature) -eq 0) {
    throw "Certificate $normalizedThumbprint cannot be used for digital signatures."
  }

  $privateKey = [System.Security.Cryptography.X509Certificates.RSACertificateExtensions]::GetRSAPrivateKey($Certificate)
  if ($null -eq $privateKey) {
    throw "Certificate $normalizedThumbprint does not have an RSA private key."
  }

  try {
    if ($privateKey.KeySize -ne 2048) {
      throw "Certificate $normalizedThumbprint does not use the required RSA-2048 key."
    }
    if ($privateKey -isnot [System.Security.Cryptography.RSACng]) {
      throw "Certificate $normalizedThumbprint does not use the required Windows CNG provider."
    }
    if ($privateKey.Key.Provider.Provider -cne $script:EmendaSigningProvider) {
      throw "Certificate $normalizedThumbprint uses unexpected provider '$($privateKey.Key.Provider.Provider)'."
    }
    if ($privateKey.Key.IsMachineKey) {
      throw "Certificate $normalizedThumbprint stores its private key in the machine key store."
    }
    if ($privateKey.Key.ExportPolicy -ne [System.Security.Cryptography.CngExportPolicies]::None) {
      throw "Certificate $normalizedThumbprint has an exportable private key."
    }
  } finally {
    $privateKey.Dispose()
  }
}

function Get-EmendaSigningCertificate {
  param(
    [Parameter(Mandatory = $true)]
    [string] $Thumbprint
  )

  $normalizedThumbprint = ConvertTo-EmendaCertificateThumbprint -Thumbprint $Thumbprint
  $certificatePath = "Cert:\CurrentUser\My\$normalizedThumbprint"
  if (-not (Test-Path -LiteralPath $certificatePath -PathType Leaf)) {
    throw "No signing certificate with thumbprint $normalizedThumbprint exists in Cert:\CurrentUser\My."
  }

  $certificate = Get-Item -LiteralPath $certificatePath
  Assert-EmendaSigningCertificate -Certificate $certificate
  $certificate
}

function Get-EmendaSignToolPath {
  # The V0.1 Surface release is x64. Do not let an inherited Tauri override
  # select an older or differently-architected tool for the certification run.
  $architectureDirectory = 'x64'

  $kitsRoots = New-Object 'System.Collections.Generic.HashSet[string]' ([StringComparer]::OrdinalIgnoreCase)
  $registryPaths = @(
    'Registry::HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows Kits\Installed Roots',
    'Registry::HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Microsoft\Windows Kits\Installed Roots'
  )
  foreach ($registryPath in $registryPaths) {
    try {
      $kitsRoot = (Get-ItemProperty -LiteralPath $registryPath -Name KitsRoot10 -ErrorAction Stop).KitsRoot10
      if (-not [string]::IsNullOrWhiteSpace($kitsRoot)) {
        [void] $kitsRoots.Add($kitsRoot)
      }
    } catch [System.Management.Automation.ItemNotFoundException] {
      continue
    } catch {
      if ($_.Exception.Message -notmatch 'cannot find path|does not exist') {
        throw
      }
    }
  }

  $candidates = @()
  foreach ($kitsRoot in $kitsRoots) {
    $binDirectory = Join-Path $kitsRoot 'bin'
    if (-not (Test-Path -LiteralPath $binDirectory -PathType Container)) {
      continue
    }
    $candidates += @(
      Get-ChildItem -Path (Join-Path $binDirectory "*\$architectureDirectory\signtool.exe") -File -ErrorAction SilentlyContinue
    )
    $legacyPath = Join-Path $binDirectory "$architectureDirectory\signtool.exe"
    if (Test-Path -LiteralPath $legacyPath -PathType Leaf) {
      $candidates += Get-Item -LiteralPath $legacyPath
    }
  }

  $signTool = @(
    $candidates |
      Sort-Object @{ Expression = {
        try { [Version] $_.Directory.Parent.Name } catch { [Version] '0.0' }
      }; Descending = $true }, FullName -Unique
  ) | Select-Object -First 1
  if ($null -eq $signTool) {
    throw 'signtool.exe was not found. Repair or install a Windows SDK before running the signed build.'
  }

  $signTool.FullName
}

function Get-EmendaTemporaryTrustPaths {
  param(
    [Parameter(Mandatory = $true)]
    [string] $Thumbprint
  )

  $normalizedThumbprint = ConvertTo-EmendaCertificateThumbprint -Thumbprint $Thumbprint
  [pscustomobject]@{
    Root = "Cert:\CurrentUser\Root\$normalizedThumbprint"
    TrustedPublisher = "Cert:\CurrentUser\TrustedPublisher\$normalizedThumbprint"
  }
}

function Assert-EmendaTemporaryTrustAbsent {
  param(
    [Parameter(Mandatory = $true)]
    [string] $Thumbprint
  )

  $trustPaths = Get-EmendaTemporaryTrustPaths -Thumbprint $Thumbprint
  $existingStores = New-Object 'System.Collections.Generic.List[string]'
  if (Test-Path -LiteralPath $trustPaths.Root -PathType Leaf) {
    $existingStores.Add('CurrentUser\Root')
  }
  if (Test-Path -LiteralPath $trustPaths.TrustedPublisher -PathType Leaf) {
    $existingStores.Add('CurrentUser\TrustedPublisher')
  }

  if ($existingStores.Count -gt 0) {
    throw "The exact Emenda test certificate is already trusted in $($existingStores -join ' and '). Remove that pre-existing trust explicitly before certification."
  }

  $trustPaths
}

function Get-EmendaCertUtilPath {
  $certUtilPath = [IO.Path]::GetFullPath((Join-Path $env:SystemRoot 'System32\certutil.exe'))
  if (-not (Test-Path -LiteralPath $certUtilPath -PathType Leaf)) {
    throw "The Windows certificate utility is missing: $certUtilPath"
  }

  $versionInfo = [Diagnostics.FileVersionInfo]::GetVersionInfo($certUtilPath)
  if ($versionInfo.CompanyName -cne 'Microsoft Corporation' -or
      $versionInfo.OriginalFilename -notin @('CertUtil.exe', 'CertUtil.exe.mui')) {
    throw "certutil.exe does not have the expected Microsoft file identity: $certUtilPath"
  }

  $signature = Get-AuthenticodeSignature -LiteralPath $certUtilPath
  if ($signature.Status -ne [System.Management.Automation.SignatureStatus]::Valid -or
      $null -eq $signature.SignerCertificate -or
      $signature.SignerCertificate.Subject -notmatch '(^|,\s*)O=Microsoft Corporation(,|$)') {
    throw "certutil.exe does not have a valid Microsoft Authenticode signature: $certUtilPath"
  }

  $certUtilPath
}

function Import-EmendaCurrentUserTrust {
  param(
    [Parameter(Mandatory = $true)]
    [string] $CertUtilPath,

    [Parameter(Mandatory = $true)]
    [string] $CertificatePath,

    [Parameter(Mandatory = $true)]
    [ValidateSet('Root', 'TrustedPublisher')]
    [string] $StoreName,

    [Parameter(Mandatory = $true)]
    [string] $ExpectedThumbprint
  )

  $normalizedThumbprint = ConvertTo-EmendaCertificateThumbprint -Thumbprint $ExpectedThumbprint
  $resolvedCertificatePath = [IO.Path]::GetFullPath($CertificatePath)
  if (-not (Test-Path -LiteralPath $resolvedCertificatePath -PathType Leaf)) {
    throw "The public certificate file does not exist: $resolvedCertificatePath"
  }

  $sourceCertificate = New-Object System.Security.Cryptography.X509Certificates.X509Certificate2 `
    -ArgumentList $resolvedCertificatePath
  try {
    if ((ConvertTo-EmendaCertificateThumbprint -Thumbprint $sourceCertificate.Thumbprint) -cne $normalizedThumbprint) {
      throw "The public certificate file does not match expected thumbprint $normalizedThumbprint."
    }
    if ($sourceCertificate.HasPrivateKey) {
      throw 'The temporary trust import source unexpectedly contains a private key.'
    }
  } finally {
    $sourceCertificate.Dispose()
  }

  $trustPaths = Get-EmendaTemporaryTrustPaths -Thumbprint $normalizedThumbprint
  $destinationPath = if ($StoreName -ceq 'Root') {
    $trustPaths.Root
  } else {
    $trustPaths.TrustedPublisher
  }
  if (Test-Path -LiteralPath $destinationPath -PathType Leaf) {
    throw "The exact Emenda certificate already exists in CurrentUser\$StoreName."
  }

  # Import-Certificate opens Windows' interactive root security warning. The
  # signed-release runners are deliberately non-interactive, so use Microsoft's
  # system utility with both current-user and force/non-prompting switches.
  $certUtilOutput = (& $CertUtilPath -user -f -addstore $StoreName $resolvedCertificatePath 2>&1 | Out-String)
  $certUtilExitCode = $LASTEXITCODE
  Write-Verbose $certUtilOutput
  if ($certUtilExitCode -ne 0) {
    throw "certutil.exe failed to import CurrentUser\$StoreName with exit code $certUtilExitCode."
  }
  if (-not (Test-Path -LiteralPath $destinationPath -PathType Leaf)) {
    throw "The exact Emenda public certificate is absent from CurrentUser\$StoreName after import."
  }

  $importedCertificate = Get-Item -LiteralPath $destinationPath
  if ((ConvertTo-EmendaCertificateThumbprint -Thumbprint $importedCertificate.Thumbprint) -cne $normalizedThumbprint -or
      $importedCertificate.Subject -cne $script:EmendaSigningSubject -or
      $importedCertificate.HasPrivateKey) {
    throw "The imported CurrentUser\$StoreName certificate does not match the expected public Emenda identity."
  }

  $importedCertificate
}

function Assert-EmendaMicrosoftSignTool {
  param(
    [Parameter(Mandatory = $true)]
    [string] $Path
  )

  if ([System.IO.Path]::GetFileName($Path) -ine 'signtool.exe') {
    throw "The configured signing tool is not named signtool.exe: $Path"
  }

  $versionInfo = [System.Diagnostics.FileVersionInfo]::GetVersionInfo($Path)
  if ($versionInfo.CompanyName -cne 'Microsoft Corporation' -or
      $versionInfo.OriginalFilename -ine 'signtool.exe') {
    throw "The configured signing tool does not have the expected Microsoft SignTool file identity: $Path"
  }

  $signature = Get-AuthenticodeSignature -LiteralPath $Path
  if ($signature.Status -ne [System.Management.Automation.SignatureStatus]::Valid) {
    throw "signtool.exe does not have a valid Microsoft Authenticode signature: $Path"
  }
  if ($null -eq $signature.SignerCertificate -or
      $signature.SignerCertificate.Subject -notmatch '(^|,\s*)O=Microsoft Corporation(,|$)') {
    throw "signtool.exe was not signed by Microsoft Corporation: $Path"
  }
}

function Assert-EmendaDigiCertTimestampCertificate {
  param(
    [Parameter(Mandatory = $true)]
    [System.Security.Cryptography.X509Certificates.X509Certificate2] $Certificate,

    [Parameter(Mandatory = $true)]
    [string] $ArtifactPath
  )

  $digiCertName = 'DigiCert'
  $hasDigiCertIdentity =
    $Certificate.Subject.IndexOf($digiCertName, [StringComparison]::OrdinalIgnoreCase) -ge 0 -and
    $Certificate.Issuer.IndexOf($digiCertName, [StringComparison]::OrdinalIgnoreCase) -ge 0
  if (-not $hasDigiCertIdentity) {
    throw "Artifact timestamp was not issued by DigiCert: $ArtifactPath"
  }

  $ekuExtension = @(
    $Certificate.Extensions |
      Where-Object { $_.Oid.Value -eq '2.5.29.37' }
  ) | Select-Object -First 1
  if ($null -eq $ekuExtension) {
    throw "Artifact timestamper has no enhanced key usage extension: $ArtifactPath"
  }

  $hasTimeStampingEku = $false
  foreach ($eku in $ekuExtension.EnhancedKeyUsages) {
    if ($eku.Value -eq $script:EmendaTimeStampingEku) {
      $hasTimeStampingEku = $true
      break
    }
  }
  if (-not $hasTimeStampingEku) {
    throw "Artifact timestamper is missing the Time Stamping enhanced key usage: $ArtifactPath"
  }
}

function Get-EmendaArtifactSignature {
  param(
    [Parameter(Mandatory = $true)]
    [string] $Path,

    [Parameter(Mandatory = $true)]
    [string] $ExpectedThumbprint,

    [switch] $RequireTrusted
  )

  $normalizedExpectedThumbprint = ConvertTo-EmendaCertificateThumbprint -Thumbprint $ExpectedThumbprint
  $signature = Get-AuthenticodeSignature -LiteralPath $Path
  if ($signature.Status -eq [System.Management.Automation.SignatureStatus]::NotSigned) {
    throw "Artifact is not Authenticode-signed: $Path"
  }
  if ($signature.Status -eq [System.Management.Automation.SignatureStatus]::HashMismatch) {
    throw "Artifact signature has a hash mismatch: $Path"
  }
  if ($signature.SignatureType.ToString() -cne 'Authenticode') {
    throw "Artifact does not have an Authenticode signature: $Path"
  }
  if ($null -eq $signature.SignerCertificate) {
    throw "Artifact has no signer certificate: $Path"
  }

  $actualThumbprint = ConvertTo-EmendaCertificateThumbprint -Thumbprint $signature.SignerCertificate.Thumbprint
  if ($actualThumbprint -cne $normalizedExpectedThumbprint) {
    throw "Artifact signer mismatch for '$Path'. Expected $normalizedExpectedThumbprint, found $actualThumbprint."
  }
  if ($signature.SignerCertificate.Subject -cne $script:EmendaSigningSubject) {
    throw "Artifact has unexpected signer subject '$($signature.SignerCertificate.Subject)': $Path"
  }
  if ($null -eq $signature.TimeStamperCertificate) {
    throw "Artifact has no RFC 3161 timestamp: $Path"
  }
  Assert-EmendaDigiCertTimestampCertificate `
    -Certificate $signature.TimeStamperCertificate `
    -ArtifactPath $Path
  if ($RequireTrusted -and $signature.Status -ne [System.Management.Automation.SignatureStatus]::Valid) {
    throw "Artifact did not validate while temporary trust was active: $Path ($($signature.StatusMessage))"
  }

  $signature
}

function Invoke-EmendaSignToolVerification {
  param(
    [Parameter(Mandatory = $true)]
    [string] $SignToolPath,

    [Parameter(Mandatory = $true)]
    [string] $ArtifactPath
  )

  & $SignToolPath verify /pa /all /v /tw $ArtifactPath
  if ($LASTEXITCODE -ne 0) {
    # SignTool documents exit code 2 for warnings, so every warning is fatal.
    throw "SignTool verification failed for '$ArtifactPath' with exit code $LASTEXITCODE."
  }
}

function Write-EmendaUtf8File {
  param(
    [Parameter(Mandatory = $true)]
    [string] $Path,

    [Parameter(Mandatory = $true)]
    [string] $Content
  )

  $encoding = New-Object System.Text.UTF8Encoding($false)
  [System.IO.File]::WriteAllText($Path, $Content, $encoding)
}
