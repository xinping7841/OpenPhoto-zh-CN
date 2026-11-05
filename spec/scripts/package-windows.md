# Windows portable package

`scripts/package-windows.ps1` requires PowerShell 7 on Windows and the MSVC build tools. It verifies that the repository toolchain channel is exactly `1.99.0`, reads the version from the root manifest's workspace package section, and runs `cargo +1.99.0 build --locked --release --target x86_64-pc-windows-msvc -p openphoto -p op-mcp`. Build failures stop packaging. The build temporarily appends `-C target-feature=+crt-static` to RUSTFLAGS and sets CARGO_TARGET_DIR to the checkout's target directory, restoring both afterward. The MSVC C runtime is linked statically; operating-system DLLs remain system dependencies. No installer, signing service, paid SDK, or secrets are required.

The optional `-Dist` parameter selects the destination directory, relative to the repository root by default. The default is `dist`. The script refuses to overwrite an existing package directory, ZIP, or checksum file. It does not delete existing outputs, including partial outputs from failed attempts.

The output is `OpenPhoto-<version>-zh-CN-windows-x64.zip` with one top-level directory of that name containing:

- `openphoto.exe` and `op-mcp.exe` from the explicit Windows release target.
- `README-zh-CN.md` from `packaging/windows`.
- `Licenses/LICENSE-MIT` and `Licenses/LICENSE-APACHE` from the repository root.
- `Licenses/SourceSans3-OFL.txt` from the Source Sans 3 OFL asset and `Licenses/NotoSansCJK-LICENSE.txt` from the Noto Sans CJK font asset.

All source files must exist before staging. After compression, the script opens the ZIP and checks each expected entry exists and has nonzero length. It emits `<archive>.sha256` as a lowercase SHA-256 plus the archive basename, encoded as UTF-8 without BOM, and prints the archive path and hash. The archive is unsigned. The script builds the actual application and MCP executables but does not launch the graphical editor or claim GPU test coverage. CI separately extracts and smoke-tests the MCP binary from the completed archive.
