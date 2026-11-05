# Windows portable usage document

`packaging/windows/README-zh-CN.md` is the Simplified Chinese usage document included at the root of the portable Windows archive. Its user-facing prose is Chinese; protocol identifiers, executable names and license names remain unchanged. This specification is English.

The document describes Windows 10/11 x64, full extraction before launch, double-click GUI startup, file opening and quoted command-line paths. It states that CLI arguments are paths and that no help flags are parsed. It explains wgpu's compatible graphics-device/driver requirement and the unsigned distribution's possible unknown-publisher prompt without instructing users to disable OS protection.

A valid JSON MCP client example points to the absolute extracted op-mcp.exe path. The text explains stdio JSON-RPC, the search_tools/call_tool sequence, independent process-local documents, explicit saving before exit and trusted-client access to local files. It does not imply that MCP controls the running GUI.

The document lists both executables, both project license texts and both font license texts, identifies the unofficial fork and no Adobe affiliation, and states the statically linked runtime and embedded fonts. It advises keeping original documents and validating complex PSD compatibility. It includes PowerShell SHA-256 verification and accurately limits CI claims to CPU tests, build, packaging and MCP handshake, excluding full GPU/UI coverage. There is no installer or paid-SDK requirement.
