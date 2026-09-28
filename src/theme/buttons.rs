use std::ops::Deref;

use eframe::egui::{self, Button, Color32, IntoAtoms, Stroke, Widget};

use crate::theme::{Palette, metrics::RADIUS_SM};

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

pub struct StyledButton<'a> {
    inner: Button<'a>,
    variant: Variant,
}

impl<'a> StyledButton<'a> {
    pub fn new(atoms: impl IntoAtoms<'a>, variant: Variant) -> Self {
        let inner = Button::new(atoms).corner_radius(RADIUS_SM);
        Self { variant, inner }
    }

    pub fn primary(atoms: impl IntoAtoms<'a>) -> Self {
        Self::new(atoms, Variant::Primary)
    }

    pub fn secondary(atoms: impl IntoAtoms<'a>) -> Self {
        Self::new(atoms, Variant::Secondary)
    }

    pub fn ghost(atoms: impl IntoAtoms<'a>) -> Self {
        Self::new(atoms, Variant::Ghost)
    }

    pub fn danger(atoms: impl IntoAtoms<'a>) -> Self {
        Self::new(atoms, Variant::Danger)
    }

    pub fn min_size(mut self, size: egui::Vec2) -> Self {
        self.inner = self.inner.min_size(size);
        self
    }

    pub fn small(mut self) -> Self {
        self.inner = self.inner.small();
        self
    }

    pub fn corner_radius(mut self, radius: impl Into<egui::CornerRadius>) -> Self {
        self.inner = self.inner.corner_radius(radius);
        self
    }

    pub fn sense(mut self, sense: egui::Sense) -> Self {
        self.inner = self.inner.sense(sense);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.inner = self.inner.selected(selected);
        self
    }

    pub fn shortcut_text(mut self, text: impl IntoAtoms<'a>) -> Self {
        self.inner = self.inner.shortcut_text(text);
        self
    }

    pub fn wrap(mut self) -> Self {
        self.inner = self.inner.wrap();
        self
    }

    pub fn truncate(mut self) -> Self {
        self.inner = self.inner.truncate();
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.inner = self.inner.gap(gap);
        self
    }
}

impl<'a> Deref for StyledButton<'a> {
    type Target = Button<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Widget for StyledButton<'_> {
    fn ui(self, ui: &mut eframe::egui::Ui) -> egui::Response {
        ui.scope(|ui| {
            self.variant.apply(ui);
            self.inner.ui(ui)
        })
        .inner
    }
}

impl Variant {
    fn apply(&self, ui: &mut eframe::egui::Ui) {
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
