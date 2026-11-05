//! Shared Chinese menu definitions and platform renderers.
use crate::commands::Command;
use crate::rulers::RulerUnit;
use crate::state::AppState;
use crate::status_info::StatusInfo;
use op_core::align::{Align, Distribute};

#[cfg(any(target_os = "macos", test))]
const ALL_COMMANDS: &[Command] = &[
    Command::New,
    Command::Open,
    Command::Close,
    Command::CloseAll,
    Command::CloseOthers,
    Command::Save,
    Command::SaveAs,
    Command::SaveACopy,
    Command::Revert,
    Command::Quit,
    Command::ExportAs,
    Command::QuickExportPng,
    Command::LayerQuickExportPng,
    Command::LayerExportAs,
    Command::Undo,
    Command::Redo,
    Command::ToggleLastState,
    Command::CanvasSize,
    Command::ImageSize,
    Command::Rotate180,
    Command::RotateArbitrary,
    Command::Rotate90Clockwise,
    Command::Rotate90CounterClockwise,
    Command::FlipCanvasHorizontal,
    Command::FlipCanvasVertical,
    Command::Crop,
    Command::Trim,
    Command::RevealAll,
    Command::Invert,
    Command::Desaturate,
    Command::Equalize,
    Command::Threshold,
    Command::Posterize,
    Command::Levels,
    Command::HueSaturation,
    Command::Exposure,
    Command::BrightnessContrast,
    Command::Curves,
    Command::ColorBalance,
    Command::BlackWhite,
    Command::Vibrance,
    Command::ChannelMixer,
    Command::SelectiveColor,
    Command::PhotoFilter,
    Command::GradientMap,
    Command::AutoTone,
    Command::AutoContrast,
    Command::AutoColor,
    Command::LastFilter,
    Command::Average,
    Command::Solarize,
    Command::Clouds,
    Command::Facet,
    Command::DifferenceClouds,
    Command::GaussianBlur,
    Command::BoxBlur,
    Command::UnsharpMask,
    Command::AddNoise,
    Command::Median,
    Command::Minimum,
    Command::Maximum,
    Command::HighPass,
    Command::Offset,
    Command::Mosaic,
    Command::MotionBlur,
    Command::Emboss,
    Command::Twirl,
    Command::Crystallize,
    Command::Pointillize,
    Command::Diffuse,
    Command::Ripple,
    Command::Mezzotint,
    Command::Tiles,
    Command::ZigZag,
    Command::HsbHsl,
    Command::ColorHalftone,
    Command::Pinch,
    Command::Spherize,
    Command::PolarCoordinates,
    Command::SurfaceBlur,
    Command::DustAndScratches,
    Command::Fragment,
    Command::CustomFilter,
    Command::Despeckle,
    Command::SharpenEdges,
    Command::TraceContour,
    Command::Wind,
    Command::RadialBlur,
    Command::SmartBlur,
    Command::ShapeBlur,
    Command::LensBlur,
    Command::ReduceNoise,
    Command::SmartSharpen,
    Command::Fibers,
    Command::LensFlare,
    Command::Extrude,
    Command::OilPaint,
    Command::Wave,
    Command::Shear,
    Command::Displace,
    Command::ShadowsHighlights,
    Command::HdrToning,
    Command::ReplaceColor,
    Command::MatchColor,
    Command::ColorLookup,
    Command::Blur,
    Command::BlurMore,
    Command::Sharpen,
    Command::SharpenMore,
    Command::FindEdges,
    Command::NewLayer,
    Command::DeleteLayer,
    Command::ToggleLayerVisibility,
    Command::DuplicateLayer,
    Command::LayerViaCopy,
    Command::LayerViaCut,
    Command::LayerFromBackground,
    Command::DeleteHiddenLayers,
    Command::BringToFront,
    Command::BringForward,
    Command::SendBackward,
    Command::SendToBack,
    Command::MergeDown,
    Command::MergeVisible,
    Command::FlattenImage,
    Command::Fill,
    Command::Clear,
    Command::FreeTransform,
    Command::TransformSelection,
    TRANSFORM_MODES[0],
    TRANSFORM_MODES[1],
    TRANSFORM_MODES[2],
    TRANSFORM_MODES[3],
    TRANSFORM_MODES[4],
    WARP,
    Command::TransformAgain,
    Command::TransformRotate180,
    Command::TransformRotate90Clockwise,
    Command::TransformRotate90CounterClockwise,
    Command::TransformFlipHorizontal,
    Command::TransformFlipVertical,
    Command::Cut,
    Command::Copy,
    Command::CopyMerged,
    Command::Paste,
    Command::PasteInPlace,
    Command::PasteInto,
    Command::PasteOutside,
    Command::MaskRevealAll,
    Command::MaskHideAll,
    Command::MaskRevealSelection,
    Command::MaskHideSelection,
    Command::MaskDelete,
    Command::MaskApply,
    Command::MaskToggle,
    Command::SelectAll,
    Command::DeselectLayers,
    Command::LinkLayers,
    Command::SelectLinkedLayers,
    Command::SelectAllLayers,
    Command::RenameLayer,
    Command::LockLayers,
    Command::Deselect,
    Command::Reselect,
    Command::SelectInverse,
    Command::ModifyBorder,
    Command::ModifySmooth,
    Command::ModifyExpand,
    Command::ModifyContract,
    Command::ModifyFeather,
    Command::Grow,
    Command::Similar,
    Command::QuickMask,
    Command::ZoomIn,
    Command::ZoomOut,
    Command::FitOnScreen,
    Command::FitLayers,
    Command::ActualPixels,
    Command::Zoom200,
    Command::PrintSize,
    Command::ActualSize,
    Command::Fade,
    Command::DefinePattern,
    Command::ClearRecent,
    Command::OpenRecent(0),
    Command::OpenRecent(1),
    Command::OpenRecent(2),
    Command::OpenRecent(3),
    Command::OpenRecent(4),
    Command::OpenRecent(5),
    Command::OpenRecent(6),
    Command::OpenRecent(7),
    Command::OpenRecent(8),
    Command::OpenRecent(9),
    Command::OpenRecent(10),
    Command::OpenRecent(11),
    Command::OpenRecent(12),
    Command::OpenRecent(13),
    Command::OpenRecent(14),
    Command::OpenRecent(15),
    Command::OpenRecent(16),
    Command::OpenRecent(17),
    Command::OpenRecent(18),
    Command::OpenRecent(19),
    Command::ToggleRulers,
    Command::ToggleExtras,
    Command::ToggleGuides,
    Command::ToggleGrid,
    Command::FlipView,
    Command::TogglePixelGrid,
    Command::ToggleSmartGuides,
    Command::ToggleSelectionEdges,
    Command::ToggleLayerEdges,
    Command::ShowAllExtras,
    Command::ShowNoExtras,
    Command::ToggleSnap,
    Command::RulerUnits(RulerUnit::Pixels),
    Command::RulerUnits(RulerUnit::Inches),
    Command::RulerUnits(RulerUnit::Centimeters),
    Command::RulerUnits(RulerUnit::Millimeters),
    Command::RulerUnits(RulerUnit::Points),
    Command::RulerUnits(RulerUnit::Picas),
    Command::RulerUnits(RulerUnit::Percent),
    Command::StatusInfo(StatusInfo::DocumentSizes),
    Command::StatusInfo(StatusInfo::DocumentProfile),
    Command::StatusInfo(StatusInfo::DocumentDimensions),
    Command::StatusInfo(StatusInfo::GpuMode),
    Command::StatusInfo(StatusInfo::CompositingMode),
    Command::StatusInfo(StatusInfo::MeasurementScale),
    Command::StatusInfo(StatusInfo::ScratchSizes),
    Command::StatusInfo(StatusInfo::Efficiency),
    Command::StatusInfo(StatusInfo::Timing),
    Command::StatusInfo(StatusInfo::CurrentTool),
    Command::StatusInfo(StatusInfo::Exposure32),
    Command::StatusInfo(StatusInfo::SaveProgress),
    Command::StatusInfo(StatusInfo::DownloadProgress),
    Command::StatusInfo(StatusInfo::SmartObjects),
    Command::StatusInfo(StatusInfo::LayerCount),
    Command::ScreenMode(crate::state::ScreenMode::Standard),
    Command::ScreenMode(crate::state::ScreenMode::FullWithMenus),
    Command::ScreenMode(crate::state::ScreenMode::Full),
    Command::SnapToGuides,
    Command::SnapToGrid,
    Command::SnapToLayers,
    Command::SnapToSlices,
    Command::SnapToBounds,
    Command::SnapToAll,
    Command::SnapToNone,
    Command::LockGuides,
    Command::ClearGuides,
    Command::EditSelectedGuides,
    Command::ClearSelectedGuides,
    Command::ClearCanvasGuides,
    Command::ToggleCanvasGuides,
    Command::NewGuide,
    Command::NewGuideLayout,
    Command::HideApp,
    Command::ToggleHistory,
    Command::ToggleInfo,
    Command::ToggleNavigator,
    Command::ToggleHistogram,
    Command::ToggleCloneSource,
    Command::ToggleBrushes,
    Command::ToggleBrushSettings,
    Command::ToggleToolPresets,
    Command::ToggleLayerComps,
    Command::WorkspaceEssentials,
    Command::ResetWorkspace,
    Command::Align(Align::Top),
    Command::Align(Align::VerticalCenter),
    Command::Align(Align::Bottom),
    Command::Align(Align::Left),
    Command::Align(Align::HorizontalCenter),
    Command::Align(Align::Right),
    Command::Distribute(Distribute::Top),
    Command::Distribute(Distribute::VerticalCenter),
    Command::Distribute(Distribute::Bottom),
    Command::Distribute(Distribute::Left),
    Command::Distribute(Distribute::HorizontalCenter),
    Command::Distribute(Distribute::Right),
    Command::Distribute(Distribute::Horizontally),
    Command::Distribute(Distribute::Vertically),
    Command::GroupLayers,
    Command::UngroupLayers,
    Command::NewGroup,
    Command::NewGroupFromLayers,
    Command::ArrangeReverse,
];

/// Edit > Transform > Warp.
pub const WARP: Command = Command::TransformIn(crate::state::TransformMode::Warp, "Warp");

/// Edit > Transform's modes, as commands.
pub const TRANSFORM_MODES: [Command; 5] = {
    use crate::state::TransformMode::*;
    [
        Command::TransformIn(Free, "Scale"),
        Command::TransformIn(Free, "Rotate"),
        Command::TransformIn(Skew, "Skew"),
        Command::TransformIn(Distort, "Distort"),
        Command::TransformIn(Perspective, "Perspective"),
    ]
};

#[derive(Clone, Debug)]
enum MenuNode {
    Item {
        label: String,
        command: Option<Command>,
        shortcut: Option<String>,
        check: bool,
    },
    Submenu {
        label: String,
        enabled: bool,
        children: Vec<MenuNode>,
    },
    Separator,
    Recent,
    System(SystemItem),
}

#[derive(Clone, Copy, Debug)]
enum SystemItem {
    About,
    Services,
    HideOthers,
    ShowAll,
}

impl MenuNode {
    fn new(label: &str, enabled: bool) -> Self {
        Self::Submenu {
            label: label.into(),
            enabled,
            children: Vec::new(),
        }
    }

    fn with_items(
        label: &str,
        enabled: bool,
        children: &[&Self],
    ) -> Result<Self, std::convert::Infallible> {
        Ok(Self::Submenu {
            label: label.into(),
            enabled,
            children: children.iter().map(|node| (*node).clone()).collect(),
        })
    }
}

fn menu_tree() -> Vec<MenuNode> {
    use MenuNode as Submenu;
    let item = |label: &str, command: Command| MenuNode::Item {
        label: label.into(),
        command: Some(command),
        shortcut: command.shortcut().map(|s| s.accelerator()),
        check: false,
    };
    let check_item = |label: &str, command: Command| MenuNode::Item {
        label: label.into(),
        command: Some(command),
        shortcut: command.shortcut().map(|s| s.accelerator()),
        check: true,
    };
    let plain_item = |label: &str, command: Command| MenuNode::Item {
        label: label.into(),
        command: Some(command),
        shortcut: None,
        check: false,
    };
    let todo = |label: &str, shortcut: Option<&str>| MenuNode::Item {
        label: label.into(),
        command: None,
        shortcut: shortcut.map(str::to_owned),
        check: false,
    };
    let todo_sub = |label: &str| Submenu::new(label, false);
    let sep = || MenuNode::Separator;
    let app_menu = Submenu::with_items(
        "OpenPhoto",
        true,
        &[
            &MenuNode::System(SystemItem::About),
            &sep(),
            &MenuNode::System(SystemItem::Services),
            &sep(),
            &item("隐藏 OpenPhoto", Command::HideApp),
            &MenuNode::System(SystemItem::HideOthers),
            &MenuNode::System(SystemItem::ShowAll),
            &sep(),
            &item("退出 OpenPhoto", Command::Quit),
        ],
    );

    let open_recent = MenuNode::Recent;
    let file = Submenu::with_items(
        "文件",
        true,
        &[
            &item("新建...", Command::New),
            &item("打开...", Command::Open),
            &todo("在 Bridge 中浏览...", Some("CmdOrCtrl+Alt+O")),
            &todo("打开为智能对象...", None),
            &open_recent,
            &sep(),
            &item("关闭", Command::Close),
            &item("关闭全部", Command::CloseAll),
            &item("关闭其他", Command::CloseOthers),
            &todo("关闭并转到 Bridge...", Some("CmdOrCtrl+Shift+W")),
            &item("保存", Command::Save),
            &item("另存为...", Command::SaveAs),
            &item("保存副本...", Command::SaveACopy),
            &item("恢复", Command::Revert),
            &sep(),
            &todo("邀请编辑...", None),
            &todo("共享以供审阅", None),
            &Submenu::with_items(
                "导出",
                true,
                &[
                    &item("快速导出为 PNG", Command::QuickExportPng),
                    &item("导出为...", Command::ExportAs),
                    &todo("导出首选项...", None),
                    &sep(),
                    &todo(
                        "存储为 Web 所用格式（旧版）...",
                        Some("CmdOrCtrl+Shift+Alt+S"),
                    ),
                    &sep(),
                    &todo("画板导出为文件...", None),
                    &todo("画板导出为 PDF...", None),
                    &todo("图层复合导出为文件...", None),
                    &todo("图层复合导出为 PDF...", None),
                    &todo("图层导出为文件...", None),
                    &sep(),
                    &todo("颜色查找表...", None),
                    &todo("数据组导出为文件...", None),
                    &todo("路径导出到 Illustrator...", None),
                    &todo("渲染视频...", None),
                    &todo("缩放切片...", None),
                ],
            )
            .expect("static menu definition is valid"),
            &sep(),
            // Photoshop's Adobe Stock and Adobe Express items are left
            // out: the app shows no Adobe brand or service
            &todo("置入嵌入对象...", None),
            &todo("置入链接对象...", None),
            &todo("打包...", None),
            &sep(),
            &todo_sub("自动"),
            &todo_sub("脚本"),
            &todo_sub("导入"),
            &todo("从 iPhone 或 iPad 导入", None),
            &sep(),
            &todo("文件简介...", Some("CmdOrCtrl+Shift+Alt+I")),
            &todo("版本历史记录", None),
            &sep(),
            &todo("打印...", Some("CmdOrCtrl+P")),
            &todo("打印一份", Some("CmdOrCtrl+Shift+Alt+P")),
        ],
    );

    let edit = Submenu::with_items(
        "编辑",
        true,
        &[
            &item("撤销", Command::Undo),
            &item("重做", Command::Redo),
            &item("切换上一个状态", Command::ToggleLastState),
            &sep(),
            &item("渐隐...", Command::Fade),
            &sep(),
            // With a text field focused these pass the edit on to it
            &item("剪切", Command::Cut),
            &item("拷贝", Command::Copy),
            &item("合并拷贝", Command::CopyMerged),
            &item("粘贴", Command::Paste),
            &Submenu::with_items(
                "选择性粘贴",
                true,
                &[
                    &item("原位粘贴", Command::PasteInPlace),
                    &item("贴入", Command::PasteInto),
                    &item("外部粘贴", Command::PasteOutside),
                ],
            )
            .expect("static menu definition is valid"),
            &plain_item("清除", Command::Clear),
            &sep(),
            &todo("搜索", None),
            &todo("拼写检查...", None),
            &todo("查找和替换文本...", None),
            &sep(),
            &item("填充...", Command::Fill),
            &todo("描边...", None),
            &todo("内容识别填充...", None),
            &sep(),
            &todo("内容识别缩放", None),
            &todo("操控变形", None),
            &todo("透视变形", None),
            &item("自由变换", Command::FreeTransform),
            &Submenu::with_items(
                "变换",
                true,
                &[
                    &item("再次", Command::TransformAgain),
                    &sep(),
                    &item("缩放", TRANSFORM_MODES[0]),
                    &item("旋转", TRANSFORM_MODES[1]),
                    &item("斜切", TRANSFORM_MODES[2]),
                    &item("扭曲", TRANSFORM_MODES[3]),
                    &item("透视", TRANSFORM_MODES[4]),
                    &sep(),
                    &item("变形", WARP),
                    &todo("水平拆分变形", None),
                    &todo("垂直拆分变形", None),
                    &todo("交叉拆分变形", None),
                    &todo("移除变形拆分", None),
                    &sep(),
                    &todo("转换变形锚点", None),
                    &sep(),
                    &todo("切换参考线", None),
                    &sep(),
                    &item("旋转 180°", Command::TransformRotate180),
                    &item("顺时针旋转 90°", Command::TransformRotate90Clockwise),
                    &item("逆时针旋转 90°", Command::TransformRotate90CounterClockwise),
                    &sep(),
                    &item("水平翻转", Command::TransformFlipHorizontal),
                    &item("垂直翻转", Command::TransformFlipVertical),
                ],
            )
            .expect("static menu definition is valid"),
            &todo("自动对齐图层...", None),
            &todo("自动混合图层...", None),
            &sep(),
            &todo("定义画笔预设...", None),
            &item("定义图案...", Command::DefinePattern),
            &todo("定义自定形状...", None),
            &sep(),
            &todo_sub("清理"),
            &sep(),
            &todo("颜色设置...", Some("CmdOrCtrl+Shift+K")),
            &todo("指定配置文件...", None),
            &todo("转换为配置文件...", None),
            &sep(),
            &todo("键盘快捷键...", Some("CmdOrCtrl+Shift+Alt+K")),
            &todo("菜单...", Some("CmdOrCtrl+Shift+Alt+M")),
            &todo("工具栏...", None),
        ],
    );

    let image = Submenu::with_items(
        "图像",
        true,
        &[
            &todo_sub("模式"),
            &sep(),
            &Submenu::with_items(
                "调整",
                true,
                &[
                    &item("亮度/对比度...", Command::BrightnessContrast),
                    &item("色阶...", Command::Levels),
                    &item("曲线...", Command::Curves),
                    &item("曝光度...", Command::Exposure),
                    &sep(),
                    &item("自然饱和度...", Command::Vibrance),
                    &item("色相/饱和度...", Command::HueSaturation),
                    &item("色彩平衡...", Command::ColorBalance),
                    &item("黑白...", Command::BlackWhite),
                    &item("照片滤镜...", Command::PhotoFilter),
                    &item("通道混合器...", Command::ChannelMixer),
                    &item("颜色查找...", Command::ColorLookup),
                    &sep(),
                    &item("反相", Command::Invert),
                    &item("色调分离...", Command::Posterize),
                    &item("阈值...", Command::Threshold),
                    &item("渐变映射...", Command::GradientMap),
                    &item("可选颜色...", Command::SelectiveColor),
                    &sep(),
                    &item("阴影/高光...", Command::ShadowsHighlights),
                    &item("HDR 色调...", Command::HdrToning),
                    &sep(),
                    &item("去色", Command::Desaturate),
                    &item("匹配颜色...", Command::MatchColor),
                    &item("替换颜色...", Command::ReplaceColor),
                    &item("色调均化", Command::Equalize),
                ],
            )
            .expect("static menu definition is valid"),
            &sep(),
            &item("自动色调", Command::AutoTone),
            &item("自动对比度", Command::AutoContrast),
            &item("自动颜色", Command::AutoColor),
            &sep(),
            &item("图像大小...", Command::ImageSize),
            &todo("生成式放大...", Some("CmdOrCtrl+Shift+Alt+U")),
            &item("画布大小...", Command::CanvasSize),
            &Submenu::with_items(
                "图像旋转",
                true,
                &[
                    &item("180°", Command::Rotate180),
                    &item("顺时针 90°", Command::Rotate90Clockwise),
                    &item("逆时针 90°", Command::Rotate90CounterClockwise),
                    &item("任意角度...", Command::RotateArbitrary),
                    &sep(),
                    &item("水平翻转画布", Command::FlipCanvasHorizontal),
                    &item("垂直翻转画布", Command::FlipCanvasVertical),
                ],
            )
            .expect("static menu definition is valid"),
            &item("裁剪", Command::Crop),
            &item("修边...", Command::Trim),
            &item("显示全部", Command::RevealAll),
            &sep(),
            &todo("复制...", None),
            &todo("应用图像...", None),
            &todo("计算...", None),
            &sep(),
            &todo_sub("变量"),
            &todo("应用数据组...", None),
            &sep(),
            &todo("陷印...", None),
            &sep(),
            &todo_sub("分析"),
        ],
    );

    let layer = Submenu::with_items(
        "图层",
        true,
        &[
            &Submenu::with_items(
                "新建",
                true,
                &[
                    &item("图层...", Command::NewLayer),
                    &item("背景图层转为普通图层...", Command::LayerFromBackground),
                    &item("组...", Command::NewGroup),
                    &item("从图层新建组...", Command::NewGroupFromLayers),
                    &todo("画板...", None),
                    &todo("从组新建画板", None),
                    &todo("从图层新建画板...", None),
                    &todo("从图层新建框架...", None),
                    &todo("转换为框架", None),
                    &sep(),
                    &item("通过拷贝的图层", Command::LayerViaCopy),
                    &item("通过剪切的图层", Command::LayerViaCut),
                ],
            )
            .expect("static menu definition is valid"),
            &todo("拷贝 CSS", None),
            &todo("拷贝 SVG", None),
            &item("复制图层...", Command::DuplicateLayer),
            &Submenu::with_items(
                "删除",
                true,
                &[
                    &item("图层", Command::DeleteLayer),
                    &item("隐藏图层", Command::DeleteHiddenLayers),
                ],
            )
            .expect("static menu definition is valid"),
            &sep(),
            &item("快速导出为 PNG", Command::LayerQuickExportPng),
            &item("导出为...", Command::LayerExportAs),
            &sep(),
            &item("重命名图层...", Command::RenameLayer),
            &todo_sub("图层样式"),
            &todo_sub("智能滤镜"),
            &sep(),
            &todo_sub("新建填充图层"),
            &todo_sub("新建调整图层"),
            &todo("协调", None),
            &todo("图层内容选项...", None),
            &sep(),
            &Submenu::with_items(
                "图层蒙版",
                true,
                &[
                    &item("显示全部", Command::MaskRevealAll),
                    &item("隐藏全部", Command::MaskHideAll),
                    &item("显示选区", Command::MaskRevealSelection),
                    &item("隐藏选区", Command::MaskHideSelection),
                    &todo("从透明区域", None),
                    &sep(),
                    &item("删除", Command::MaskDelete),
                    &item("应用", Command::MaskApply),
                    &sep(),
                    &item("停用", Command::MaskToggle),
                    &todo("链接", None),
                ],
            )
            .expect("static menu definition is valid"),
            &todo_sub("矢量蒙版"),
            &todo("创建剪贴蒙版", Some("CmdOrCtrl+Alt+G")),
            &todo("为所有对象添加蒙版", None),
            &sep(),
            &todo_sub("智能对象"),
            &todo_sub("视频图层"),
            &todo_sub("栅格化"),
            &sep(),
            &todo("新建基于图层的切片", None),
            &sep(),
            &item("编组图层", Command::GroupLayers),
            &item("取消图层编组", Command::UngroupLayers),
            &item("隐藏图层", Command::ToggleLayerVisibility),
            &sep(),
            &Submenu::with_items(
                "排列",
                true,
                &[
                    &item("置为顶层", Command::BringToFront),
                    &item("前移一层", Command::BringForward),
                    &item("后移一层", Command::SendBackward),
                    &item("置为底层", Command::SendToBack),
                    &item("反向", Command::ArrangeReverse),
                ],
            )
            .expect("static menu definition is valid"),
            &todo_sub("合并形状"),
            &sep(),
            &Submenu::with_items(
                "对齐",
                true,
                &[
                    &item("顶边", Command::Align(Align::Top)),
                    &item("垂直居中", Command::Align(Align::VerticalCenter)),
                    &item("底边", Command::Align(Align::Bottom)),
                    &sep(),
                    &item("左边", Command::Align(Align::Left)),
                    &item("水平居中", Command::Align(Align::HorizontalCenter)),
                    &item("右边", Command::Align(Align::Right)),
                ],
            )
            .expect("submenu"),
            &Submenu::with_items(
                "分布",
                true,
                &[
                    &item("顶边", Command::Distribute(Distribute::Top)),
                    &item("垂直居中", Command::Distribute(Distribute::VerticalCenter)),
                    &item("底边", Command::Distribute(Distribute::Bottom)),
                    &sep(),
                    &item("左边", Command::Distribute(Distribute::Left)),
                    &item(
                        "水平居中",
                        Command::Distribute(Distribute::HorizontalCenter),
                    ),
                    &item("右边", Command::Distribute(Distribute::Right)),
                    &sep(),
                    &item("水平", Command::Distribute(Distribute::Horizontally)),
                    &item("垂直", Command::Distribute(Distribute::Vertically)),
                ],
            )
            .expect("submenu"),
            &sep(),
            &item("锁定图层...", Command::LockLayers),
            &sep(),
            &item("链接图层", Command::LinkLayers),
            &item("选择链接图层", Command::SelectLinkedLayers),
            &sep(),
            &item("向下合并", Command::MergeDown),
            &item("合并可见图层", Command::MergeVisible),
            &item("拼合图像", Command::FlattenImage),
            &sep(),
            &todo_sub("修边处理"),
        ],
    );

    let select = Submenu::with_items(
        "选择",
        true,
        &[
            &item("全部", Command::SelectAll),
            &item("取消选择", Command::Deselect),
            &item("重新选择", Command::Reselect),
            &item("反选", Command::SelectInverse),
            &sep(),
            &item("所有图层", Command::SelectAllLayers),
            &item("取消选择图层", Command::DeselectLayers),
            &todo("查找图层", Some("CmdOrCtrl+Shift+Alt+F")),
            &todo("隔离图层", None),
            &sep(),
            &todo("色彩范围...", None),
            &todo("焦点区域...", None),
            &todo("主体", None),
            &todo("天空", None),
            &sep(),
            &todo("选择并遮住...", Some("CmdOrCtrl+Alt+R")),
            &Submenu::with_items(
                "修改",
                true,
                &[
                    &item("边界...", Command::ModifyBorder),
                    &item("平滑...", Command::ModifySmooth),
                    &item("扩展...", Command::ModifyExpand),
                    &item("收缩...", Command::ModifyContract),
                    &item("羽化...", Command::ModifyFeather),
                ],
            )
            .expect("static menu definition is valid"),
            &sep(),
            &item("扩大选取", Command::Grow),
            &item("选取相似", Command::Similar),
            &sep(),
            &item("变换选区", Command::TransformSelection),
            &sep(),
            &check_item("以快速蒙版模式编辑", Command::QuickMask),
            &sep(),
            &todo("载入选区...", None),
            &todo("存储选区...", None),
        ],
    );

    let type_menu = Submenu::with_items(
        "文字",
        true,
        &[
            // Photoshop's first item, More from Adobe Fonts..., and its
            // separator are left out: the app shows no Adobe brand
            &todo_sub("面板"),
            &sep(),
            &todo_sub("消除锯齿"),
            &sep(),
            &todo_sub("方向"),
            &todo_sub("OpenType"),
            &sep(),
            &todo("创建工作路径", None),
            &todo("转换为形状", None),
            &sep(),
            &todo("栅格化文字图层", None),
            &todo("转换为段落文本", None),
            &todo_sub("转换为动态文本"),
            &todo("文字变形...", None),
            &todo("匹配字体...", None),
            &sep(),
            &todo_sub("字体预览大小"),
            &sep(),
            &todo_sub("语言选项"),
            &sep(),
            &todo("更新所有文字图层", None),
            &todo("管理缺失字体", None),
            &sep(),
            &todo("粘贴占位文本", None),
            &sep(),
            &todo("载入默认文字样式", None),
            &todo("存储默认文字样式", None),
        ],
    );

    // Items of a Filter submenu: implemented filters by command, the
    // rest disabled
    let filter_sub = |label: &str, entries: &[(&str, Option<Command>)]| {
        let built: Vec<MenuNode> = entries
            .iter()
            .map(|&(name, command)| match command {
                Some(command) => item(name, command),
                None => todo(name, None),
            })
            .collect();
        let refs: Vec<&MenuNode> = built.iter().collect();
        Submenu::with_items(label, true, &refs).expect("static menu definition is valid")
    };
    let filter = Submenu::with_items(
        "滤镜",
        true,
        &[
            &item("上次滤镜", Command::LastFilter),
            &sep(),
            &todo("转换为智能滤镜", None),
            &sep(),
            &todo("神经网络滤镜...", None),
            &sep(),
            &todo("滤镜库...", None),
            &todo("自适应广角...", Some("CmdOrCtrl+Shift+Alt+A")),
            &todo("Camera Raw 滤镜...", Some("CmdOrCtrl+Shift+A")),
            &todo("AI 降噪...", None),
            &todo("AI 锐化...", None),
            &todo("镜头校正...", Some("CmdOrCtrl+Shift+R")),
            &todo("液化...", Some("CmdOrCtrl+Shift+X")),
            &todo("消失点...", Some("CmdOrCtrl+Alt+V")),
            &sep(),
            &filter_sub(
                "模糊",
                &[
                    ("平均", Some(Command::Average)),
                    ("模糊", Some(Command::Blur)),
                    ("进一步模糊", Some(Command::BlurMore)),
                    ("方框模糊...", Some(Command::BoxBlur)),
                    ("高斯模糊...", Some(Command::GaussianBlur)),
                    ("镜头模糊...", Some(Command::LensBlur)),
                    ("动感模糊...", Some(Command::MotionBlur)),
                    ("径向模糊...", Some(Command::RadialBlur)),
                    ("形状模糊...", Some(Command::ShapeBlur)),
                    ("特殊模糊...", Some(Command::SmartBlur)),
                    ("表面模糊...", Some(Command::SurfaceBlur)),
                ],
            ),
            &todo_sub("模糊画廊"),
            &filter_sub(
                "扭曲",
                &[
                    ("置换...", Some(Command::Displace)),
                    ("挤压...", Some(Command::Pinch)),
                    ("极坐标...", Some(Command::PolarCoordinates)),
                    ("波纹...", Some(Command::Ripple)),
                    ("切变...", Some(Command::Shear)),
                    ("球面化...", Some(Command::Spherize)),
                    ("旋转扭曲...", Some(Command::Twirl)),
                    ("波浪...", Some(Command::Wave)),
                    ("水波...", Some(Command::ZigZag)),
                ],
            ),
            &filter_sub(
                "杂色",
                &[
                    ("添加杂色...", Some(Command::AddNoise)),
                    ("去斑", Some(Command::Despeckle)),
                    ("蒙尘与划痕...", Some(Command::DustAndScratches)),
                    ("中间值...", Some(Command::Median)),
                    ("减少杂色...", Some(Command::ReduceNoise)),
                ],
            ),
            &filter_sub(
                "像素化",
                &[
                    ("彩色半调...", Some(Command::ColorHalftone)),
                    ("晶格化...", Some(Command::Crystallize)),
                    ("彩块化", Some(Command::Facet)),
                    ("碎片", Some(Command::Fragment)),
                    ("铜版雕刻...", Some(Command::Mezzotint)),
                    ("马赛克...", Some(Command::Mosaic)),
                    ("点状化...", Some(Command::Pointillize)),
                ],
            ),
            &Submenu::with_items(
                "渲染",
                true,
                &[
                    &todo("火焰...", None),
                    &todo("图片框...", None),
                    &todo("树...", None),
                    &sep(),
                    &item("云彩", Command::Clouds),
                    &item("分层云彩", Command::DifferenceClouds),
                    &item("纤维...", Command::Fibers),
                    &item("镜头光晕...", Command::LensFlare),
                ],
            )
            .expect("static menu definition is valid"),
            &filter_sub(
                "锐化",
                &[
                    ("锐化", Some(Command::Sharpen)),
                    ("锐化边缘", Some(Command::SharpenEdges)),
                    ("进一步锐化", Some(Command::SharpenMore)),
                    ("智能锐化...", Some(Command::SmartSharpen)),
                    ("USM 锐化...", Some(Command::UnsharpMask)),
                ],
            ),
            &filter_sub(
                "风格化",
                &[
                    ("扩散...", Some(Command::Diffuse)),
                    ("浮雕效果...", Some(Command::Emboss)),
                    ("凸出...", Some(Command::Extrude)),
                    ("查找边缘", Some(Command::FindEdges)),
                    ("油画...", Some(Command::OilPaint)),
                    ("曝光过度", Some(Command::Solarize)),
                    ("拼贴...", Some(Command::Tiles)),
                    ("等高线...", Some(Command::TraceContour)),
                    ("风...", Some(Command::Wind)),
                ],
            ),
            &todo_sub("视频"),
            &filter_sub(
                "其他",
                &[
                    ("自定...", Some(Command::CustomFilter)),
                    ("高反差保留...", Some(Command::HighPass)),
                    ("HSB/HSL", Some(Command::HsbHsl)),
                    ("最大值...", Some(Command::Maximum)),
                    ("最小值...", Some(Command::Minimum)),
                    ("位移...", Some(Command::Offset)),
                ],
            ),
        ],
    );

    let plugins = Submenu::with_items(
        "增效工具",
        true,
        &[&todo("增效工具面板", None), &todo("管理增效工具...", None)],
    );

    let view = Submenu::with_items(
        "视图",
        true,
        &[
            &todo_sub("校样设置"),
            &todo("校样颜色", Some("CmdOrCtrl+Y")),
            &todo("色域警告", Some("CmdOrCtrl+Shift+Y")),
            &todo_sub("像素长宽比"),
            &todo("像素长宽比校正", None),
            &todo("32 位预览选项...", None),
            &sep(),
            &item("放大", Command::ZoomIn),
            &item("缩小", Command::ZoomOut),
            &item("适合屏幕", Command::FitOnScreen),
            &item("图层适合屏幕", Command::FitLayers),
            &todo("画板适合屏幕", None),
            &item("100%", Command::ActualPixels),
            &item("200%", Command::Zoom200),
            &item("打印尺寸", Command::PrintSize),
            &item("实际大小", Command::ActualSize),
            &check_item("水平翻转", Command::FlipView),
            &todo("图案预览", None),
            &sep(),
            &Submenu::with_items(
                "屏幕模式",
                true,
                &[
                    &check_item(
                        "标准屏幕模式",
                        Command::ScreenMode(crate::state::ScreenMode::Standard),
                    ),
                    &check_item(
                        "带菜单栏的全屏模式",
                        Command::ScreenMode(crate::state::ScreenMode::FullWithMenus),
                    ),
                    &check_item(
                        "全屏模式",
                        Command::ScreenMode(crate::state::ScreenMode::Full),
                    ),
                ],
            )
            .expect("static menu definition is valid"),
            &sep(),
            &check_item("额外内容", Command::ToggleExtras),
            &Submenu::with_items(
                "显示",
                true,
                &[
                    &check_item("图层边缘", Command::ToggleLayerEdges),
                    &check_item("选区边缘", Command::ToggleSelectionEdges),
                    &todo("目标路径", Some("CmdOrCtrl+Shift+H")),
                    &check_item("网格", Command::ToggleGrid),
                    &check_item("参考线", Command::ToggleGuides),
                    &check_item("画布参考线", Command::ToggleCanvasGuides),
                    &todo("画板参考线", None),
                    &todo("画板名称", None),
                    &todo("计数", None),
                    &check_item("智能参考线", Command::ToggleSmartGuides),
                    &todo("切片", None),
                    &todo("注释", None),
                    &check_item("像素网格", Command::TogglePixelGrid),
                    &todo("图案预览拼贴边界", None),
                    &sep(),
                    &todo("网格线", None),
                    &todo("编辑图钉", None),
                    &sep(),
                    &item("全部", Command::ShowAllExtras),
                    &item("无", Command::ShowNoExtras),
                    &sep(),
                    &todo("显示额外内容选项...", None),
                ],
            )
            .expect("static menu definition is valid"),
            &sep(),
            &check_item("标尺", Command::ToggleRulers),
            &sep(),
            &check_item("对齐", Command::ToggleSnap),
            &Submenu::with_items(
                "对齐到",
                true,
                &[
                    &check_item("参考线", Command::SnapToGuides),
                    &check_item("网格", Command::SnapToGrid),
                    &check_item("图层", Command::SnapToLayers),
                    &check_item("切片", Command::SnapToSlices),
                    &check_item("文档边界", Command::SnapToBounds),
                    &sep(),
                    &item("全部", Command::SnapToAll),
                    &item("无", Command::SnapToNone),
                ],
            )
            .expect("static menu definition is valid"),
            &sep(),
            &Submenu::with_items(
                "参考线",
                true,
                &[
                    &item("编辑所选参考线...", Command::EditSelectedGuides),
                    &check_item("锁定参考线", Command::LockGuides),
                    &item("清除参考线", Command::ClearGuides),
                    &item("清除所选参考线", Command::ClearSelectedGuides),
                    &todo("清除所选画板参考线", None),
                    &item("清除画布参考线", Command::ClearCanvasGuides),
                    &item("新建参考线...", Command::NewGuide),
                    &item("新建参考线版面...", Command::NewGuideLayout),
                    &todo("从形状新建参考线", None),
                ],
            )
            .expect("static menu definition is valid"),
            &sep(),
            &todo("锁定切片", None),
            &todo("清除切片", None),
        ],
    );

    let window = Submenu::with_items(
        "窗口",
        true,
        &[
            &todo_sub("排列"),
            &Submenu::with_items(
                "工作区",
                true,
                &[
                    &check_item("基本功能（默认）", Command::WorkspaceEssentials),
                    &sep(),
                    &todo("3D", None),
                    &todo("图形和 Web", None),
                    &todo("动感", None),
                    &todo("绘画", None),
                    &todo("摄影", None),
                    &sep(),
                    &item("复位基本功能", Command::ResetWorkspace),
                    &todo("新建工作区...", None),
                    &todo("删除工作区...", None),
                    &todo("键盘快捷键和菜单...", None),
                    &todo("锁定工作区", None),
                ],
            )
            .expect("static menu definition is valid"),
            &sep(),
            &todo("动作", Some("Alt+F9")),
            &todo("调整", None),
            &check_item("画笔设置", Command::ToggleBrushSettings),
            &check_item("画笔", Command::ToggleBrushes),
            &todo("通道", None),
            &todo("字符", None),
            &todo("字符样式", None),
            &check_item("仿制源", Command::ToggleCloneSource),
            &todo("颜色", Some("F6")),
            &todo("评论", None),
            &todo("内容凭据（测试版）", None),
            &todo("字形", None),
            &todo("渐变", None),
            &check_item("直方图", Command::ToggleHistogram),
            &check_item("历史记录", Command::ToggleHistory),
            &check_item("信息", Command::ToggleInfo),
            &check_item("图层复合", Command::ToggleLayerComps),
            &todo("图层", Some("F7")),
            &todo("库", None),
            &todo("材质", None),
            &todo("测量记录", None),
            &check_item("导航器", Command::ToggleNavigator),
            &todo("注释", None),
            &todo("段落", None),
            &todo("段落样式", None),
            &todo("路径", None),
            &todo("图案", None),
            &todo("属性", None),
            &todo("形状", None),
            &todo("样式", None),
            &todo("色板", None),
            &todo("时间轴", None),
            &check_item("工具预设", Command::ToggleToolPresets),
            &todo("版本历史记录", None),
            &sep(),
            &todo("应用程序框架", None),
            &todo("选项", None),
            &todo("工具", None),
            &todo("上下文任务栏", None),
        ],
    );
    let help = Submenu::with_items("帮助", true, &[&todo("OpenPhoto 帮助", None)]);

    [
        app_menu, file, edit, image, layer, type_menu, select, filter, view, plugins, window, help,
    ]
    .into_iter()
    .map(|node| node.expect("static menu definition is valid"))
    .collect()
}

fn dynamic_label(command: Command, app: &AppState) -> Option<String> {
    let doc = app.active_doc.and_then(|id| app.docs.get(&id));
    match command {
        Command::Undo => Some(match doc.and_then(|d| d.history.undo_name()) {
            Some(name) => format!("撤销{}", crate::state::display_name(name)),
            None => "撤销".into(),
        }),
        Command::Redo => Some(match doc.and_then(|d| d.history.redo_name()) {
            Some(name) => format!("重做{}", crate::state::display_name(name)),
            None => "重做".into(),
        }),
        // Disable or Enable, for the active layer's mask
        Command::MaskToggle => {
            let mask = doc
                .and_then(|d| d.doc.active_layer.and_then(|id| d.doc.layer(id)))
                .and_then(|l| l.mask.as_ref());
            Some(
                if mask.is_some_and(|m| !m.enabled) {
                    "启用"
                } else {
                    "停用"
                }
                .into(),
            )
        }
        // "Fade Blur..." for the edit it would fade
        Command::Fade => Some(match doc.and_then(|d| d.fade.as_ref()) {
            Some((_, name)) if command.enabled(app) => {
                format!("渐隐{}...", crate::state::display_name(name))
            }
            _ => "渐隐...".into(),
        }),
        // Like Photoshop, the item names the last filter used
        Command::LastFilter => Some(app.last_filter.map_or_else(
            || "上次滤镜".to_owned(),
            |f| crate::state::display_name(f.name()),
        )),
        // Merge Layers with several layers selected
        Command::MergeDown => Some(
            if doc.is_some_and(|d| d.doc.selected_layers().len() > 1) {
                "合并图层"
            } else if doc.is_some_and(|d| {
                d.doc
                    .active_layer
                    .and_then(|id| d.doc.layer(id))
                    .is_some_and(|l| l.is_group())
            }) {
                "合并组"
            } else {
                "向下合并"
            }
            .into(),
        ),
        // Equalize asks how to use a selection
        Command::Equalize => Some(
            if doc.is_some_and(|d| d.doc.selection().is_some()) {
                "色调均化..."
            } else {
                "色调均化"
            }
            .into(),
        ),
        // Unlink Layers when every selected layer is linked
        Command::LinkLayers => Some(
            if doc.is_some_and(|d| op_core::link::can_unlink(&d.doc)) {
                "取消链接图层"
            } else {
                "链接图层"
            }
            .into(),
        ),
        Command::ToggleLayerVisibility => {
            // Show Layers when every selected layer is hidden
            let hidden = doc.is_some_and(|d| {
                let selected = d.doc.selected_layers();
                !selected.is_empty()
                    && selected
                        .iter()
                        .all(|&id| d.doc.layer(id).is_some_and(|l| !l.visible))
            });
            Some(
                if hidden {
                    "显示图层"
                } else {
                    "隐藏图层"
                }
                .into(),
            )
        }
        _ => None,
    }
}

#[cfg(target_os = "macos")]
mod native {
    use super::*;
    use muda::accelerator::{Accelerator, KeyAccelerator};
    use muda::{
        AboutMetadata, CheckMenuItem, ContextMenu, IsMenuItem, Menu, MenuEvent, MenuItem,
        PredefinedMenuItem, Submenu,
    };
    use std::str::FromStr;
    use std::sync::mpsc::{Receiver, channel};

    fn build_native(
        node: &MenuNode,
        items: &mut Vec<(Command, MenuItem)>,
        checks: &mut Vec<(Command, CheckMenuItem)>,
        recent: &mut Option<Submenu>,
    ) -> muda::MenuItemKind {
        match node {
            MenuNode::Item {
                label,
                command,
                shortcut,
                check,
            } => {
                let accel = shortcut.as_deref().and_then(accelerator);
                if let Some(command) = command {
                    if *check {
                        let item = CheckMenuItem::with_id(id(*command), label, false, false, accel);
                        checks.push((*command, item.clone()));
                        item.kind()
                    } else {
                        let item = MenuItem::with_id(id(*command), label, false, accel);
                        if *command == Command::ZoomIn {
                            let _ = item
                                .set_key_accelerator(KeyAccelerator::from_str("CmdOrCtrl++").ok());
                        }
                        items.push((*command, item.clone()));
                        item.kind()
                    }
                } else {
                    MenuItem::new(label, false, accel).kind()
                }
            }
            MenuNode::Submenu {
                label,
                enabled,
                children,
            } => {
                let submenu = Submenu::new(label, *enabled);
                for child in children {
                    let child = build_native(child, items, checks, recent);
                    let child: &dyn IsMenuItem = match &child {
                        muda::MenuItemKind::MenuItem(item) => item,
                        muda::MenuItemKind::Submenu(item) => item,
                        muda::MenuItemKind::Predefined(item) => item,
                        muda::MenuItemKind::Check(item) => item,
                        muda::MenuItemKind::Icon(item) => item,
                    };
                    submenu.append(child).expect("append menu item");
                }
                submenu.kind()
            }
            MenuNode::Separator => PredefinedMenuItem::separator().kind(),
            MenuNode::Recent => {
                let submenu = Submenu::new("最近打开文件", true);
                *recent = Some(submenu.clone());
                submenu.kind()
            }
            MenuNode::System(system) => match system {
                SystemItem::About => PredefinedMenuItem::about(
                    Some("关于 OpenPhoto"),
                    Some(AboutMetadata {
                        name: Some("OpenPhoto".into()),
                        version: Some(env!("CARGO_PKG_VERSION").into()),
                        ..Default::default()
                    }),
                ),
                SystemItem::Services => PredefinedMenuItem::services(Some("服务")),
                SystemItem::HideOthers => PredefinedMenuItem::hide_others(Some("隐藏其他")),
                SystemItem::ShowAll => PredefinedMenuItem::show_all(Some("显示全部")),
            }
            .kind(),
        }
    }

    /// Where a dropdown's menu opens against its button (view points), as
    /// macOS places a pop-up button's.
    const POPUP_LEFT: f64 = -13.5;
    const POPUP_TOP: f64 = -4.5;
    const POPUP_WIDER: f64 = 0.0;

    fn id(command: Command) -> String {
        format!("{command:?}")
    }

    /// "⌥1" as a menu key equivalent.
    fn popup_accelerator(key: &str) -> Option<Accelerator> {
        let digit = key.strip_prefix('\u{2325}')?;
        accelerator(&format!("Alt+{digit}"))
    }

    fn accelerator(text: &str) -> Option<Accelerator> {
        Accelerator::from_str(text).ok()
    }

    pub struct NativeMenu {
        /// Keeps the native menu alive.
        _menu: Menu,
        items: Vec<(Command, MenuItem)>,
        /// View menu switches, shown with a check mark.
        checks: Vec<(Command, CheckMenuItem)>,
        events: Receiver<Command>,
        /// Last applied (enabled, dynamic label) per item, to avoid redundant
        /// native calls.
        applied: Vec<(bool, Option<String>)>,
        /// The window's NSView, for menus that pop up in it.
        view: Option<usize>,
        /// File › Open Recent, rebuilt when the list changes (the revision it
        /// shows).
        open_recent: Submenu,
        recent_shown: u64,
        /// Picks from dialog dropdown menus (`native_popup`), by entry index.
        popup_events: Receiver<usize>,
    }

    /// Menu ids of dialog dropdown entries.
    const POPUP_PREFIX: &str = "popup:";

    impl NativeMenu {
        pub fn install(ctx: &egui::Context) -> Self {
            let mut items = Vec::new();
            let mut checks = Vec::new();
            let mut open_recent = None;
            let menu = Menu::new();
            let submenus: Vec<Submenu> = menu_tree()
                .iter()
                .map(
                    |node| match build_native(node, &mut items, &mut checks, &mut open_recent) {
                        muda::MenuItemKind::Submenu(submenu) => submenu,
                        _ => unreachable!("top-level menus are submenus"),
                    },
                )
                .collect();
            for submenu in &submenus {
                menu.append(submenu).expect("append submenu");
            }
            menu.init_for_nsapp();
            // "Window" also gets the standard window list from macOS
            submenus[10].set_as_windows_menu_for_nsapp();

            // Menu events arrive on the main thread outside egui's frame; queue
            // them and wake egui up.
            let (tx, events) = channel();
            let (popup_tx, popup_events) = channel();
            let ctx = ctx.clone();
            MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
                if let Some(index) = event
                    .id
                    .as_ref()
                    .strip_prefix(POPUP_PREFIX)
                    .and_then(|n| n.parse::<usize>().ok())
                {
                    let _ = popup_tx.send(index);
                    ctx.request_repaint();
                    return;
                }
                if let Some(&command) = ALL_COMMANDS.iter().find(|c| event.id == id(**c).as_str()) {
                    // Lock Layers... shows Cmd+/, but in Photoshop the key toggles
                    // Lock all; only the menu opens the dialog
                    let command =
                        if command == Command::LockLayers && crate::app_kit::handling_key_press() {
                            Command::ToggleLockAll
                        } else {
                            command
                        };
                    let _ = tx.send(command);
                    ctx.request_repaint();
                }
            }));

            let applied = vec![(false, None); items.len()];
            Self {
                _menu: menu,
                items,
                checks,
                events,
                applied,
                view: None,
                open_recent: open_recent.expect("recent menu"),
                recent_shown: 0,
                popup_events,
            }
        }

        /// Rebuilds File › Open Recent from the list: the files' names newest
        /// first, a separator and Clear Recent File List (disabled when empty),
        /// as in Photoshop.
        fn sync_open_recent(&mut self, app: &AppState) {
            if self.recent_shown == app.recent.revision() {
                return;
            }
            self.recent_shown = app.recent.revision();
            while self.open_recent.remove_at(0).is_some() {}
            for (i, path) in app.recent.files().iter().enumerate() {
                let name = path.file_name().map_or_else(
                    || path.display().to_string(),
                    |n| n.to_string_lossy().into_owned(),
                );
                let item = MenuItem::with_id(id(Command::OpenRecent(i as u8)), name, true, None);
                let _ = self.open_recent.append(&item);
            }
            let _ = self.open_recent.append(&PredefinedMenuItem::separator());
            let clear = MenuItem::with_id(
                id(Command::ClearRecent),
                "清除最近使用的文件列表",
                !app.recent.files().is_empty(),
                None,
            );
            let _ = self.open_recent.append(&clear);
        }

        /// The window's NSView (from its raw window handle), where pop-up menus
        /// open.
        pub fn set_view(&mut self, view: *mut std::ffi::c_void) {
            self.view = Some(view as usize);
        }

        /// The status bar's menu, as Photoshop's native one: what the bar
        /// shows, the current item checked.
        pub fn popup_status(&self, app: &AppState) {
            let items = StatusInfo::ALL.map(|info| {
                (
                    Command::StatusInfo(info),
                    info.label(),
                    app.status_info == info,
                )
            });
            self.popup(&items);
        }

        /// A ruler's right-click menu: the units, the current one checked.
        pub fn popup_ruler_units(&self, app: &AppState) {
            let items = RulerUnit::ALL.map(|unit| {
                (
                    Command::RulerUnits(unit),
                    unit.label(),
                    app.ruler_units == unit,
                )
            });
            self.popup(&items);
        }

        /// A native context menu of check items opening down from the pointer;
        /// a pick arrives as a command like the menu bar's.
        fn popup(&self, items: &[(Command, &str, bool)]) {
            let Some(view) = self.view else {
                return;
            };
            let menu = Menu::new();
            for &(command, label, checked) in items {
                let item = CheckMenuItem::with_id(id(command), label, true, checked, None);
                let _ = menu.append(&item);
            }
            // SAFETY: the window's live content view, on the main thread
            unsafe {
                menu.show_context_menu_for_nsview(view as *const std::ffi::c_void, None);
            }
        }

        /// A dialog dropdown's menu as macOS shows a pop-up button's: the
        /// checked entry over the button's value, at least as wide as the
        /// button. Returns the picked entry's index once the menu closes.
        /// `scale` turns egui points into the view's points.
        pub fn popup_menu(
            &self,
            request: &crate::native_popup::Request,
            scale: f32,
        ) -> Option<usize> {
            use crate::native_popup::Entry;
            use objc2::msg_send;
            use objc2::runtime::AnyObject;
            use objc2_foundation::{NSPoint, NSRect};
            let view = self.view? as *mut AnyObject;
            while self.popup_events.try_recv().is_ok() {}
            let menu = Menu::new();
            for (i, entry) in request.entries.iter().enumerate() {
                let _ = match entry {
                    Entry::Item {
                        label,
                        checked,
                        enabled,
                        shortcut,
                    } => menu.append(&CheckMenuItem::with_id(
                        format!("{POPUP_PREFIX}{i}"),
                        label,
                        *enabled,
                        *checked,
                        shortcut.as_deref().and_then(popup_accelerator),
                    )),
                    Entry::Separator => menu.append(&PredefinedMenuItem::separator()),
                };
            }
            let b = request.button;
            // SAFETY: the window's live content view and the menu's NSMenu, on
            // the main thread
            unsafe {
                let ns_menu = menu.ns_menu() as *mut AnyObject;
                let item: *mut AnyObject =
                    msg_send![ns_menu, itemAtIndex: request.positioning() as isize];
                let _: () =
                    msg_send![ns_menu, setMinimumWidth: (b.width() * scale) as f64 + POPUP_WIDER];
                // The system's look (light or dark), as Photoshop's menus, not
                // the window's dark one
                let app: *mut AnyObject =
                    msg_send![objc2::class!(NSApplication), sharedApplication];
                let appearance: *mut AnyObject = msg_send![app, effectiveAppearance];
                let _: () = msg_send![ns_menu, setAppearance: appearance];
                let frame: NSRect = msg_send![view, frame];
                let flipped: bool = msg_send![view, isFlipped];
                let x = (b.left() * scale) as f64 + POPUP_LEFT;
                let y = (b.top() * scale) as f64 + POPUP_TOP;
                let y = if flipped { y } else { frame.size.height - y };
                let _: bool = msg_send![
                    ns_menu,
                    popUpMenuPositioningItem: item,
                    atLocation: NSPoint::new(x, y),
                    inView: view
                ];
            }
            self.popup_events.try_recv().ok()
        }

        /// Commands chosen from the menu (or via their shortcuts) since the last call.
        pub fn poll(&self) -> Vec<Command> {
            self.events.try_iter().collect()
        }

        /// Syncs enabled states and dynamic labels ("Undo Canvas Size") with the app.
        pub fn update(&mut self, app: &AppState) {
            self.sync_open_recent(app);
            for (command, item) in &self.checks {
                let enabled = command.enabled(app);
                if item.is_enabled() != enabled {
                    item.set_enabled(enabled);
                }
                let checked = command.checked(app).unwrap_or(false);
                if item.is_checked() != checked {
                    item.set_checked(checked);
                }
            }
            for (i, (command, item)) in self.items.iter().enumerate() {
                let enabled = command.enabled(app);
                let label = dynamic_label(*command, app);
                let applied = &mut self.applied[i];
                if applied.0 != enabled {
                    item.set_enabled(enabled);
                    applied.0 = enabled;
                }
                if let Some(label) = label
                    && applied.1.as_ref() != Some(&label)
                {
                    item.set_text(&label);
                    applied.1 = Some(label);
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        /// Every shortcut shows in the native menu: a key muda can't parse
        /// would silently drop the key equivalent.
        #[test]
        fn every_shortcut_is_a_menu_accelerator() {
            let broken: Vec<String> = ALL_COMMANDS
                .iter()
                .filter_map(|c| Some((c, c.shortcut()?.accelerator())))
                .filter(|(_, a)| accelerator(a).is_none())
                .map(|(c, a)| format!("{c:?}: {a}"))
                .collect();
            assert!(broken.is_empty(), "{broken:?}");
        }
    }
}

#[cfg(target_os = "macos")]
pub use native::NativeMenu;

/// The same menu data as the native bar. Keyboard handling stays exclusively
/// in `commands::from_shortcuts`; menu shortcut text is display-only.
#[cfg(not(target_os = "macos"))]
#[derive(Default)]
pub struct PortableMenu {
    tree: Vec<MenuNode>,
    context: Option<(egui::Pos2, bool)>,
}

#[cfg(not(target_os = "macos"))]
impl PortableMenu {
    pub fn show(&mut self, ui: &mut egui::Ui, app: &AppState) -> Option<Command> {
        if self.tree.is_empty() {
            self.tree = menu_tree();
        }
        let mut picked = None;
        egui::MenuBar::new().ui(ui, |ui| {
            for node in &self.tree {
                portable_node(ui, node, app, &mut picked);
            }
        });
        picked
    }

    /// Status/ruler requests originate in the document view and retain the
    /// pointer's position until a choice or an outside click closes them.
    pub fn show_context(&mut self, ctx: &egui::Context, app: &mut AppState) -> Option<Command> {
        let status = std::mem::take(&mut app.status_menu);
        let ruler = std::mem::take(&mut app.ruler_menu);
        let opened = status || ruler;
        if opened {
            self.context = Some((ctx.pointer_latest_pos().unwrap_or(egui::Pos2::ZERO), ruler));
        }
        let (position, ruler) = self.context?;
        let mut picked = None;
        let response = egui::Area::new(egui::Id::new("portable-document-menu"))
            .order(egui::Order::Foreground)
            .fixed_pos(position)
            .constrain(true)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style()).show(ui, |ui| {
                    if ruler {
                        for unit in RulerUnit::ALL {
                            portable_command(
                                ui,
                                unit.label(),
                                Command::RulerUnits(unit),
                                None,
                                app,
                                &mut picked,
                            );
                        }
                    } else {
                        for info in StatusInfo::ALL {
                            portable_command(
                                ui,
                                info.label(),
                                Command::StatusInfo(info),
                                None,
                                app,
                                &mut picked,
                            );
                        }
                    }
                });
            });
        let outside = ctx.input(|input| input.pointer.any_pressed())
            && ctx
                .pointer_latest_pos()
                .is_some_and(|pos| !response.response.rect.contains(pos));
        if picked.is_some()
            || ctx.input(|input| input.key_pressed(egui::Key::Escape))
            || (!opened && outside)
        {
            self.context = None;
        }
        picked
    }
}

#[cfg(not(target_os = "macos"))]
fn portable_command(
    ui: &mut egui::Ui,
    label: &str,
    command: Command,
    shortcut: Option<&str>,
    app: &AppState,
    picked: &mut Option<Command>,
) {
    let label = dynamic_label(command, app).unwrap_or_else(|| label.into());
    let mut button = egui::Button::new(label);
    if let Some(checked) = command.checked(app) {
        button = button.selected(checked);
    }
    if let Some(shortcut) = shortcut {
        button = button.shortcut_text(portable_shortcut(shortcut));
    }
    if ui
        .add_enabled(portable_enabled(command, app), button)
        .clicked()
    {
        *picked = Some(command);
        ui.close();
    }
}

#[cfg(not(target_os = "macos"))]
fn portable_enabled(command: Command, app: &AppState) -> bool {
    // Hiding an application is a macOS service; every portable command uses
    // exactly the same availability gate as execution.
    command != Command::HideApp && command.enabled(app)
}

#[cfg(not(target_os = "macos"))]
fn portable_shortcut(shortcut: &str) -> String {
    shortcut.replace("CmdOrCtrl", "Ctrl")
}

#[cfg(not(target_os = "macos"))]
fn portable_node(ui: &mut egui::Ui, node: &MenuNode, app: &AppState, picked: &mut Option<Command>) {
    match node {
        MenuNode::Item {
            label,
            command: Some(command),
            shortcut,
            check,
        } => {
            debug_assert!(!check || command.checked(app).is_some());
            portable_command(ui, label, *command, shortcut.as_deref(), app, picked);
        }
        MenuNode::Item {
            label,
            command: None,
            shortcut,
            ..
        } => {
            let mut button = egui::Button::new(label);
            if let Some(shortcut) = shortcut {
                button = button.shortcut_text(portable_shortcut(shortcut));
            }
            ui.add_enabled(false, button);
        }
        MenuNode::Submenu {
            label,
            enabled,
            children,
        } => {
            ui.add_enabled_ui(*enabled, |ui| {
                ui.menu_button(label, |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(ui.ctx().content_rect().height() * 0.8)
                        .show(ui, |ui| {
                            for child in children {
                                portable_node(ui, child, app, picked);
                            }
                        });
                });
            });
        }
        MenuNode::Separator => {
            ui.separator();
        }
        MenuNode::Recent => {
            ui.menu_button("最近打开文件", |ui| {
                for (index, path) in app.recent.files().iter().enumerate() {
                    let label = path.file_name().map_or_else(
                        || path.display().to_string(),
                        |name| name.to_string_lossy().into_owned(),
                    );
                    portable_command(
                        ui,
                        &label,
                        Command::OpenRecent(index as u8),
                        None,
                        app,
                        picked,
                    );
                }
                ui.separator();
                portable_command(
                    ui,
                    "清除最近使用的文件列表",
                    Command::ClearRecent,
                    None,
                    app,
                    picked,
                );
            });
        }
        MenuNode::System(system) => {
            // AppKit-only services have no portable equivalent.
            let label = match system {
                SystemItem::About => "关于 OpenPhoto",
                SystemItem::Services => "服务",
                SystemItem::HideOthers => "隐藏其他",
                SystemItem::ShowAll => "显示全部",
            };
            if matches!(system, SystemItem::About) {
                ui.label(format!("OpenPhoto {}", env!("CARGO_PKG_VERSION")))
                    .on_hover_text(label);
            } else {
                ui.add_enabled(false, egui::Button::new(label));
            }
        }
    }
}

#[cfg(test)]
mod shared_tests {
    use super::*;

    fn visit(nodes: &[MenuNode], commands: &mut Vec<Command>, labels: &mut Vec<String>) {
        for node in nodes {
            match node {
                MenuNode::Item { label, command, .. } => {
                    labels.push(label.clone());
                    if let Some(command) = command {
                        commands.push(*command);
                    }
                }
                MenuNode::Submenu {
                    label, children, ..
                } => {
                    labels.push(label.clone());
                    visit(children, commands, labels);
                }
                _ => {}
            }
        }
    }

    #[test]
    fn shared_tree_has_all_static_menu_commands_and_chinese_labels() {
        let tree = menu_tree();
        assert_eq!(tree.len(), 12);
        let (mut commands, mut labels) = (Vec::new(), Vec::new());
        visit(&tree, &mut commands, &mut labels);
        assert!(labels.len() > 300);
        assert!(!labels.iter().any(|label| label.contains("Adobe")));
        for command in ALL_COMMANDS {
            // These commands belong to dynamic recent/status/ruler menus.
            if matches!(
                command,
                Command::OpenRecent(_)
                    | Command::ClearRecent
                    | Command::RulerUnits(_)
                    | Command::StatusInfo(_)
            ) {
                continue;
            }
            assert!(commands.contains(command), "missing {command:?}");
        }
        assert_eq!(
            commands
                .iter()
                .filter(|&&command| command == Command::LockLayers)
                .count(),
            1
        );
        assert!(!commands.contains(&Command::ToggleLockAll));
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn portable_dispatch_uses_command_gate_and_leaves_shortcuts_to_egui() {
        let app = AppState::default();
        assert!(portable_enabled(Command::New, &app));
        assert!(!portable_enabled(Command::Save, &app));
        assert!(!portable_enabled(Command::HideApp, &app));
        assert_eq!(portable_shortcut("CmdOrCtrl+Shift+N"), "Ctrl+Shift+N");
        let ctx = egui::Context::default();
        let mut app = app;
        let before = Command::ToggleRulers.checked(&app).unwrap();
        crate::commands::run(Command::ToggleRulers, &ctx, &mut app);
        assert_eq!(Command::ToggleRulers.checked(&app), Some(!before));
        // A disabled selection must not open a document dialog.
        crate::commands::run(Command::ImageSize, &ctx, &mut app);
        assert!(app.image_size_dialog.is_none());
    }
}
