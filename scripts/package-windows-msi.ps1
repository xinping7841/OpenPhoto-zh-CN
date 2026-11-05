#requires -Version 7.0
[CmdletBinding()]
param(
    [Parameter(Mandatory)][string]$SourceDirectory,
    [string]$OutputDirectory = 'dist'
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
if (-not $IsWindows) { throw 'MSI packaging requires Windows.' }
$root = Split-Path $PSScriptRoot -Parent
$source = (Resolve-Path -LiteralPath $SourceDirectory).Path
$output = [IO.Path]::GetFullPath($OutputDirectory)
$version = '0.2.2.1'
$filename = "OpenPhoto-$version-zh-CN-windows-x64.msi"
$msi = Join-Path $output $filename
$checksum = "$msi.sha256"
foreach ($path in @($msi, $checksum, [IO.Path]::ChangeExtension($msi, '.wixpdb'))) {
    if (Test-Path -LiteralPath $path) { throw "Refusing to overwrite output: $path" }
}
$required = @('openphoto.exe', 'op-mcp.exe', 'README-zh-CN.md', 'Licenses/LICENSE-MIT', 'Licenses/LICENSE-APACHE', 'Licenses/SourceSans3-OFL.txt', 'Licenses/NotoSansCJK-LICENSE.txt')
foreach ($relative in $required) {
    $file = Get-Item -LiteralPath (Join-Path $source $relative)
    if ($file.PSIsContainer -or $file.Length -eq 0) { throw "Missing or empty payload: $relative" }
}
$wixVersion = & wix --version
if ($LASTEXITCODE -ne 0 -or "$wixVersion" -notmatch '^4\.0\.6(?:\+.*)?$') {
    throw "WiX 4.0.6 is required; found: $wixVersion"
}
New-Item -ItemType Directory -Path $output -Force | Out-Null
& wix build (Join-Path $root 'packaging/windows/installer.wxs') -arch x64 -d "PackageSource=$source" -d "ProductVersion=$version" -o $msi
if ($LASTEXITCODE -ne 0) { throw "WiX build failed ($LASTEXITCODE)." }
if (-not (Test-Path -LiteralPath $msi -PathType Leaf) -or (Get-Item -LiteralPath $msi).Length -eq 0) {
    throw 'WiX did not produce a nonempty MSI.'
}
$hash = (Get-FileHash -LiteralPath $msi -Algorithm SHA256).Hash.ToLowerInvariant()
"$hash  $filename" | Set-Content -LiteralPath $checksum -Encoding utf8NoBOM
Write-Host "MSI: $msi"
Write-Host "SHA256: $hash"
