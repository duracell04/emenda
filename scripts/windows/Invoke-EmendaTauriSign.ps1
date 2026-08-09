[CmdletBinding()]
param(
  [Parameter(Mandatory = $true)]
  [string] $SignToolPath,

  [Parameter(Mandatory = $true)]
  [string] $CertificateThumbprint,

  [Parameter(Mandatory = $true)]
  [string] $TimestampUrl,

  [Parameter(Mandatory = $true)]
  [string] $ProductName,

  [Parameter(Mandatory = $true)]
  [string] $ExpectedMainExecutable,

  [Parameter(Mandatory = $true)]
  [string] $NsisPayloadCapturePath,

  [Parameter(Mandatory = $true)]
  [string] $ArtifactPath
)

$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'EmendaSigning.Common.ps1')

Assert-EmendaWindowsHost
Assert-EmendaNonElevatedProcess

if ($TimestampUrl -cne 'http://timestamp.digicert.com') {
  throw "The Tauri signer requires the approved DigiCert RFC 3161 endpoint; found '$TimestampUrl'."
}
if ([string]::IsNullOrWhiteSpace($ProductName)) {
  throw 'The Tauri signer requires a non-empty product name.'
}

$normalizedThumbprint = ConvertTo-EmendaCertificateThumbprint -Thumbprint $CertificateThumbprint
[void] (Get-EmendaSigningCertificate -Thumbprint $normalizedThumbprint)
Assert-EmendaMicrosoftSignTool -Path $SignToolPath

$resolvedArtifactPath = [IO.Path]::GetFullPath($ArtifactPath)
$resolvedMainExecutable = [IO.Path]::GetFullPath($ExpectedMainExecutable)
$resolvedCapturePath = [IO.Path]::GetFullPath($NsisPayloadCapturePath)
$temporaryRoot = [IO.Path]::GetFullPath([IO.Path]::GetTempPath()).TrimEnd('\') + '\'
if (-not $resolvedCapturePath.StartsWith($temporaryRoot, [StringComparison]::OrdinalIgnoreCase)) {
  throw "The NSIS payload capture path must remain below the OS temporary directory: $resolvedCapturePath"
}
if (-not (Test-Path -LiteralPath $resolvedArtifactPath -PathType Leaf)) {
  throw "The Tauri signing target does not exist: $resolvedArtifactPath"
}

& $SignToolPath sign `
  /sha1 $normalizedThumbprint `
  /s My `
  /fd SHA256 `
  /tr $TimestampUrl `
  /td SHA256 `
  /d $ProductName `
  /v `
  $resolvedArtifactPath
if ($LASTEXITCODE -ne 0) {
  # SignTool documents exit code 2 for warnings, so every non-zero result is fatal.
  throw "SignTool signing failed for '$resolvedArtifactPath' with exit code $LASTEXITCODE."
}

[void] (Get-EmendaArtifactSignature `
  -Path $resolvedArtifactPath `
  -ExpectedThumbprint $normalizedThumbprint)

if ($resolvedArtifactPath -ieq $resolvedMainExecutable) {
  $binaryText = [Text.Encoding]::ASCII.GetString([IO.File]::ReadAllBytes($resolvedArtifactPath))
  $hasNsisMarker = $binaryText.Contains('__TAURI_BUNDLE_TYPE_VAR_NSS')
  $hasMsiMarker = $binaryText.Contains('__TAURI_BUNDLE_TYPE_VAR_MSI')
  if ($hasNsisMarker -and $hasMsiMarker) {
    throw 'The signed main executable contains both NSIS and MSI bundle markers.'
  }
  if ($hasNsisMarker) {
    if (Test-Path -LiteralPath $resolvedCapturePath) {
      throw "The NSIS payload capture already exists: $resolvedCapturePath"
    }
    [IO.File]::Copy($resolvedArtifactPath, $resolvedCapturePath, $false)
    $sourceHash = (Get-FileHash -LiteralPath $resolvedArtifactPath -Algorithm SHA256).Hash
    $captureHash = (Get-FileHash -LiteralPath $resolvedCapturePath -Algorithm SHA256).Hash
    if ($sourceHash -cne $captureHash) {
      throw 'The captured NSIS executable payload does not match the signed Tauri input.'
    }
  }
}
