//! The Brushes panel (the brush presets) and the Brush Settings panel (the
//! current painting tool's tip shape, and what the pen's pressure
//! controls).

use egui::Ui;

use crate::state::AppState;

/// Brushes panel: the size, and Photoshop's General Brushes.
pub fn brushes(ui: &mut Ui, app: &mut AppState) {
    let tool = app.tool;
    let Some(options) = app.paint_options(tool) else {
        ui.label("请先选择绘画工具。");
        return;
    };
    ui.horizontal(|ui| {
        ui.label("大小：");
        ui.add(
            egui::Slider::new(
                &mut options.size,
                1.0..=crate::state::PaintOptions::MAX_SIZE,
            )
            .logarithmic(true)
            .max_decimals(0)
            .suffix(" px"),
        );
    });
    ui.separator();
    ui.label("常规画笔");
    for (k, p) in crate::brush_presets::GENERAL.iter().enumerate() {
        if ui
            .selectable_label(options.preset == Some(k), p.display_name())
            .clicked()
        {
            crate::brush_presets::apply(options, k);
        }
    }
}

/// Brush Settings panel: Brush Tip Shape (size, angle, roundness,
/// hardness, spacing), and Shape Dynamics' and Transfer's pen pressure.
pub fn brush_settings(ui: &mut Ui, app: &mut AppState) {
    let tool = app.tool;
    let Some(options) = app.paint_options(tool) else {
        ui.label("请先选择绘画工具以设置画笔。");
        return;
    };
    ui.strong("画笔笔尖形状");
    ui.horizontal(|ui| {
        ui.label("大小：");
        ui.add(
            egui::Slider::new(
                &mut options.size,
                1.0..=crate::state::PaintOptions::MAX_SIZE,
            )
            .logarithmic(true)
            .max_decimals(0)
            .suffix(" px"),
        );
    });
    ui.horizontal(|ui| {
        ui.label("角度：");
        ui.add(
            egui::DragValue::new(&mut options.angle)
                .range(-180.0..=180.0)
                .suffix("°"),
        );
        ui.label("圆度：");
        let mut pct = options.roundness * 100.0;
        if ui
            .add(
                egui::DragValue::new(&mut pct)
                    .range(1.0..=100.0)
                    .max_decimals(0)
                    .suffix("%"),
            )
            .changed()
        {
            options.roundness = pct / 100.0;
        }
    });
    ui.horizontal(|ui| {
        ui.label("硬度：");
        let mut pct = options.hardness * 100.0;
        if ui
            .add(
                egui::Slider::new(&mut pct, 0.0..=100.0)
                    .max_decimals(0)
                    .suffix("%"),
            )
            .changed()
        {
            options.hardness = pct / 100.0;
        }
    });
    ui.horizontal(|ui| {
        ui.label("间距：");
        let mut pct = options.spacing * 100.0;
        if ui
            .add(
                egui::Slider::new(&mut pct, 1.0..=1000.0)
                    .logarithmic(true)
                    .max_decimals(0)
                    .suffix("%"),
            )
            .changed()
        {
            options.spacing = pct / 100.0;
        }
    });
    ui.separator();
    ui.checkbox(&mut options.pressure.size, "形状动态：大小抖动 — 钢笔压力");
    ui.checkbox(
        &mut options.pressure.opacity,
        "传递：不透明度抖动 — 钢笔压力",
    );
    ui.checkbox(&mut options.pressure.flow, "传递：流量抖动 — 钢笔压力");
}
