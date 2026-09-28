use egui::{self, Button, Color32, IntoAtoms, Stroke, Widget};

use crate::theme::{Palette, metrics::RADIUS_SM};

/// Visual variant of a [`StyledButton`].
///
/// Each variant maps to a set of colors from the active [`Palette`], so the
/// button follows the current dark or light theme automatically.
pub enum Variant {
    /// Primary default actions
    Primary,
    /// Ordinary actions. Filled, but quiet.
    Secondary,
    /// Tertiary actions: no fill until hovered. Good in toolbars and lists.
    Ghost,
    /// Destructive actions.
    Danger,
}

/// A themed wrapper around [`egui::Button`].
///
/// `StyledButton` applies the colors of a [`Variant`] when it is added to a
/// [`egui::Ui`], while exposing the usual [`egui::Button`] builder options.
/// Buttons use [`RADIUS_SM`] corners by default.
///
/// The [`UiButtons`](crate::theme::UiButtons) helpers are shortcuts for the
/// most common case; use this widget directly when you need to customize the
/// button.
///
/// ```no_run
/// use eframe::egui;
/// use modern_egui::theme::buttons::StyledButton;
///
/// # fn show(ui: &mut egui::Ui) {
/// let response = ui.add(StyledButton::primary("Save").min_size(egui::vec2(120.0, 0.0)));
/// if response.clicked() {
///     // ...
/// }
/// # }
/// ```
pub struct StyledButton<'a> {
    inner: Button<'a>,
    variant: Variant,
}

impl<'a> StyledButton<'a> {
    /// Creates a button with the given content and variant.
    ///
    /// Prefer the variant-specific constructors ([`primary`](Self::primary),
    /// [`secondary`](Self::secondary), ...) unless the variant is chosen at runtime.
    pub fn new(atoms: impl IntoAtoms<'a>, variant: Variant) -> Self {
        let inner = Button::new(atoms).corner_radius(RADIUS_SM);
        Self { variant, inner }
    }

    /// Creates a [`Variant::Primary`] button, for the main action of a view.
    pub fn primary(atoms: impl IntoAtoms<'a>) -> Self {
        Self::new(atoms, Variant::Primary)
    }

    /// Creates a [`Variant::Secondary`] button, for ordinary actions.
    pub fn secondary(atoms: impl IntoAtoms<'a>) -> Self {
        Self::new(atoms, Variant::Secondary)
    }

    /// Creates a [`Variant::Ghost`] button, for low-emphasis actions.
    pub fn ghost(atoms: impl IntoAtoms<'a>) -> Self {
        Self::new(atoms, Variant::Ghost)
    }

    /// Creates a [`Variant::Danger`] button, for destructive actions.
    pub fn danger(atoms: impl IntoAtoms<'a>) -> Self {
        Self::new(atoms, Variant::Danger)
    }

    /// Sets the minimum size of the button.
    ///
    /// See [`egui::Button::min_size`].
    pub fn min_size(mut self, size: egui::Vec2) -> Self {
        self.inner = self.inner.min_size(size);
        self
    }

    /// Makes the button smaller, as used for inline buttons in text.
    ///
    /// See [`egui::Button::small`].
    pub fn small(mut self) -> Self {
        self.inner = self.inner.small();
        self
    }

    /// Overrides the default [`RADIUS_SM`] corner radius.
    ///
    /// See [`egui::Button::corner_radius`].
    pub fn corner_radius(mut self, radius: impl Into<egui::CornerRadius>) -> Self {
        self.inner = self.inner.corner_radius(radius);
        self
    }

    /// Sets which interactions the button responds to.
    ///
    /// See [`egui::Button::sense`].
    pub fn sense(mut self, sense: egui::Sense) -> Self {
        self.inner = self.inner.sense(sense);
        self
    }

    /// Draws the button in its selected state, e.g. for toggles.
    ///
    /// See [`egui::Button::selected`].
    pub fn selected(mut self, selected: bool) -> Self {
        self.inner = self.inner.selected(selected);
        self
    }

    /// Shows a keyboard shortcut hint on the right side of the button.
    ///
    /// See [`egui::Button::shortcut_text`].
    pub fn shortcut_text(mut self, text: impl IntoAtoms<'a>) -> Self {
        self.inner = self.inner.shortcut_text(text);
        self
    }

    /// Wraps the button text onto multiple lines if it does not fit.
    ///
    /// See [`egui::Button::wrap`].
    pub fn wrap(mut self) -> Self {
        self.inner = self.inner.wrap();
        self
    }

    /// Truncates the button text with an ellipsis if it does not fit.
    ///
    /// See [`egui::Button::truncate`].
    pub fn truncate(mut self) -> Self {
        self.inner = self.inner.truncate();
        self
    }

    /// Sets the gap between the button's content items, such as an image and
    /// its text.
    ///
    /// See [`egui::Button::gap`].
    pub fn gap(mut self, gap: f32) -> Self {
        self.inner = self.inner.gap(gap);
        self
    }
}

impl Widget for StyledButton<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.scope(|ui| {
            self.variant.apply(ui);
            self.inner.ui(ui)
        })
        .inner
    }
}

impl Variant {
    fn apply(&self, ui: &mut egui::Ui) {
        let p = Palette::of(ui.ctx());
        let widgets = &mut ui.style_mut().visuals.widgets;

        match self {
            Variant::Primary => {
                widgets.inactive.bg_fill = p.accent;
                widgets.inactive.weak_bg_fill = p.accent;
                widgets.inactive.bg_stroke = Stroke::new(1.0, p.accent);
                widgets.inactive.fg_stroke.color = p.on_accent;

                widgets.hovered.bg_fill = p.accent_hover;
                widgets.hovered.weak_bg_fill = p.accent_hover;
                widgets.hovered.bg_stroke = Stroke::new(1.0, p.accent_hover);
                widgets.hovered.fg_stroke.color = p.on_accent;

                widgets.active.bg_fill = p.accent_active;
                widgets.active.weak_bg_fill = p.accent_active;
                widgets.active.bg_stroke = Stroke::new(1.0, p.accent_active);
                widgets.active.fg_stroke.color = p.on_accent;
            }
            Variant::Secondary => {}
            Variant::Ghost => {
                widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
                // widgets.inactive.bg_fill = Color32::TRANSPARENT;
                widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::TRANSPARENT);
                widgets.inactive.fg_stroke.color = p.text;

                widgets.hovered.weak_bg_fill = p.surface_alt;
                widgets.hovered.bg_fill = p.surface_alt;
                widgets.hovered.bg_stroke = Stroke::new(1.0, p.border);
                widgets.hovered.fg_stroke.color = p.text_strong;

                widgets.active.weak_bg_fill = p.elevated;
                widgets.active.bg_fill = p.elevated;
                widgets.active.bg_stroke = Stroke::new(1.0, p.border_strong);
                widgets.active.fg_stroke.color = p.text_strong;
            }
            Variant::Danger => {
                widgets.inactive.bg_fill = p.danger;
                widgets.inactive.weak_bg_fill = p.danger;
                widgets.inactive.bg_stroke = Stroke::new(1.0, p.danger);
                widgets.inactive.fg_stroke.color = p.on_danger;

                widgets.hovered.bg_fill = p.danger_hover;
                widgets.hovered.weak_bg_fill = p.danger_hover;
                widgets.hovered.bg_stroke = Stroke::new(1.0, p.danger_hover);
                widgets.hovered.fg_stroke.color = p.on_danger;

                widgets.active.bg_fill = p.danger_active;
                widgets.active.weak_bg_fill = p.danger_active;
                widgets.active.bg_stroke = Stroke::new(1.0, p.danger_active);
                widgets.active.fg_stroke.color = p.on_danger;
            }
        }
    }
}
