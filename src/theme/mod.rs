//! The modern-egui theme: palette, design tokens and `Ui` helpers.
//!
//! Call [`apply`] once to install the theme, then use the extension traits
//! re-exported here ([`UiText`], [`UiMetrics`], [`UiButtons`], [`UiInputs`])
//! to build the interface.

pub mod buttons;
pub mod metrics;
pub mod palette;
pub mod text;
pub mod ui_ext;

use egui::{Context, Stroke, Style, Theme};

pub use buttons::{StyledButton, Variant};
pub use palette::Palette;
pub use text::{StyledText, TextColor, TextSize};
pub use ui_ext::{UiButtons, UiInputs, UiMetrics, UiPanels, UiText};

/// Applies the modern-egui theme to an egui context.
///
/// This installs the built-in dark and light palettes, maps their semantic
/// colors onto egui's widget visuals, and applies the shared typography and
/// layout metrics.
///
/// Call this once when initializing the application:
///
/// ```no_run
/// use modern_egui::theme;
///
/// # fn configure(ctx: &eframe::egui::Context) {
/// theme::apply(ctx);
/// # }
/// ```
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

#[cfg(test)]
mod tests {
    use egui;

    use super::*;

    #[test]
    fn apply_installs_dark_palette() {
        let ctx = Context::default();

        apply(&ctx);

        ctx.set_theme(egui::ThemePreference::Dark);

        assert_eq!(Palette::of(&ctx), Palette::dark(),);
    }

    #[test]
    fn apply_installs_light_palette() {
        let ctx = Context::default();

        apply(&ctx);

        ctx.set_theme(egui::ThemePreference::Light);

        assert_eq!(Palette::of(&ctx), Palette::light(),);
    }
}
