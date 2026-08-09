[CmdletBinding(SupportsShouldProcess = $true, ConfirmImpact = 'Medium')]
param()

$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'EmendaSigning.Common.ps1')

Assert-EmendaWindowsHost
Assert-EmendaNonElevatedProcess

$matchingCertificates = @(
  Get-ChildItem -Path 'Cert:\CurrentUser\My' |
    Where-Object {
      $_.Subject -ceq $script:EmendaSigningSubject -or
      $_.FriendlyName -ceq $script:EmendaSigningFriendlyName
    }
)

if ($matchingCertificates.Count -gt 1) {
  throw 'Multiple Emenda local test-signing certificates exist. Resolve the duplicate certificates explicitly before continuing.'
}

if ($matchingCertificates.Count -eq 1) {
  $certificate = $matchingCertificates[0]
  Assert-EmendaSigningCertificate -Certificate $certificate
  Write-Verbose "Reusing valid non-exportable certificate $($certificate.Thumbprint) from Cert:\CurrentUser\My."
  Write-Output $certificate.Thumbprint
  return
}

if (-not $PSCmdlet.ShouldProcess('Cert:\CurrentUser\My', 'Create the Emenda local test code-signing certificate')) {
  return
}

$certificateNotBefore = (Get-Date).AddMinutes(-5)
$certificateParameters = @{
  Type = 'CodeSigningCert'
  Subject = $script:EmendaSigningSubject
  FriendlyName = $script:EmendaSigningFriendlyName
  CertStoreLocation = 'Cert:\CurrentUser\My'
  Provider = $script:EmendaSigningProvider
  KeyAlgorithm = 'RSA'
  KeyLength = 2048
  HashAlgorithm = 'SHA256'
  KeyUsage = 'DigitalSignature'
  KeyExportPolicy = 'NonExportable'
  NotBefore = $certificateNotBefore
  NotAfter = $certificateNotBefore.AddDays(365)
}

$certificate = $null
$createdCertificatePath = $null
try {
  $certificate = New-SelfSignedCertificate @certificateParameters
  $createdThumbprint = ConvertTo-EmendaCertificateThumbprint -Thumbprint $certificate.Thumbprint
  $createdCertificatePath = "Cert:\CurrentUser\My\$createdThumbprint"
  Assert-EmendaSigningCertificate -Certificate $certificate
} catch {
  $provisioningError = $_
  $rollbackError = $null
  if ($null -ne $certificate) {
    try {
      if ($null -eq $createdCertificatePath) {
        $createdThumbprint = ConvertTo-EmendaCertificateThumbprint -Thumbprint $certificate.Thumbprint
        $createdCertificatePath = "Cert:\CurrentUser\My\$createdThumbprint"
      }
      if (Test-Path -LiteralPath $createdCertificatePath -PathType Leaf -ErrorAction Stop) {
        Remove-Item -LiteralPath $createdCertificatePath -DeleteKey -Force -ErrorAction Stop
        Write-Verbose "Rolled back invalid certificate $createdThumbprint and its private key."
      }
    } catch {
      $rollbackError = $_
    }
  }

  if ($null -ne $rollbackError) {
    $combinedMessage = @(
      "Certificate provisioning failed: $($provisioningError.Exception.Message)"
      "Rollback also failed: $($rollbackError.Exception.Message)"
      "Review the exact certificate at '$createdCertificatePath' before retrying."
    ) -join "`n"
    throw [System.InvalidOperationException]::new($combinedMessage, $provisioningError.Exception)
  }
  throw $provisioningError
}

Write-Verbose "Created non-exportable certificate $($certificate.Thumbprint) in Cert:\CurrentUser\My."
Write-Output $certificate.Thumbprint
