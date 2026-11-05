#requires -Version 7.0
[CmdletBinding()]
param(
    [string]$Dist = 'dist'
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
if (-not $IsWindows) { throw 'Windows packaging requires Windows and the MSVC build tools.' }

$Root = Split-Path $PSScriptRoot -Parent
$Target = 'x86_64-pc-windows-msvc'
Push-Location $Root
try {
    $toolchain = Get-Content -LiteralPath 'rust-toolchain.toml' -Raw
    if ($toolchain -notmatch '(?m)^channel\s*=\s*"1\.99\.0"\s*$') {
        throw 'Expected the repository-pinned Rust toolchain 1.99.0.'
    }
    $manifest = Get-Content -LiteralPath 'Cargo.toml' -Raw
    $workspace = [regex]::Match($manifest, '(?ms)^\[workspace\.package\]\s*\r?\n(?<body>.*?)(?=^\[|\z)')
    $versionMatch = [regex]::Match($workspace.Groups['body'].Value, '(?m)^version\s*=\s*"(?<version>[0-9]+\.[0-9]+\.[0-9]+(?:[-+][A-Za-z0-9.-]+)?)"\s*$')
    if (-not $versionMatch.Success) { throw 'Cannot read workspace package version.' }
    $packageName = "OpenPhoto-$($versionMatch.Groups['version'].Value)-zh-CN-windows-x64"
    $distPath = [IO.Path]::GetFullPath($Dist, $Root)
    $packagePath = Join-Path $distPath $packageName
    $archivePath = Join-Path $distPath "$packageName.zip"
    $checksumPath = "$archivePath.sha256"
    foreach ($path in @($packagePath, $archivePath, $checksumPath)) {
        if (Test-Path -LiteralPath $path) { throw "Refusing to overwrite existing output: $path" }
    }

    # Statically link the MSVC runtime so a clean PC does not need a VC++ installer.
    $oldRustflags = $env:RUSTFLAGS
    $oldTargetDir = $env:CARGO_TARGET_DIR
    try {
        $env:RUSTFLAGS = "$oldRustflags -C target-feature=+crt-static".Trim()
        $env:CARGO_TARGET_DIR = Join-Path $Root 'target'
        & cargo +1.99.0 build --locked --release --target $Target -p openphoto -p op-mcp
        if ($LASTEXITCODE -ne 0) { throw "Windows release build failed ($LASTEXITCODE)." }
    }
    finally {
        $env:RUSTFLAGS = $oldRustflags
        $env:CARGO_TARGET_DIR = $oldTargetDir
    }

    $inputs = @{
        'openphoto.exe' = "target/$Target/release/openphoto.exe"
        'op-mcp.exe' = "target/$Target/release/op-mcp.exe"
        'README-zh-CN.md' = 'packaging/windows/README-zh-CN.md'
        'Licenses/LICENSE-MIT' = 'LICENSE-MIT'
        'Licenses/LICENSE-APACHE' = 'LICENSE-APACHE'
        'Licenses/SourceSans3-OFL.txt' = 'crates/op-ui/assets/fonts/OFL.txt'
        'Licenses/NotoSansCJK-LICENSE.txt' = 'crates/op-ui/assets/fonts/NotoSansCJK-LICENSE.txt'
    }
    foreach ($source in $inputs.Values) {
        if (-not (Test-Path -LiteralPath $source -PathType Leaf)) { throw "Missing package input: $source" }
    }
    New-Item -ItemType Directory -Path (Join-Path $packagePath 'Licenses') -Force | Out-Null
    foreach ($entry in $inputs.GetEnumerator()) {
        Copy-Item -LiteralPath $entry.Value -Destination (Join-Path $packagePath $entry.Key)
    }
    Compress-Archive -LiteralPath $packagePath -DestinationPath $archivePath -CompressionLevel Optimal

    # Verify the archive layout and required license texts, not just the staging directory.
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $zip = [IO.Compression.ZipFile]::OpenRead($archivePath)
    try {
        foreach ($relative in $inputs.Keys) {
            $name = "$packageName/$relative"
            $entry = $zip.Entries | Where-Object { $_.FullName.Replace('\', '/') -eq $name }
            if ($null -eq $entry -or $entry.Length -eq 0) { throw "Archive missing or empty: $name" }
        }
    }
    finally { $zip.Dispose() }
    $hash = (Get-FileHash -LiteralPath $archivePath -Algorithm SHA256).Hash.ToLowerInvariant()
    "$hash  $packageName.zip" | Set-Content -LiteralPath $checksumPath -Encoding utf8NoBOM
    Write-Host "Portable ZIP: $archivePath"
    Write-Host "SHA256: $hash"
}
finally { Pop-Location }
