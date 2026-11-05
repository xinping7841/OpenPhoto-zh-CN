//! The per-user root for OpenPhoto's persistent data and presets.

use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(Clone, Copy)]
enum Platform {
    Windows,
    MacOs,
    Linux,
}

/// Resolve the current user's data folder without creating it.
pub(crate) fn data_dir() -> Option<PathBuf> {
    let platform = if cfg!(target_os = "windows") {
        Platform::Windows
    } else if cfg!(target_os = "macos") {
        Platform::MacOs
    } else {
        Platform::Linux
    };
    resolve_data_dir(
        platform,
        std::env::var_os("APPDATA").as_deref(),
        std::env::var_os("HOME").as_deref(),
        std::env::var_os("XDG_CONFIG_HOME").as_deref(),
    )
}

/// Pure path policy, with environment values supplied by the caller.
fn resolve_data_dir(
    platform: Platform,
    appdata: Option<&OsStr>,
    home: Option<&OsStr>,
    xdg_config_home: Option<&OsStr>,
) -> Option<PathBuf> {
    let nonempty = |value: Option<&OsStr>| value.filter(|s| !s.is_empty()).map(PathBuf::from);
    match platform {
        Platform::Windows => Some(nonempty(appdata)?.join("OpenPhoto")),
        Platform::MacOs => Some(
            nonempty(home)?
                .join("Library")
                .join("Application Support")
                .join("OpenPhoto"),
        ),
        Platform::Linux => Some(
            nonempty(xdg_config_home)
                .or_else(|| nonempty(home).map(|p| p.join(".config")))?
                .join("OpenPhoto"),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_uses_appdata_and_preserves_chinese_paths() {
        let appdata = OsStr::new(r"C:\Users\张小明\AppData\Roaming");
        assert_eq!(
            resolve_data_dir(
                Platform::Windows,
                Some(appdata),
                Some(OsStr::new("/ignored/home")),
                Some(OsStr::new("/ignored/config")),
            ),
            Some(PathBuf::from(appdata).join("OpenPhoto")),
        );
        for appdata in [None, Some(OsStr::new(""))] {
            assert_eq!(
                resolve_data_dir(Platform::Windows, appdata, Some(OsStr::new("/home")), None),
                None,
            );
        }
    }

    #[test]
    fn macos_keeps_existing_application_support_path() {
        assert_eq!(
            resolve_data_dir(
                Platform::MacOs,
                Some(OsStr::new("/ignored/appdata")),
                Some(OsStr::new("/Users/张小明")),
                Some(OsStr::new("/ignored/config")),
            ),
            Some(PathBuf::from(
                "/Users/张小明/Library/Application Support/OpenPhoto",
            )),
        );
        assert_eq!(resolve_data_dir(Platform::MacOs, None, None, None), None);
        assert_eq!(
            resolve_data_dir(Platform::MacOs, None, Some(OsStr::new("")), None),
            None,
        );
    }

    #[test]
    fn linux_prefers_xdg_config_home_even_without_home() {
        for home in [None, Some(OsStr::new("/home/张小明"))] {
            assert_eq!(
                resolve_data_dir(
                    Platform::Linux,
                    None,
                    home,
                    Some(OsStr::new("/配置/照片编辑")),
                ),
                Some(PathBuf::from("/配置/照片编辑/OpenPhoto")),
            );
        }
    }

    #[test]
    fn linux_falls_back_to_home_config_for_missing_or_empty_xdg() {
        for xdg in [None, Some(OsStr::new(""))] {
            assert_eq!(
                resolve_data_dir(Platform::Linux, None, Some(OsStr::new("/home/张小明")), xdg),
                Some(PathBuf::from("/home/张小明/.config/OpenPhoto")),
            );
        }
        assert_eq!(resolve_data_dir(Platform::Linux, None, None, None), None);
        assert_eq!(
            resolve_data_dir(Platform::Linux, None, Some(OsStr::new("")), None),
            None,
        );
    }
}
