# Windows x64 portable CI

`.github/workflows/windows.yml` runs on `windows-latest` for manual workflow dispatch, pushes to branch `windows-zh-CN`, and pushes of tags matching `windows*`. It uses only read access to repository contents, disables persisted checkout credentials, and requires no secrets. Checkout and artifact upload actions are pinned to full commit SHAs. New runs for the same ref cancel older runs; the job has a 60-minute timeout.

The job validates the repository pin and installs Rust 1.99.0, rustfmt, clippy and the x86_64-pc-windows-msvc target. Every Cargo command explicitly selects 1.99.0. Build, lint and test commands that resolve dependencies use `--locked`. RUSTFLAGS requests the statically linked MSVC runtime.

Required checks execute real commands and fail the job on errors:

- `cargo fmt --all -- --check`.
- Clippy across all workspace targets for Windows with warnings denied.
- CPU tests for op-core, op-color, op-io, op-tools and op-mcp.
- `op-ui` library tests selected by `offline_tests`, `theme::tests`, `menu::` (including shared menu tests) and `platform::tests` in explicit CPU-only steps. The multi-command menu/platform step checks the first exit code before starting the second command.
- The Windows packaging script's release build, required-file and ZIP-entry checks.
- Extraction of the unique completed ZIP into a fresh runner temporary directory and initialize/tools/list smoke against the extracted op-mcp executable.

The hosted job does not run GPU-dependent UI/screenshot tests, interactive application launch, or WARP rendering. Its step summary explicitly states that boundary. Compiling all targets for Clippy does not mean all UI tests have executed.

After successful checks, the job uploads the portable ZIP and SHA-256 sidecar under artifact name `OpenPhoto-zh-CN-windows-x64` with 14-day retention and fails when artifact files are missing. It does not create GitHub releases or modify repository contents. The lead can download the validated artifact for release publication.
