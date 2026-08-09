[CmdletBinding()]
param(
  [string] $CertificateThumbprint = $env:EMENDA_WINDOWS_SIGNING_THUMBPRINT,

  [switch] $PreflightOnly
)

$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'EmendaSigning.Common.ps1')

function Get-EmendaPropertyValue {
  param(
    [Parameter(Mandatory = $true)]
    [object] $InputObject,

    [Parameter(Mandatory = $true)]
    [string] $Name,

    [switch] $Required
  )

  $property = $InputObject.PSObject.Properties[$Name]
  if ($null -eq $property) {
    if ($Required) {
      throw "Required property '$Name' is missing."
    }
    return $null
  }

  $property.Value
}

function Get-EmendaUninstallEntries {
  param(
    [Parameter(Mandatory = $true)]
    [string] $ProductName,

    [Parameter(Mandatory = $true)]
    [string] $ExpectedInstallDirectory
  )

  $roots = @(
    [pscustomobject]@{
      Scope = 'CurrentUser'
      Path = 'Registry::HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Uninstall'
    },
    [pscustomobject]@{
      Scope = 'CurrentUser'
      Path = 'Registry::HKEY_CURRENT_USER\Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall'
    },
    [pscustomobject]@{
      Scope = 'LocalMachine'
      Path = 'Registry::HKEY_LOCAL_MACHINE\Software\Microsoft\Windows\CurrentVersion\Uninstall'
    },
    [pscustomobject]@{
      Scope = 'LocalMachine'
      Path = 'Registry::HKEY_LOCAL_MACHINE\Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall'
    }
  )

  $escapedProductName = [Regex]::Escape($ProductName)
  foreach ($root in $roots) {
    if (-not (Test-Path -LiteralPath $root.Path -PathType Container)) {
      continue
    }

    foreach ($key in Get-ChildItem -LiteralPath $root.Path -ErrorAction Stop) {
      $entry = Get-ItemProperty -LiteralPath $key.PSPath -ErrorAction Stop
      $displayName = [string] (Get-EmendaPropertyValue -InputObject $entry -Name 'DisplayName')
      $displayVersion = [string] (Get-EmendaPropertyValue -InputObject $entry -Name 'DisplayVersion')
      $installLocation = [string] (Get-EmendaPropertyValue -InputObject $entry -Name 'InstallLocation')
      $uninstallString = [string] (Get-EmendaPropertyValue -InputObject $entry -Name 'UninstallString')
      $quietUninstallString = [string] (Get-EmendaPropertyValue -InputObject $entry -Name 'QuietUninstallString')
      $windowsInstallerValue = Get-EmendaPropertyValue -InputObject $entry -Name 'WindowsInstaller'

      $nameMatches = $displayName -match "^$escapedProductName(?:\s|$)"
      $pathMatches = $installLocation.IndexOf($ExpectedInstallDirectory, [StringComparison]::OrdinalIgnoreCase) -ge 0 -or
        $uninstallString.IndexOf($ExpectedInstallDirectory, [StringComparison]::OrdinalIgnoreCase) -ge 0 -or
        $quietUninstallString.IndexOf($ExpectedInstallDirectory, [StringComparison]::OrdinalIgnoreCase) -ge 0
      if ($nameMatches -or $pathMatches) {
        [pscustomobject]@{
          Scope = $root.Scope
          RegistryPath = $key.Name
          DisplayName = $displayName
          DisplayVersion = $displayVersion
          InstallLocation = $installLocation
          UninstallString = $uninstallString
          QuietUninstallString = $quietUninstallString
          WindowsInstaller = $windowsInstallerValue
        }
      }
    }
  }
}

function Assert-EmendaRegisteredUninstallCommand {
  param(
    [Parameter(Mandatory = $true)]
    [string] $Command,

    [Parameter(Mandatory = $true)]
    [string] $ExpectedUninstaller,

    [Parameter(Mandatory = $true)]
    [string] $Label,

    [switch] $AllowSilentArgument
  )

  if ([string]::IsNullOrWhiteSpace($Command)) {
    throw "The current-user uninstall registration has no $Label."
  }

  $expectedPath = [IO.Path]::GetFullPath($ExpectedUninstaller)
  $quotedExpectedPath = '"' + $expectedPath + '"'
  $allowedCommands = New-Object 'System.Collections.Generic.HashSet[string]' ([StringComparer]::OrdinalIgnoreCase)
  [void] $allowedCommands.Add($expectedPath)
  [void] $allowedCommands.Add($quotedExpectedPath)
  if ($AllowSilentArgument) {
    [void] $allowedCommands.Add("$expectedPath /S")
    [void] $allowedCommands.Add("$quotedExpectedPath /S")
  }

  $normalizedCommand = $Command.Trim()
  if (-not $allowedCommands.Contains($normalizedCommand)) {
    throw "The current-user $Label does not identify the exact expected NSIS uninstaller: $normalizedCommand"
  }
}

function Assert-EmendaNoInstallConflict {
  param(
    [Parameter(Mandatory = $true)]
    [string] $ProductName,

    [Parameter(Mandatory = $true)]
    [string] $ExpectedInstallDirectory,

    [Parameter(Mandatory = $true)]
    [string[]] $ShellPaths
  )

  $processes = @(Get-Process -Name 'emenda' -ErrorAction SilentlyContinue)
  if ($processes.Count -gt 0) {
    throw "An Emenda process is already running (PID(s): $($processes.Id -join ', ')). Close it explicitly before certification."
  }

  if (Test-Path -LiteralPath $ExpectedInstallDirectory) {
    throw "The expected current-user install directory already exists: $ExpectedInstallDirectory"
  }

  $existingShellPaths = @(
    $ShellPaths | Where-Object { Test-Path -LiteralPath $_ }
  )
  if ($existingShellPaths.Count -gt 0) {
    throw "An Emenda shell entry already exists: $($existingShellPaths -join ', ')"
  }

  $entries = @(
    Get-EmendaUninstallEntries `
      -ProductName $ProductName `
      -ExpectedInstallDirectory $ExpectedInstallDirectory
  )
  if ($entries.Count -gt 0) {
    throw "A conflicting Emenda uninstall registration already exists: $($entries.RegistryPath -join ', ')"
  }
}

function Assert-EmendaInstalledInventory {
  param(
    [Parameter(Mandatory = $true)]
    [string] $InstallDirectory,

    [Parameter(Mandatory = $true)]
    [string[]] $ExpectedFiles
  )

  if (-not (Test-Path -LiteralPath $InstallDirectory -PathType Container)) {
    throw "The expected install directory was not created: $InstallDirectory"
  }

  $expectedPaths = New-Object 'System.Collections.Generic.HashSet[string]' ([StringComparer]::OrdinalIgnoreCase)
  foreach ($expectedFile in $ExpectedFiles) {
    [void] $expectedPaths.Add([IO.Path]::GetFullPath($expectedFile))
  }

  $installedItems = @(Get-ChildItem -LiteralPath $InstallDirectory -Force -ErrorAction Stop)
  $unexpectedItems = @(
    $installedItems | Where-Object {
      $_.PSIsContainer -or -not $expectedPaths.Contains([IO.Path]::GetFullPath($_.FullName))
    }
  )
  if ($unexpectedItems.Count -gt 0) {
    throw "The installed directory contains unexpected development or payload files: $($unexpectedItems.Name -join ', ')"
  }
  if ($installedItems.Count -ne $expectedPaths.Count) {
    throw "The installed directory should contain exactly $($expectedPaths.Count) files; found $($installedItems.Count)."
  }
  foreach ($expectedPath in $expectedPaths) {
    if (-not (Test-Path -LiteralPath $expectedPath -PathType Leaf)) {
      throw "An expected installed file is missing: $expectedPath"
    }
  }
}

function Assert-EmendaShortcutTarget {
  param(
    [Parameter(Mandatory = $true)]
    [string] $ShortcutPath,

    [Parameter(Mandatory = $true)]
    [string] $ExpectedTarget,

    [Parameter(Mandatory = $true)]
    [string] $ShortcutLabel
  )

  if (-not (Test-Path -LiteralPath $ShortcutPath -PathType Leaf)) {
    throw "The expected $ShortcutLabel shortcut was not created: $ShortcutPath"
  }

  $shell = $null
  $shortcut = $null
  try {
    $shell = New-Object -ComObject WScript.Shell
    $shortcut = $shell.CreateShortcut($ShortcutPath)
    $targetPath = [Environment]::ExpandEnvironmentVariables([string] $shortcut.TargetPath)
    if ([string]::IsNullOrWhiteSpace($targetPath)) {
      throw "The $ShortcutLabel shortcut has no target: $ShortcutPath"
    }
    $resolvedTarget = [IO.Path]::GetFullPath($targetPath.Trim().Trim('"'))
    if ($resolvedTarget -ine [IO.Path]::GetFullPath($ExpectedTarget)) {
      throw "The $ShortcutLabel shortcut targets '$resolvedTarget' instead of '$ExpectedTarget'."
    }
    if (-not [string]::IsNullOrWhiteSpace([string] $shortcut.Arguments)) {
      throw "The $ShortcutLabel shortcut contains unexpected launch arguments: $($shortcut.Arguments)"
    }
  } finally {
    if ($null -ne $shortcut -and [Runtime.InteropServices.Marshal]::IsComObject($shortcut)) {
      [void] [Runtime.InteropServices.Marshal]::FinalReleaseComObject($shortcut)
    }
    if ($null -ne $shell -and [Runtime.InteropServices.Marshal]::IsComObject($shell)) {
      [void] [Runtime.InteropServices.Marshal]::FinalReleaseComObject($shell)
    }
  }
}

function Resolve-EmendaSignedManifest {
  param(
    [Parameter(Mandatory = $true)]
    [string] $ManifestPath,

    [Parameter(Mandatory = $true)]
    [string] $RepositoryRoot,

    [Parameter(Mandatory = $true)]
    [string] $ExpectedThumbprint
  )

  if (-not (Test-Path -LiteralPath $ManifestPath -PathType Leaf)) {
    throw "The signed-release manifest does not exist: $ManifestPath"
  }

  $manifest = Get-Content -Raw -LiteralPath $ManifestPath | ConvertFrom-Json
  $schemaVersion = Get-EmendaPropertyValue -InputObject $manifest -Name 'schemaVersion' -Required
  $testSigningOnly = Get-EmendaPropertyValue -InputObject $manifest -Name 'testSigningOnly' -Required
  $rustTarget = [string] (Get-EmendaPropertyValue -InputObject $manifest -Name 'rustTarget' -Required)
  $manifestArtifacts = @(
    Get-EmendaPropertyValue -InputObject $manifest -Name 'artifacts' -Required
  )

  if ($schemaVersion -isnot [int] -or $schemaVersion -ne 1) {
    throw "Unsupported signed-release manifest schema version '$schemaVersion'."
  }
  if ($testSigningOnly -isnot [bool] -or $testSigningOnly -ne $true) {
    throw 'The manifest is not marked as a local test-signing release.'
  }
  if ($rustTarget -cne 'x86_64-pc-windows-msvc') {
    throw "The Surface certification requires x86_64-pc-windows-msvc artifacts; found '$rustTarget'."
  }
  if ($manifestArtifacts.Count -ne 3) {
    throw "The manifest must contain exactly release-exe, nsis, and msi artifacts; found $($manifestArtifacts.Count)."
  }

  $expectedLabels = @('release-exe', 'nsis', 'msi')
  $repositoryPrefix = $RepositoryRoot.TrimEnd('\') + '\'
  $resolvedArtifacts = @{}
  foreach ($artifact in $manifestArtifacts) {
    $label = [string] (Get-EmendaPropertyValue -InputObject $artifact -Name 'label' -Required)
    if (-not ($expectedLabels -ccontains $label)) {
      throw "The manifest contains an unexpected artifact label '$label'."
    }
    if ($resolvedArtifacts.ContainsKey($label)) {
      throw "The manifest contains duplicate '$label' artifacts."
    }

    $relativePath = [string] (Get-EmendaPropertyValue -InputObject $artifact -Name 'path' -Required)
    if ([string]::IsNullOrWhiteSpace($relativePath) -or [IO.Path]::IsPathRooted($relativePath)) {
      throw "Manifest artifact '$label' does not use a repository-relative path."
    }
    $fullPath = [IO.Path]::GetFullPath((Join-Path $RepositoryRoot $relativePath))
    if (-not $fullPath.StartsWith($repositoryPrefix, [StringComparison]::OrdinalIgnoreCase)) {
      throw "Manifest artifact '$label' escapes the repository: $relativePath"
    }
    if (-not (Test-Path -LiteralPath $fullPath -PathType Leaf)) {
      throw "Manifest artifact '$label' does not exist: $fullPath"
    }

    $declaredLength = 0L
    $lengthText = [string] (Get-EmendaPropertyValue -InputObject $artifact -Name 'length' -Required)
    if (-not [long]::TryParse($lengthText, [ref] $declaredLength) -or $declaredLength -lt 1) {
      throw "Manifest artifact '$label' has an invalid length '$lengthText'."
    }
    $actualLength = (Get-Item -LiteralPath $fullPath).Length
    if ($actualLength -ne $declaredLength) {
      throw "Manifest length mismatch for '$label'. Expected $declaredLength, found $actualLength."
    }

    $declaredHash = ([string] (Get-EmendaPropertyValue -InputObject $artifact -Name 'sha256' -Required)).ToUpperInvariant()
    if ($declaredHash -notmatch '^[0-9A-F]{64}$') {
      throw "Manifest artifact '$label' has an invalid SHA-256 hash."
    }
    $actualHash = (Get-FileHash -LiteralPath $fullPath -Algorithm SHA256).Hash.ToUpperInvariant()
    if ($actualHash -cne $declaredHash) {
      throw "Manifest SHA-256 mismatch for '$label'."
    }

    $signature = Get-EmendaArtifactSignature `
      -Path $fullPath `
      -ExpectedThumbprint $ExpectedThumbprint
    $manifestSignerSubject = [string] (Get-EmendaPropertyValue -InputObject $artifact -Name 'signerSubject' -Required)
    $manifestSignerThumbprint = ConvertTo-EmendaCertificateThumbprint `
      -Thumbprint ([string] (Get-EmendaPropertyValue -InputObject $artifact -Name 'signerThumbprint' -Required))
    $manifestTimestampSubject = [string] (Get-EmendaPropertyValue -InputObject $artifact -Name 'timestampSubject' -Required)
    $manifestTimestampThumbprint = ConvertTo-EmendaCertificateThumbprint `
      -Thumbprint ([string] (Get-EmendaPropertyValue -InputObject $artifact -Name 'timestampThumbprint' -Required))
    $actualSignerThumbprint = ConvertTo-EmendaCertificateThumbprint -Thumbprint $signature.SignerCertificate.Thumbprint
    $actualTimestampThumbprint = ConvertTo-EmendaCertificateThumbprint -Thumbprint $signature.TimeStamperCertificate.Thumbprint
    if ($manifestSignerSubject -cne $signature.SignerCertificate.Subject -or
        $manifestSignerThumbprint -cne $actualSignerThumbprint -or
        $manifestTimestampSubject -cne $signature.TimeStamperCertificate.Subject -or
        $manifestTimestampThumbprint -cne $actualTimestampThumbprint) {
      throw "Manifest signature metadata mismatch for '$label'."
    }

    $resolvedArtifacts.Add($label, [pscustomobject]@{
      Label = $label
      Path = $fullPath
      Length = $actualLength
      Sha256 = $actualHash
    })
  }

  $expectedReleasePath = [IO.Path]::GetFullPath((Join-Path $RepositoryRoot 'src-tauri\target\release\emenda.exe'))
  $expectedNsisDirectory = [IO.Path]::GetFullPath((Join-Path $RepositoryRoot 'src-tauri\target\release\bundle\nsis')).TrimEnd('\') + '\'
  $expectedMsiDirectory = [IO.Path]::GetFullPath((Join-Path $RepositoryRoot 'src-tauri\target\release\bundle\msi')).TrimEnd('\') + '\'
  if ($resolvedArtifacts['release-exe'].Path -ine $expectedReleasePath) {
    throw "The release-exe manifest path is unexpected: $($resolvedArtifacts['release-exe'].Path)"
  }
  if (-not $resolvedArtifacts['nsis'].Path.StartsWith($expectedNsisDirectory, [StringComparison]::OrdinalIgnoreCase) -or
      [IO.Path]::GetExtension($resolvedArtifacts['nsis'].Path) -ine '.exe') {
    throw "The NSIS manifest path is unexpected: $($resolvedArtifacts['nsis'].Path)"
  }
  if (-not $resolvedArtifacts['msi'].Path.StartsWith($expectedMsiDirectory, [StringComparison]::OrdinalIgnoreCase) -or
      [IO.Path]::GetExtension($resolvedArtifacts['msi'].Path) -ine '.msi') {
    throw "The MSI manifest path is unexpected: $($resolvedArtifacts['msi'].Path)"
  }

  $resolvedArtifacts
}

function Assert-EmendaCurrentUserRegistration {
  param(
    [Parameter(Mandatory = $true)]
    [string] $ProductName,

    [Parameter(Mandatory = $true)]
    [string] $ExpectedInstallDirectory,

    [Parameter(Mandatory = $true)]
    [string] $ExpectedUninstaller,

    [Parameter(Mandatory = $true)]
    [string] $ExpectedVersion
  )

  $entries = @(
    Get-EmendaUninstallEntries `
      -ProductName $ProductName `
      -ExpectedInstallDirectory $ExpectedInstallDirectory
  )
  $currentUserEntries = @($entries | Where-Object { $_.Scope -ceq 'CurrentUser' })
  $machineEntries = @($entries | Where-Object { $_.Scope -ceq 'LocalMachine' })
  if ($machineEntries.Count -ne 0) {
    throw 'The NSIS certification unexpectedly created or found a LocalMachine Emenda registration.'
  }
  if ($currentUserEntries.Count -ne 1) {
    throw "Expected one current-user Emenda uninstall registration; found $($currentUserEntries.Count)."
  }

  $entry = $currentUserEntries[0]
  if ($entry.DisplayName -cne $ProductName) {
    throw "The current-user uninstall registration has unexpected DisplayName '$($entry.DisplayName)'."
  }
  if (-not [string]::IsNullOrWhiteSpace($entry.DisplayVersion) -and
      $entry.DisplayVersion -cne $ExpectedVersion) {
    throw "The current-user uninstall registration has unexpected DisplayVersion '$($entry.DisplayVersion)'."
  }
  if (([string] $entry.WindowsInstaller) -eq '1') {
    throw 'The installed registration is marked as Windows Installer/MSI instead of NSIS.'
  }
  if ([string]::IsNullOrWhiteSpace($entry.InstallLocation)) {
    throw 'The current-user uninstall registration has no InstallLocation.'
  }
  $registeredInstallDirectory = [IO.Path]::GetFullPath($entry.InstallLocation.Trim().Trim('"')).TrimEnd('\')
  if ($registeredInstallDirectory -ine [IO.Path]::GetFullPath($ExpectedInstallDirectory).TrimEnd('\')) {
    throw "The uninstall registration reports an unexpected install location '$($entry.InstallLocation)'."
  }

  Assert-EmendaRegisteredUninstallCommand `
    -Command $entry.UninstallString `
    -ExpectedUninstaller $ExpectedUninstaller `
    -Label 'UninstallString'
  if (-not [string]::IsNullOrWhiteSpace($entry.QuietUninstallString)) {
    Assert-EmendaRegisteredUninstallCommand `
      -Command $entry.QuietUninstallString `
      -ExpectedUninstaller $ExpectedUninstaller `
      -Label 'QuietUninstallString' `
      -AllowSilentArgument
  }
}

Assert-EmendaWindowsHost
Assert-EmendaNonElevatedProcess
if ([string]::IsNullOrWhiteSpace($CertificateThumbprint)) {
  throw 'Set EMENDA_WINDOWS_SIGNING_THUMBPRINT to the local test certificate thumbprint before NSIS certification.'
}

$normalizedThumbprint = ConvertTo-EmendaCertificateThumbprint -Thumbprint $CertificateThumbprint
$certificate = Get-EmendaSigningCertificate -Thumbprint $normalizedThumbprint
$signToolPath = Get-EmendaSignToolPath
Assert-EmendaMicrosoftSignTool -Path $signToolPath
$certUtilPath = Get-EmendaCertUtilPath
$repositoryRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..\..')).Path
$tauriConfigurationPath = Join-Path $repositoryRoot 'src-tauri\tauri.conf.json'
$tauriConfiguration = Get-Content -Raw -LiteralPath $tauriConfigurationPath | ConvertFrom-Json
$productName = [string] (Get-EmendaPropertyValue -InputObject $tauriConfiguration -Name 'productName' -Required)
$productVersion = [string] (Get-EmendaPropertyValue -InputObject $tauriConfiguration -Name 'version' -Required)
if ($productName -cne 'Emenda') {
  throw "The V0.1 certification expects productName 'Emenda'; found '$productName'."
}
if ($productVersion -cne '0.1.0') {
  throw "The V0.1 certification expects version '0.1.0'; found '$productVersion'."
}

$bundle = Get-EmendaPropertyValue -InputObject $tauriConfiguration -Name 'bundle' -Required
$windows = Get-EmendaPropertyValue -InputObject $bundle -Name 'windows'
$nsis = $null
$startMenuFolderName = ''
if ($null -ne $windows) {
  $nsis = Get-EmendaPropertyValue -InputObject $windows -Name 'nsis'
  if ($null -ne $nsis) {
    $installMode = [string] (Get-EmendaPropertyValue -InputObject $nsis -Name 'installMode')
    if (-not [string]::IsNullOrWhiteSpace($installMode) -and $installMode -cne 'currentUser') {
      throw "The authoritative NSIS installer must use currentUser install mode; found '$installMode'."
    }
    $startMenuFolderName = [string] (Get-EmendaPropertyValue -InputObject $nsis -Name 'startMenuFolder')
  }
}

$localAppData = [Environment]::GetFolderPath([Environment+SpecialFolder]::LocalApplicationData)
$roamingAppData = [Environment]::GetFolderPath([Environment+SpecialFolder]::ApplicationData)
$desktopDirectory = [Environment]::GetFolderPath([Environment+SpecialFolder]::DesktopDirectory)
if ([string]::IsNullOrWhiteSpace($localAppData) -or
    [string]::IsNullOrWhiteSpace($roamingAppData) -or
    [string]::IsNullOrWhiteSpace($desktopDirectory)) {
  throw 'Windows did not provide the current user application-data directories.'
}
$expectedInstallDirectory = Join-Path $localAppData 'Emenda'
$expectedInstalledExecutable = Join-Path $expectedInstallDirectory 'emenda.exe'
$expectedUninstaller = Join-Path $expectedInstallDirectory 'uninstall.exe'
$expectedDesktopShortcut = Join-Path $desktopDirectory 'Emenda.lnk'
$startMenuRoot = Join-Path $roamingAppData 'Microsoft\Windows\Start Menu\Programs'
if ([string]::IsNullOrWhiteSpace($startMenuFolderName)) {
  $expectedStartMenuShortcut = Join-Path $startMenuRoot 'Emenda.lnk'
  $expectedStartMenuFolder = $null
} else {
  if ([IO.Path]::IsPathRooted($startMenuFolderName)) {
    throw "The configured NSIS startMenuFolder must be relative: $startMenuFolderName"
  }
  $expectedStartMenuFolder = [IO.Path]::GetFullPath((Join-Path $startMenuRoot $startMenuFolderName))
  $startMenuPrefix = [IO.Path]::GetFullPath($startMenuRoot).TrimEnd('\') + '\'
  if (-not $expectedStartMenuFolder.StartsWith($startMenuPrefix, [StringComparison]::OrdinalIgnoreCase)) {
    throw "The configured NSIS startMenuFolder escapes the current user's Start menu: $startMenuFolderName"
  }
  $expectedStartMenuShortcut = Join-Path $expectedStartMenuFolder 'Emenda.lnk'
}
$shellConflictPaths = @(
  @(
    $expectedStartMenuFolder,
    $expectedStartMenuShortcut,
    (Join-Path $startMenuRoot 'Emenda.lnk'),
    $expectedDesktopShortcut
  ) | Where-Object { -not [string]::IsNullOrWhiteSpace($_) } | Select-Object -Unique
)
$manifestPath = Join-Path $repositoryRoot 'src-tauri\target\release\bundle\emenda-test-signing-manifest.json'
$artifacts = Resolve-EmendaSignedManifest `
  -ManifestPath $manifestPath `
  -RepositoryRoot $repositoryRoot `
  -ExpectedThumbprint $normalizedThumbprint
$expectedNsisName = "${productName}_${productVersion}_x64-setup.exe"
$expectedMsiPrefix = [Regex]::Escape("${productName}_${productVersion}_x64_")
$expectedMsiPattern = "^${expectedMsiPrefix}.+\.msi$"
if ([IO.Path]::GetFileName($artifacts['nsis'].Path) -ine $expectedNsisName) {
  throw "The manifest does not identify the expected V0.1 NSIS installer '$expectedNsisName'."
}
if ([IO.Path]::GetFileName($artifacts['msi'].Path) -notmatch $expectedMsiPattern) {
  throw 'The manifest does not identify the expected V0.1 x64 MSI artifact.'
}
$trustPaths = Assert-EmendaTemporaryTrustAbsent -Thumbprint $normalizedThumbprint
Assert-EmendaNoInstallConflict `
  -ProductName $productName `
  -ExpectedInstallDirectory $expectedInstallDirectory `
  -ShellPaths $shellConflictPaths

if ($PreflightOnly) {
  Write-Host "NSIS certification preflight passed for '$($artifacts['nsis'].Path)' and install path '$expectedInstallDirectory'."
  return
}

$temporaryIdentifier = "emenda-install-certification-$PID-$([Guid]::NewGuid().ToString('N'))"
$temporaryCertificatePath = Join-Path ([IO.Path]::GetTempPath()) "$temporaryIdentifier.cer"
$rootTrustImportAttempted = $false
$publisherTrustImportAttempted = $false
$operationError = $null
$cleanupErrors = New-Object 'System.Collections.Generic.List[string]'

try {
  [void] (Assert-EmendaTemporaryTrustAbsent -Thumbprint $normalizedThumbprint)
  Export-Certificate -Cert $certificate -FilePath $temporaryCertificatePath -Type CERT | Out-Null

  if (Test-Path -LiteralPath $trustPaths.Root -PathType Leaf) {
    throw 'The exact Emenda certificate appeared in CurrentUser\Root before this script could import it.'
  }
  $rootTrustImportAttempted = $true
  [void] (Import-EmendaCurrentUserTrust `
    -CertUtilPath $certUtilPath `
    -CertificatePath $temporaryCertificatePath `
    -StoreName 'Root' `
    -ExpectedThumbprint $normalizedThumbprint)
  if (-not (Test-Path -LiteralPath $trustPaths.Root -PathType Leaf)) {
    throw 'The exact Emenda public certificate is absent from CurrentUser\Root after import.'
  }

  if (Test-Path -LiteralPath $trustPaths.TrustedPublisher -PathType Leaf) {
    throw 'The exact Emenda certificate appeared in CurrentUser\TrustedPublisher before this script could import it.'
  }
  $publisherTrustImportAttempted = $true
  [void] (Import-EmendaCurrentUserTrust `
    -CertUtilPath $certUtilPath `
    -CertificatePath $temporaryCertificatePath `
    -StoreName 'TrustedPublisher' `
    -ExpectedThumbprint $normalizedThumbprint)
  if (-not (Test-Path -LiteralPath $trustPaths.TrustedPublisher -PathType Leaf)) {
    throw 'The exact Emenda public certificate is absent from CurrentUser\TrustedPublisher after import.'
  }

  # Re-read and revalidate the manifest after trust is established. This makes
  # the files verified below authoritative for this execution rather than the
  # potentially older objects captured by the read-only preflight.
  $artifacts = Resolve-EmendaSignedManifest `
    -ManifestPath $manifestPath `
    -RepositoryRoot $repositoryRoot `
    -ExpectedThumbprint $normalizedThumbprint
  if ([IO.Path]::GetFileName($artifacts['nsis'].Path) -ine $expectedNsisName) {
    throw "The execution-time manifest does not identify the expected V0.1 NSIS installer '$expectedNsisName'."
  }
  if ([IO.Path]::GetFileName($artifacts['msi'].Path) -notmatch $expectedMsiPattern) {
    throw 'The execution-time manifest does not identify the expected V0.1 x64 MSI artifact.'
  }

  foreach ($label in @('release-exe', 'nsis', 'msi')) {
    $artifact = $artifacts[$label]
    [void] (Get-EmendaArtifactSignature `
      -Path $artifact.Path `
      -ExpectedThumbprint $normalizedThumbprint `
      -RequireTrusted)
    Invoke-EmendaSignToolVerification -SignToolPath $signToolPath -ArtifactPath $artifact.Path
  }

  # Close the race between preflight and installer launch. Never remove or
  # replace a conflicting installation on behalf of the certification run.
  Assert-EmendaNoInstallConflict `
    -ProductName $productName `
    -ExpectedInstallDirectory $expectedInstallDirectory `
    -ShellPaths $shellConflictPaths

  $installerProcess = Start-Process `
    -FilePath $artifacts['nsis'].Path `
    -ArgumentList @('/S') `
    -WindowStyle Hidden `
    -Wait `
    -PassThru
  if ($installerProcess.ExitCode -ne 0) {
    throw "The current-user NSIS installer failed with exit code $($installerProcess.ExitCode)."
  }

  if (-not (Test-Path -LiteralPath $expectedInstalledExecutable -PathType Leaf)) {
    throw "The installed Emenda executable was not found: $expectedInstalledExecutable"
  }
  if (-not (Test-Path -LiteralPath $expectedUninstaller -PathType Leaf)) {
    throw "The signed NSIS uninstaller was not found: $expectedUninstaller"
  }
  Assert-EmendaInstalledInventory `
    -InstallDirectory $expectedInstallDirectory `
    -ExpectedFiles @($expectedInstalledExecutable, $expectedUninstaller)
  Assert-EmendaShortcutTarget `
    -ShortcutPath $expectedStartMenuShortcut `
    -ExpectedTarget $expectedInstalledExecutable `
    -ShortcutLabel 'current-user Start-menu'
  Assert-EmendaShortcutTarget `
    -ShortcutPath $expectedDesktopShortcut `
    -ExpectedTarget $expectedInstalledExecutable `
    -ShortcutLabel 'current-user Desktop'
  Assert-EmendaCurrentUserRegistration `
    -ProductName $productName `
    -ExpectedInstallDirectory $expectedInstallDirectory `
    -ExpectedUninstaller $expectedUninstaller `
    -ExpectedVersion $productVersion

  $installedHash = (Get-FileHash -LiteralPath $expectedInstalledExecutable -Algorithm SHA256).Hash.ToUpperInvariant()
  if ($installedHash -cne $artifacts['release-exe'].Sha256) {
    throw 'The installed Emenda executable does not match the signed release payload SHA-256.'
  }

  foreach ($installedPath in @($expectedInstalledExecutable, $expectedUninstaller)) {
    [void] (Get-EmendaArtifactSignature `
      -Path $installedPath `
      -ExpectedThumbprint $normalizedThumbprint `
      -RequireTrusted)
    Invoke-EmendaSignToolVerification -SignToolPath $signToolPath -ArtifactPath $installedPath
  }
} catch {
  $operationError = $_
} finally {
  foreach ($trustEntry in @(
    [pscustomobject]@{
      Label = 'TrustedPublisher'
      Path = $trustPaths.TrustedPublisher
      ImportAttempted = $publisherTrustImportAttempted
    },
    [pscustomobject]@{
      Label = 'Root'
      Path = $trustPaths.Root
      ImportAttempted = $rootTrustImportAttempted
    }
  )) {
    if ($trustEntry.ImportAttempted) {
      try {
        if (Test-Path -LiteralPath $trustEntry.Path -PathType Leaf -ErrorAction Stop) {
          Remove-Item -LiteralPath $trustEntry.Path -Force -ErrorAction Stop
        }
      } catch {
        $cleanupErrors.Add("Could not remove temporary $($trustEntry.Label) entry: $($_.Exception.Message)")
      }
    }
  }

  try {
    if (Test-Path -LiteralPath $temporaryCertificatePath -PathType Leaf -ErrorAction Stop) {
      Remove-Item -LiteralPath $temporaryCertificatePath -Force -ErrorAction Stop
    }
  } catch {
    $cleanupErrors.Add("Could not remove temporary public certificate '$temporaryCertificatePath': $($_.Exception.Message)")
  }
}

if ($null -ne $operationError) {
  if ($cleanupErrors.Count -gt 0) {
    $combinedMessage = @(
      "NSIS certification failed: $($operationError.Exception.Message)"
      'Cleanup also failed:'
      $cleanupErrors
      'No installed files or registration were removed.'
    ) -join "`n"
    throw [InvalidOperationException]::new($combinedMessage, $operationError.Exception)
  }
  throw $operationError
}
if ($cleanupErrors.Count -gt 0) {
  throw "NSIS certification trust cleanup failed:`n$($cleanupErrors -join "`n")"
}
if (Test-Path -LiteralPath $trustPaths.Root -PathType Leaf) {
  throw 'The temporary CurrentUser Root trust entry remained after NSIS certification.'
}
if (Test-Path -LiteralPath $trustPaths.TrustedPublisher -PathType Leaf) {
  throw 'The temporary CurrentUser TrustedPublisher entry remained after NSIS certification.'
}

Assert-EmendaInstalledInventory `
  -InstallDirectory $expectedInstallDirectory `
  -ExpectedFiles @($expectedInstalledExecutable, $expectedUninstaller)
Assert-EmendaShortcutTarget `
  -ShortcutPath $expectedStartMenuShortcut `
  -ExpectedTarget $expectedInstalledExecutable `
  -ShortcutLabel 'current-user Start-menu'
Assert-EmendaShortcutTarget `
  -ShortcutPath $expectedDesktopShortcut `
  -ExpectedTarget $expectedInstalledExecutable `
  -ShortcutLabel 'current-user Desktop'
foreach ($installedPath in @($expectedInstalledExecutable, $expectedUninstaller)) {
  [void] (Get-EmendaArtifactSignature `
    -Path $installedPath `
    -ExpectedThumbprint $normalizedThumbprint)
}
$postCleanupInstalledHash = (Get-FileHash -LiteralPath $expectedInstalledExecutable -Algorithm SHA256).Hash.ToUpperInvariant()
if ($postCleanupInstalledHash -cne $artifacts['release-exe'].Sha256) {
  throw 'The installed Emenda executable changed after temporary trust cleanup.'
}

Write-Host 'Signed NSIS current-user installation verified; MSI was not installed.'
Write-Host "Installed executable: $expectedInstalledExecutable"
Write-Host "Installed SHA-256: $postCleanupInstalledHash"
Write-Host 'Temporary CurrentUser Root and TrustedPublisher trust was removed.'
Write-Warning 'The verified installation remains installed. This certification script never uninstalls or replaces an existing installation.'
