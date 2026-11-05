# platform.rs: Per-user data directory

`data_dir() -> Option<PathBuf>` supplies the shared root for recent files, crop presets, New Document presets, adjustment presets, Image Size presets and user Color Lookup LUTs. It reads environment variables as OS strings and preserves Unicode paths, including Chinese user and directory names, without converting through a lossy string representation.

| Platform | Root |
| --- | --- |
| Windows | `%APPDATA%/OpenPhoto` |
| macOS | `$HOME/Library/Application Support/OpenPhoto` |
| Linux and other non-Windows, non-macOS targets | `$XDG_CONFIG_HOME/OpenPhoto`, falling back to `$HOME/.config/OpenPhoto` |

Empty environment values are treated as missing. Windows requires `APPDATA`; macOS requires `HOME`; Linux requires either a nonempty `XDG_CONFIG_HOME` or a nonempty `HOME`. Without the required base, the function returns `None`, retaining each caller's existing unavailable-store behavior. Windows does not fall back to `HOME`, and macOS ignores `APPDATA` and `XDG_CONFIG_HOME`.

Resolution only computes a path. It does not create folders, inspect existing files, migrate data, delete data or rename serialization keys, preset folders or files. macOS retains the existing root and existing saved data location. Each caller retains its own read/write handling and appends its existing file or folder name.

`resolve_data_dir(platform, appdata, home, xdg_config_home)` is a pure helper receiving environment values as parameters. Its unit tests run all platform policies on any host, covering Chinese Unicode paths, platform-specific precedence, Linux fallback and absent/empty values. Tests do not mutate the process environment or touch user data.
