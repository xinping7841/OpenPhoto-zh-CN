//! Tool definitions: every Photoshop 2026 tool, how they are grouped in the
//! toolbar, their names and single-key shortcuts.

/// Every tool in Photoshop's toolbar, including the ones hidden in groups.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tool {
    Move,
    Artboard,
    RectangularMarquee,
    EllipticalMarquee,
    SingleRowMarquee,
    SingleColumnMarquee,
    Lasso,
    PolygonalLasso,
    MagneticLasso,
    /// Photoshop 2026's Selection Brush (paints a selection).
    SelectionBrush,
    ObjectSelection,
    QuickSelection,
    MagicWand,
    Crop,
    PerspectiveCrop,
    Slice,
    SliceSelect,
    Frame,
    Eyedropper,
    ColorSampler,
    Ruler,
    Note,
    Count,
    SpotHealingBrush,
    Remove,
    HealingBrush,
    Patch,
    ContentAwareMove,
    RedEye,
    Brush,
    Pencil,
    ColorReplacement,
    MixerBrush,
    CloneStamp,
    PatternStamp,
    HistoryBrush,
    ArtHistoryBrush,
    Eraser,
    BackgroundEraser,
    MagicEraser,
    Gradient,
    PaintBucket,
    Blur,
    Sharpen,
    Smudge,
    /// Photoshop 2025+'s Adjustment Brush (paints an adjustment).
    AdjustmentBrush,
    Dodge,
    Burn,
    Sponge,
    Pen,
    FreeformPen,
    CurvaturePen,
    AddAnchorPoint,
    DeleteAnchorPoint,
    ConvertPoint,
    HorizontalType,
    VerticalType,
    VerticalTypeMask,
    HorizontalTypeMask,
    PathSelection,
    DirectSelection,
    Rectangle,
    Ellipse,
    Triangle,
    Polygon,
    Line,
    CustomShape,
    Hand,
    RotateView,
    Zoom,
}

use Tool::*;

/// The toolbar's slots from top to bottom; each slot is a group of tools that
/// share a button, in the order of Photoshop's flyout menu.
pub const TOOLBAR: &[&[Tool]] = &[
    &[Move, Artboard],
    &[
        RectangularMarquee,
        EllipticalMarquee,
        SingleRowMarquee,
        SingleColumnMarquee,
    ],
    // Photoshop 2026 lists the Selection Brush first, though the Lasso is
    // the one the slot shows at first
    &[SelectionBrush, Lasso, PolygonalLasso, MagneticLasso],
    &[ObjectSelection, QuickSelection, MagicWand],
    &[Crop, PerspectiveCrop, Slice, SliceSelect],
    &[Frame],
    &[Eyedropper, ColorSampler, Ruler, Note, Count],
    &[
        SpotHealingBrush,
        Remove,
        HealingBrush,
        Patch,
        ContentAwareMove,
        RedEye,
    ],
    &[Brush, Pencil, ColorReplacement, MixerBrush],
    &[CloneStamp, PatternStamp],
    &[HistoryBrush, ArtHistoryBrush],
    &[Eraser, BackgroundEraser, MagicEraser],
    &[Gradient, PaintBucket],
    &[Blur, Sharpen, Smudge],
    &[AdjustmentBrush],
    &[Dodge, Burn, Sponge],
    &[
        Pen,
        FreeformPen,
        CurvaturePen,
        AddAnchorPoint,
        DeleteAnchorPoint,
        ConvertPoint,
    ],
    &[
        HorizontalType,
        VerticalType,
        VerticalTypeMask,
        HorizontalTypeMask,
    ],
    &[PathSelection, DirectSelection],
    &[Rectangle, Ellipse, Triangle, Polygon, Line, CustomShape],
    &[Hand, RotateView],
    &[Zoom],
];

/// The tool a toolbar slot shows before another is picked: the group's
/// first, except the Lasso in its group (as in Photoshop 2026).
pub fn default_in_group(group: &[Tool]) -> Tool {
    if group.contains(&Lasso) {
        Lasso
    } else {
        group[0]
    }
}

impl Tool {
    pub fn name(self) -> &'static str {
        match self {
            Move => "移动工具",
            Artboard => "画板工具",
            RectangularMarquee => "矩形选框工具",
            EllipticalMarquee => "椭圆选框工具",
            SingleRowMarquee => "单行选框工具",
            SingleColumnMarquee => "单列选框工具",
            Lasso => "套索工具",
            PolygonalLasso => "多边形套索工具",
            MagneticLasso => "磁性套索工具",
            SelectionBrush => "选择画笔工具",
            ObjectSelection => "对象选择工具",
            QuickSelection => "快速选择工具",
            MagicWand => "魔棒工具",
            Crop => "裁剪工具",
            PerspectiveCrop => "透视裁剪工具",
            Slice => "切片工具",
            SliceSelect => "切片选择工具",
            Frame => "框架工具",
            Eyedropper => "吸管工具",
            ColorSampler => "颜色取样器工具",
            Ruler => "标尺工具",
            Note => "注释工具",
            Count => "计数工具",
            SpotHealingBrush => "污点修复画笔工具",
            Remove => "移除工具",
            HealingBrush => "修复画笔工具",
            Patch => "修补工具",
            ContentAwareMove => "内容识别移动工具",
            RedEye => "红眼工具",
            Brush => "画笔工具",
            Pencil => "铅笔工具",
            ColorReplacement => "颜色替换工具",
            MixerBrush => "混合器画笔工具",
            CloneStamp => "仿制图章工具",
            PatternStamp => "图案图章工具",
            HistoryBrush => "历史记录画笔工具",
            ArtHistoryBrush => "历史记录艺术画笔工具",
            Eraser => "橡皮擦工具",
            BackgroundEraser => "背景橡皮擦工具",
            MagicEraser => "魔术橡皮擦工具",
            Gradient => "渐变工具",
            PaintBucket => "油漆桶工具",
            Blur => "模糊工具",
            Sharpen => "锐化工具",
            Smudge => "涂抹工具",
            AdjustmentBrush => "调整画笔工具",
            Dodge => "减淡工具",
            Burn => "加深工具",
            Sponge => "海绵工具",
            Pen => "钢笔工具",
            FreeformPen => "自由钢笔工具",
            CurvaturePen => "弯度钢笔工具",
            AddAnchorPoint => "添加锚点工具",
            DeleteAnchorPoint => "删除锚点工具",
            ConvertPoint => "转换点工具",
            HorizontalType => "横排文字工具",
            VerticalType => "直排文字工具",
            VerticalTypeMask => "直排文字蒙版工具",
            HorizontalTypeMask => "横排文字蒙版工具",
            PathSelection => "路径选择工具",
            DirectSelection => "直接选择工具",
            Rectangle => "矩形工具",
            Ellipse => "椭圆工具",
            Triangle => "三角形工具",
            Polygon => "多边形工具",
            Line => "直线工具",
            CustomShape => "自定形状工具",
            Hand => "抓手工具",
            RotateView => "旋转视图工具",
            Zoom => "缩放工具",
        }
    }

    /// Single-key shortcut, as in Photoshop. Tools in a group share the
    /// group's key (Shift+key cycles through them); a few have none.
    pub fn shortcut(self) -> Option<char> {
        Some(match self {
            Move | Artboard => 'V',
            RectangularMarquee | EllipticalMarquee => 'M',
            SelectionBrush | Lasso | PolygonalLasso | MagneticLasso => 'L',
            ObjectSelection | QuickSelection | MagicWand => 'W',
            Crop | PerspectiveCrop | Slice | SliceSelect => 'C',
            Frame => 'K',
            Eyedropper | ColorSampler | Ruler | Note | Count => 'I',
            SpotHealingBrush | Remove | HealingBrush | Patch | ContentAwareMove | RedEye => 'J',
            Brush | Pencil | ColorReplacement | MixerBrush => 'B',
            CloneStamp | PatternStamp => 'S',
            HistoryBrush | ArtHistoryBrush => 'Y',
            Eraser | BackgroundEraser | MagicEraser => 'E',
            Gradient | PaintBucket => 'G',
            Dodge | Burn | Sponge => 'O',
            Pen | FreeformPen | CurvaturePen => 'P',
            HorizontalType | VerticalType | VerticalTypeMask | HorizontalTypeMask => 'T',
            PathSelection | DirectSelection => 'A',
            Rectangle | Ellipse | Triangle | Polygon | Line | CustomShape => 'U',
            Hand => 'H',
            RotateView => 'R',
            Zoom => 'Z',
            SingleRowMarquee | SingleColumnMarquee | Blur | Sharpen | Smudge | AddAnchorPoint
            | DeleteAnchorPoint | ConvertPoint | AdjustmentBrush => return None,
        })
    }

    /// Index of the toolbar slot this tool lives in.
    pub fn slot(self) -> usize {
        TOOLBAR
            .iter()
            .position(|group| group.contains(&self))
            .expect("every tool is in the toolbar")
    }
}

/// The tool a key press selects. `current` is the tool shown in each slot
/// (Photoshop remembers the last tool used per group); with `shift`, the key
/// cycles to the next tool in the group that has the same shortcut.
pub fn tool_for_key(
    key: char,
    shift: bool,
    active: Tool,
    current: impl Fn(usize) -> Tool,
) -> Option<Tool> {
    let key = key.to_ascii_uppercase();
    // Rotate View shares the Hand slot but has its own key
    let slot = TOOLBAR
        .iter()
        .position(|group| group.iter().any(|t| t.shortcut() == Some(key)))?;
    let group = TOOLBAR[slot];
    let shown = current(slot);
    if group.iter().filter(|t| t.shortcut() == Some(key)).count() == 1 {
        return group.iter().copied().find(|t| t.shortcut() == Some(key));
    }
    if !shift || active.slot() != slot || shown.shortcut() != Some(key) {
        return Some(if shown.shortcut() == Some(key) {
            shown
        } else {
            group.iter().copied().find(|t| t.shortcut() == Some(key))?
        });
    }
    let i = group.iter().position(|t| *t == shown)?;
    (1..=group.len())
        .map(|k| group[(i + k) % group.len()])
        .find(|t| t.shortcut() == Some(key))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn first(slot: usize) -> Tool {
        TOOLBAR[slot][0]
    }

    #[test]
    fn every_tool_appears_once() {
        let all: Vec<Tool> = TOOLBAR.iter().flat_map(|g| g.iter().copied()).collect();
        let mut dedup = all.clone();
        dedup.sort_by_key(|t| *t as usize);
        dedup.dedup();
        assert_eq!(all.len(), dedup.len());
        assert_eq!(all.len(), 70);
        assert_eq!(TOOLBAR.len(), 22);
        assert!(all.iter().all(|tool| tool.name().ends_with("工具")));
        assert_eq!(Tool::Brush.name(), "画笔工具");
    }

    #[test]
    fn keys_select_the_group_and_shift_cycles() {
        assert_eq!(
            tool_for_key('m', false, Move, first),
            Some(RectangularMarquee)
        );
        // Shift+M from the rectangular marquee goes to the elliptical one
        assert_eq!(
            tool_for_key('M', true, RectangularMarquee, first),
            Some(EllipticalMarquee)
        );
        // ...and back, skipping the single row/column tools (no shortcut)
        let shown = |slot: usize| {
            if slot == 1 {
                EllipticalMarquee
            } else {
                first(slot)
            }
        };
        assert_eq!(
            tool_for_key('M', true, EllipticalMarquee, shown),
            Some(RectangularMarquee)
        );
        // Without Shift the group's current tool is chosen
        assert_eq!(
            tool_for_key('m', false, Move, shown),
            Some(EllipticalMarquee)
        );
        // Shift+key from another group just selects the group
        assert_eq!(
            tool_for_key('M', true, Brush, first),
            Some(RectangularMarquee)
        );
        // Keys unique within their slot
        assert_eq!(tool_for_key('r', false, Move, first), Some(RotateView));
        assert_eq!(tool_for_key('h', true, RotateView, first), Some(Hand));
        assert_eq!(tool_for_key('q', false, Move, first), None);
    }
}
