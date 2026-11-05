//! What the document window's status bar shows beside the zoom box, picked
//! from the menu behind its arrow (Photoshop's labels and formats, read
//! from Photoshop 2026).

use crate::state::{AppState, DocState};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum StatusInfo {
    DocumentSizes,
    DocumentProfile,
    #[default]
    DocumentDimensions,
    GpuMode,
    CompositingMode,
    MeasurementScale,
    ScratchSizes,
    Efficiency,
    Timing,
    CurrentTool,
    Exposure32,
    SaveProgress,
    DownloadProgress,
    SmartObjects,
    LayerCount,
}

impl StatusInfo {
    /// In the menu's order.
    pub const ALL: [Self; 15] = [
        Self::DocumentSizes,
        Self::DocumentProfile,
        Self::DocumentDimensions,
        Self::GpuMode,
        Self::CompositingMode,
        Self::MeasurementScale,
        Self::ScratchSizes,
        Self::Efficiency,
        Self::Timing,
        Self::CurrentTool,
        Self::Exposure32,
        Self::SaveProgress,
        Self::DownloadProgress,
        Self::SmartObjects,
        Self::LayerCount,
    ];

    /// The menu item.
    pub fn label(self) -> &'static str {
        match self {
            Self::DocumentSizes => "文档大小",
            Self::DocumentProfile => "文档配置文件",
            Self::DocumentDimensions => "文档尺寸",
            Self::GpuMode => "GPU 模式",
            Self::CompositingMode => "合成模式",
            Self::MeasurementScale => "测量比例",
            Self::ScratchSizes => "暂存盘大小",
            Self::Efficiency => "效率",
            Self::Timing => "计时",
            Self::CurrentTool => "当前工具",
            Self::Exposure32 => "32 位曝光度",
            Self::SaveProgress => "保存进度",
            Self::DownloadProgress => "下载进度",
            Self::SmartObjects => "智能对象",
            Self::LayerCount => "图层计数",
        }
    }

    /// Save and Download Progress show a progress track instead of text
    /// (empty while nothing is saved or downloaded).
    pub fn is_progress(self) -> bool {
        matches!(self, Self::SaveProgress | Self::DownloadProgress)
    }

    /// The status bar's text for the document.
    pub fn text(self, app: &AppState, doc: &DocState) -> String {
        let d = &doc.doc;
        match self {
            Self::DocumentSizes => {
                let (flat, layered) = document_sizes(doc);
                format!("文档：{}/{}", size(flat), size(layered))
            }
            // Documents carry no color profile yet
            Self::DocumentProfile => "未标记的 RGB（8 位/通道）".to_owned(),
            Self::DocumentDimensions => format!(
                "{} px x {} px ({} ppi)",
                d.width,
                d.height,
                d.resolution.round()
            ),
            Self::GpuMode => "Metal".to_owned(),
            Self::CompositingMode => "经典".to_owned(),
            Self::MeasurementScale => "1 像素 = 1.0000 像素".to_owned(),
            Self::ScratchSizes => {
                let used: u64 = app.docs.values().map(|d| document_sizes(d).1).sum();
                format!("暂存盘：{}/{}", size(used), size(scratch_available()))
            }
            // Nothing is ever paged out to disk
            Self::Efficiency => "效率：100%".to_owned(),
            Self::Timing => format!("{:.1} 秒 (100%)", app.last_timing),
            Self::CurrentTool => {
                let name = app.tool.name();
                crate::state::display_name(name)
                    .trim_end_matches("工具")
                    .to_owned()
            }
            Self::Exposure32 => "曝光度仅适用于 32 位图像".to_owned(),
            Self::SaveProgress | Self::DownloadProgress => String::new(),
            Self::SmartObjects => "缺失：0 / 已更改：0".to_owned(),
            Self::LayerCount => {
                let groups = d.layers.iter().filter(|l| l.is_group()).count();
                let layers = d.layers.len() - groups;
                let mut s = if layers == 1 {
                    "1 个图层".to_owned()
                } else {
                    format!("{layers} 个图层")
                };
                match groups {
                    0 => {}
                    1 => s.push_str("，1 个组"),
                    n => s.push_str(&format!("，{n} 个组")),
                }
                s
            }
        }
    }
}

/// The flattened image's size (8-bit RGB) and an estimate of the layered
/// document's: the flattened size plus 4 bytes per pixel that each further
/// layer holds.
pub fn document_sizes(doc: &DocState) -> (u64, u64) {
    let d = &doc.doc;
    let flat = d.width as u64 * d.height as u64 * 3;
    let extra: u64 = d
        .layers
        .iter()
        .filter_map(|l| l.image())
        .skip(1)
        .filter_map(|i| i.content_bounds())
        .map(|(x0, y0, x1, y1)| ((x1 - x0) * (y1 - y0)).max(0) as u64 * 4)
        .sum();
    (flat, flat + extra)
}

/// Free space on the disk holding the temporary directory (Photoshop's
/// scratch disk).
fn scratch_available() -> u64 {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        let dir = std::env::temp_dir();
        let Ok(path) = std::ffi::CString::new(dir.as_os_str().as_bytes()) else {
            return 0;
        };
        let mut s = std::mem::MaybeUninit::<libc::statvfs>::uninit();
        // SAFETY: a valid C string and a buffer statvfs fills on success
        if unsafe { libc::statvfs(path.as_ptr(), s.as_mut_ptr()) } == 0 {
            let s = unsafe { s.assume_init() };
            // The field types differ between platforms
            #[allow(clippy::unnecessary_cast)]
            return s.f_bavail as u64 * s.f_frsize as u64;
        }
    }
    0
}

/// A byte count as Photoshop writes it: three significant digits with K, M
/// or G ("13.5K", "1.70M", "82.9G"), whole bytes below 1K.
pub fn size(bytes: u64) -> String {
    let units = [("G", 1u64 << 30), ("M", 1 << 20), ("K", 1 << 10)];
    for (unit, scale) in units {
        if bytes >= scale {
            let v = bytes as f64 / scale as f64;
            let decimals = if v >= 100.0 {
                0
            } else if v >= 10.0 {
                1
            } else {
                2
            };
            return format!("{v:.decimals$}{unit}");
        }
    }
    format!("{bytes} 字节")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes_read_like_photoshops() {
        // Photoshop's own readings: 64 × 72 RGB, 734 × 811 RGB
        assert_eq!(size(64 * 72 * 3), "13.5K");
        assert_eq!(size(734 * 811 * 3), "1.70M");
        assert_eq!(size(89_014_000_000), "82.9G");
        assert_eq!(size(300 << 10), "300K");
        assert_eq!(size(0), "0 字节");
    }
}
