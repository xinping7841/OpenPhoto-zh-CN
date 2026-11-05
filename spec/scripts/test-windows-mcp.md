# Packaged MCP smoke test

`scripts/test-windows-mcp.ps1` requires PowerShell 7, a mandatory `-Executable` path, and an optional `-TimeoutSeconds` from 1 to 120 (default 20). It starts the resolved executable directly with redirected stdin, stdout, and stderr and no shell. Stderr is drained concurrently to avoid pipe deadlock.

The script sends one newline-delimited JSON-RPC initialize request with ID 1 and protocol version `2024-11-05`. It requires a JSON-RPC 2.0 result with the matching ID, no protocol error, the requested protocol version, server name `openphoto-mcp`, and tools capability. It sends `notifications/initialized`, then a tools/list request with ID 2. The reply must expose exactly `search_tools` and `call_tool`, each with an object input schema and a nonempty description. Notifications must produce no extra reply.

Each response has a bounded read timeout. Closing stdin must cause a zero exit within the timeout. Extra stdout, malformed JSON, missing fields, wrong IDs, RPC errors, missing tools, nonzero exit, and stalled processes fail the script. A started process still running during cleanup is killed with its process tree and disposed. No document editing, user images, network service, or UI/GPU initialization is needed. This is a real executable protocol test, separate from in-process Rust tests.
