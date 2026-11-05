#requires -Version 7.0
[CmdletBinding()]
param(
    [Parameter(Mandatory)][string]$Executable,
    [ValidateRange(1, 120)][int]$TimeoutSeconds = 20
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$executablePath = (Resolve-Path -LiteralPath $Executable).Path
$startInfo = [Diagnostics.ProcessStartInfo]::new()
$startInfo.FileName = $executablePath
$startInfo.UseShellExecute = $false
$startInfo.RedirectStandardInput = $true
$startInfo.RedirectStandardOutput = $true
$startInfo.RedirectStandardError = $true
$startInfo.CreateNoWindow = $true
$process = [Diagnostics.Process]::new()
$process.StartInfo = $startInfo

function Read-RpcResponse([int]$Id) {
    $read = $process.StandardOutput.ReadLineAsync()
    if (-not $read.Wait($TimeoutSeconds * 1000)) { throw "MCP response $Id timed out." }
    if ($null -eq $read.Result) { throw "MCP closed stdout before response $Id." }
    $message = ConvertFrom-Json -InputObject $read.Result -AsHashtable
    if ($message.jsonrpc -ne '2.0' -or $message.id -ne $Id -or $message.ContainsKey('error') -or -not $message.ContainsKey('result')) {
        throw "Invalid MCP response ${Id}: $($read.Result)"
    }
    return $message.result
}

$started = $false
try {
    $started = $process.Start()
    if (-not $started) { throw 'Cannot start MCP executable.' }
    # Drain stderr concurrently so diagnostic output cannot block the child process.
    $stderr = $process.StandardError.ReadToEndAsync()
    $process.StandardInput.WriteLine('{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"windows-package-smoke","version":"1"}}}')
    $process.StandardInput.Flush()
    $initialize = Read-RpcResponse 1
    if ($initialize.protocolVersion -ne '2024-11-05' -or $initialize.serverInfo.name -ne 'openphoto-mcp' -or -not $initialize.capabilities.ContainsKey('tools')) {
        throw 'MCP initialize returned an unexpected server contract.'
    }
    $process.StandardInput.WriteLine('{"jsonrpc":"2.0","method":"notifications/initialized"}')
    $process.StandardInput.WriteLine('{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}')
    $process.StandardInput.Flush()
    $list = Read-RpcResponse 2
    $names = @($list.tools | ForEach-Object { $_.name })
    if ($names.Count -ne 2 -or $names -notcontains 'search_tools' -or $names -notcontains 'call_tool') {
        throw "MCP tools/list returned unexpected tools: $($names -join ', ')"
    }
    foreach ($tool in $list.tools) {
        if ($tool.inputSchema.type -ne 'object' -or [string]::IsNullOrWhiteSpace($tool.description)) {
            throw "MCP tool definition is incomplete: $($tool.name)"
        }
    }
    $process.StandardInput.Close()
    if (-not $process.WaitForExit($TimeoutSeconds * 1000)) { throw 'MCP did not exit after stdin closed.' }
    if ($process.ExitCode -ne 0) { throw "MCP exited with code $($process.ExitCode): $($stderr.Result)" }
    if ($null -ne $process.StandardOutput.ReadLine()) { throw 'Unexpected extra stdout after MCP responses.' }
    Write-Host "MCP initialize + tools/list passed: $executablePath"
}
finally {
    if ($started -and -not $process.HasExited) {
        $process.Kill($true)
        $process.WaitForExit()
    }
    $process.Dispose()
}
