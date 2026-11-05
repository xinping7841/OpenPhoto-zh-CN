#requires -Version 7.0
[CmdletBinding()]
param(
    [Parameter(Mandatory)][string]$MsiPath,
    [Parameter(Mandatory)][string]$SourceDirectory,
    [string]$LogDirectory = 'dist/msi-test-logs'
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
if (-not $IsWindows) { throw 'MSI smoke testing requires Windows.' }
$msi = (Resolve-Path -LiteralPath $MsiPath).Path
$source = (Resolve-Path -LiteralPath $SourceDirectory).Path
$logs = [IO.Path]::GetFullPath($LogDirectory)
$installDirectory = Join-Path ([Environment]::GetFolderPath('LocalApplicationData')) 'Programs/OpenPhoto-zh-CN'
$shortcutDirectory = Join-Path ([Environment]::GetFolderPath('Programs')) 'OpenPhoto-zh-CN'
$shortcutPath = Join-Path $shortcutDirectory 'OpenPhoto 简体中文版.lnk'
$required = @('openphoto.exe', 'op-mcp.exe', 'README-zh-CN.md', 'Licenses/LICENSE-MIT', 'Licenses/LICENSE-APACHE', 'Licenses/SourceSans3-OFL.txt', 'Licenses/NotoSansCJK-LICENSE.txt')

function Get-OpenPhotoRegistration {
    foreach ($registryRoot in @('HKCU:/Software/Microsoft/Windows/CurrentVersion/Uninstall', 'HKCU:/Software/WOW6432Node/Microsoft/Windows/CurrentVersion/Uninstall')) {
        if (Test-Path -LiteralPath $registryRoot) {
            Get-ChildItem -LiteralPath $registryRoot | ForEach-Object {
                $properties = Get-ItemProperty -LiteralPath $_.PSPath
                if ($properties.PSObject.Properties['DisplayName'] -and $properties.DisplayName -eq 'OpenPhoto 简体中文版') { $properties }
            }
        }
    }
}

function Invoke-Msi([string]$Operation, [string]$LogPath) {
    $arguments = "$Operation `"$msi`" /qn /norestart /L*v `"$LogPath`""
    $installer = Start-Process -FilePath (Join-Path $env:SystemRoot 'System32/msiexec.exe') -ArgumentList $arguments -PassThru
    if (-not $installer.WaitForExit(300000)) { throw "msiexec timed out; inspect $LogPath" }
    if ($installer.ExitCode -notin @(0, 3010)) { throw "msiexec failed ($($installer.ExitCode)); inspect $LogPath" }
}

if ((Test-Path -LiteralPath $installDirectory) -or (Test-Path -LiteralPath $shortcutDirectory) -or @(Get-OpenPhotoRegistration).Count -ne 0) {
    throw 'Refusing to modify a pre-existing OpenPhoto installation or Start Menu folder.'
}
foreach ($relative in $required) {
    if (-not (Test-Path -LiteralPath (Join-Path $source $relative) -PathType Leaf)) { throw "Missing source payload: $relative" }
}
New-Item -ItemType Directory -Path $logs -Force | Out-Null
$runId = [guid]::NewGuid().ToString()
$installLog = Join-Path $logs "$runId-install.log"
$uninstallLog = Join-Path $logs "$runId-uninstall.log"
$transcriptLog = Join-Path $logs "$runId-smoke.log"
Start-Transcript -LiteralPath $transcriptLog | Out-Null
$installedSuccessfully = $false
try {
    Invoke-Msi '/i' $installLog
    $installedSuccessfully = $true
    foreach ($relative in $required) {
        $installed = Join-Path $installDirectory $relative
        if (-not (Test-Path -LiteralPath $installed -PathType Leaf)) { throw "Installed file missing: $relative" }
        $expected = (Get-FileHash -LiteralPath (Join-Path $source $relative) -Algorithm SHA256).Hash
        $actual = (Get-FileHash -LiteralPath $installed -Algorithm SHA256).Hash
        if ($actual -ne $expected) { throw "Installed bytes differ from ZIP payload: $relative" }
    }
    if (-not (Test-Path -LiteralPath $shortcutPath -PathType Leaf)) { throw 'Start Menu shortcut missing.' }
    Copy-Item -LiteralPath $shortcutPath -Destination (Join-Path $logs "$runId-installed-shortcut.lnk")
    if (-not ('OpenPhotoInstallerSmoke.ShortcutReader' -as [type])) {
        Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
using System.Runtime.InteropServices.ComTypes;
using System.Text;
namespace OpenPhotoInstallerSmoke {
    [ComImport, Guid("000214F9-0000-0000-C000-000000000046"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
    internal interface IShellLinkW {
        void GetPath([Out, MarshalAs(UnmanagedType.LPWStr)] StringBuilder path, int capacity, IntPtr findData, uint flags);
        void GetIDList(out IntPtr identifierList);
        void SetIDList(IntPtr identifierList);
        void GetDescription([Out, MarshalAs(UnmanagedType.LPWStr)] StringBuilder description, int capacity);
        void SetDescription([MarshalAs(UnmanagedType.LPWStr)] string description);
        void GetWorkingDirectory([Out, MarshalAs(UnmanagedType.LPWStr)] StringBuilder directory, int capacity);
    }
    public static class ShortcutReader {
        public static string[] Read(string filename) {
            object instance = Activator.CreateInstance(Type.GetTypeFromCLSID(new Guid("00021401-0000-0000-C000-000000000046"), true));
            try {
                ((IPersistFile)instance).Load(filename, 0);
                var link = (IShellLinkW)instance;
                var target = new StringBuilder(32768);
                var working = new StringBuilder(32768);
                link.GetPath(target, target.Capacity, IntPtr.Zero, 4);
                link.GetWorkingDirectory(working, working.Capacity);
                return new[] { target.ToString(), working.ToString() };
            }
            finally { Marshal.FinalReleaseComObject(instance); }
        }
    }
}
'@
    }
    $shortcut = [OpenPhotoInstallerSmoke.ShortcutReader]::Read([IO.Path]::GetFullPath($shortcutPath))
    Write-Host "Native Unicode shortcut target=[$($shortcut[0])] working=[$($shortcut[1])] link=[$shortcutPath]"
    if ([string]::IsNullOrWhiteSpace($shortcut[0]) -or [string]::IsNullOrWhiteSpace($shortcut[1])) {
        throw 'Installed shortcut has an empty native target or working directory; preserved the original link in test logs.'
    }
    $target = [IO.Path]::GetFullPath($shortcut[0])
    $expectedTarget = [IO.Path]::GetFullPath((Join-Path $installDirectory 'openphoto.exe'))
    $working = [IO.Path]::TrimEndingDirectorySeparator([IO.Path]::GetFullPath($shortcut[1]))
    $expectedWorking = [IO.Path]::TrimEndingDirectorySeparator([IO.Path]::GetFullPath($installDirectory))
    if (-not [StringComparer]::OrdinalIgnoreCase.Equals($target, $expectedTarget) -or -not [StringComparer]::OrdinalIgnoreCase.Equals($working, $expectedWorking)) {
        throw "Start Menu shortcut mismatch: target=$target expected=$expectedTarget; working=$working expected=$expectedWorking"
    }
    $registration = @(Get-OpenPhotoRegistration)
    if ($registration.Count -ne 1 -or $registration[0].DisplayVersion -ne '0.2.2.1' -or [string]::IsNullOrWhiteSpace($registration[0].UninstallString)) {
        throw 'Per-user Apps uninstall registration missing or incorrect.'
    }
    & (Join-Path $PSScriptRoot 'test-windows-mcp.ps1') -Executable (Join-Path $installDirectory 'op-mcp.exe')
    Write-Host 'Installed payload hashes, shortcut, Apps registration and MCP smoke passed.'
}
finally {
    try {
        if ($installedSuccessfully) {
            Invoke-Msi '/x' $uninstallLog
            foreach ($relative in $required) {
                if (Test-Path -LiteralPath (Join-Path $installDirectory $relative)) { throw "Uninstall left a packaged file: $relative" }
            }
            if ((Test-Path -LiteralPath $shortcutPath) -or (Test-Path -LiteralPath $shortcutDirectory)) { throw 'Uninstall left the Start Menu shortcut or folder.' }
            if (@(Get-OpenPhotoRegistration).Count -ne 0) { throw 'Uninstall left the Apps registration.' }
            Write-Host 'MSI uninstall removed all packaged files, shortcut and Apps registration.'
        }
    }
    finally {
        Stop-Transcript | Out-Null
        Write-Host "Preserved MSI test logs: $logs"
    }
}
