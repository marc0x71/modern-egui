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

use egui::{Color32, Context, Stroke, Style, Theme};

pub use buttons::{StyledButton, Variant};
pub use palette::Palette;
pub use text::{StyledText, TextColor, TextSize};
pub use ui_ext::{
    InputWidth, PanelVariant, UiButtons, UiInputs, UiLayouts, UiMetrics, UiPanels, UiText,
};

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
///
/// To use your own colors, see [`apply_with`]
pub fn apply(ctx: &Context) {
    apply_with(ctx, Palette::dark(), Palette::light());
}

/// Applies the modern-egui theme to an egui context using custom palettes.
///
/// Works like [`apply`], but installs `dark` and `light` instead of the
/// built-in palettes. Each palette is stored for its theme, so that
/// [`Palette::of`] returns it, and its semantic colors are mapped onto egui's
/// widget visuals. The shared typography and layout metrics are applied as
/// well.
///
/// Call this once when initializing the application. It can be called again
/// later to switch palettes at runtime; doing so reinstalls the metrics,
/// overwriting any change made to them in the meantime.
///
/// # Examples
///
/// ```no_run
/// use eframe::egui::Color32;
/// use modern_egui::theme::{self, Palette};
///
/// # fn configure(ctx: &eframe::egui::Context) {
/// let dark = Palette {
///     accent: Color32::from_rgb(167, 139, 250),
///     accent_hover: Color32::from_rgb(196, 181, 253),
///     accent_active: Color32::from_rgb(139, 92, 246),
///     on_accent: Color32::from_rgb(17, 18, 23),
///     ..Palette::dark()
/// };
///
/// theme::apply_with(ctx, dark, Palette::light());
/// # }
/// ```
///
/// See the `gruvbox` example for two complete custom palettes.
pub fn apply_with(ctx: &Context, dark: Palette, light: Palette) {
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

/// Builds an opaque color from a `0xRRGGBB` literal.
///
/// This is a `const fn`, so it can be used to define a [`Palette`] as a
/// constant, with colors written the way design tools and color schemes
/// usually list them.
///
/// Only the lowest 24 bits are read: any higher bits are ignored, so the
/// result is always fully opaque.
///
/// # Examples
///
/// ```
/// use egui::Color32;
/// use modern_egui::theme;
///
/// const BG: Color32 = theme::rgb_hex(0x1d2021);
///
/// assert_eq!(BG, Color32::from_rgb(29, 32, 33));
/// ```
pub const fn rgb_hex(rgb: u32) -> Color32 {
    Color32::from_rgb((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8)
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
