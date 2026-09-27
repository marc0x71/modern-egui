pub mod metrics;
pub mod palette;
pub mod ui_ext;

use eframe::egui::{Context, Stroke, Style, Theme};

pub use palette::Palette;
pub use ui_ext::{UiMetrics, UiText};

pub fn apply(ctx: &Context) {
    let dark = Palette::dark();
    let light = Palette::light();

    dark.store(ctx, Theme::Dark);
    light.store(ctx, Theme::Light);

    ctx.style_mut_of(Theme::Dark, |s| {
        colors(s, &dark);
    });

    ctx.style_mut_of(Theme::Light, |s| {
        colors(s, &light);
    });

    ctx.all_styles_mut(metrics::apply);
}

fn colors(s: &mut Style, p: &Palette) {
    let w = &mut s.visuals.widgets;

    // Separators, frame outlines, non-interactive text.
    w.noninteractive.bg_fill = p.surface;
    w.noninteractive.weak_bg_fill = p.surface;
    w.noninteractive.bg_stroke = Stroke::new(1.0, p.border);
    w.noninteractive.fg_stroke = Stroke::new(1.0, p.text);

    // Buttons, combo boxes and checkboxes at rest.
    w.inactive.weak_bg_fill = p.surface_alt;
    w.inactive.bg_fill = p.surface_alt;
    w.inactive.bg_stroke = Stroke::new(1.0, p.border);
    w.inactive.fg_stroke = Stroke::new(1.0, p.text);

    w.hovered.weak_bg_fill = p.surface_alt.lerp_to_gamma(p.text, 0.08);
    w.hovered.bg_fill = p.surface_alt.lerp_to_gamma(p.text, 0.08);
    w.hovered.bg_stroke = Stroke::new(1.0, p.border_strong);
    w.hovered.fg_stroke = Stroke::new(1.0, p.text);

    // Pressed state.
    w.active.weak_bg_fill = p.surface_alt.lerp_to_gamma(p.text, 0.16);
    w.active.bg_fill = p.surface_alt.lerp_to_gamma(p.text, 0.16);
    w.active.bg_stroke = Stroke::new(1.0, p.accent);
    w.active.fg_stroke = Stroke::new(1.0, p.text_strong);

    w.open.weak_bg_fill = p.surface_alt;
    w.open.bg_fill = p.surface_alt;
    w.open.bg_stroke = Stroke::new(1.0, p.border_strong);
    w.open.fg_stroke = Stroke::new(1.0, p.text);

    // Surfaces.
    s.visuals.panel_fill = p.bg;
    s.visuals.window_fill = p.elevated;
    s.visuals.window_stroke = Stroke::new(1.0, p.border);
    s.visuals.extreme_bg_color = p.bg; // background of text fields
    s.visuals.faint_bg_color = p.surface_alt.gamma_multiply(0.5); // striped rows
    s.visuals.code_bg_color = p.surface_alt;

    // Selection, links, semantic colours.
    s.visuals.selection.bg_fill = p.accent;
    s.visuals.selection.stroke = Stroke::new(1.0, p.on_accent);
    s.visuals.hyperlink_color = p.accent;
    s.visuals.error_fg_color = p.danger;
    s.visuals.warn_fg_color = p.warning;
    s.visuals.text_cursor.stroke = Stroke::new(2.0, p.accent);
}
