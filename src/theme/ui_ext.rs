use eframe::egui::{self, Color32, Response, RichText, Stroke, Ui};

use crate::theme::{Palette, metrics::*};

pub trait UiMetrics {
    // Raw spacing scale
    fn space1(&mut self);
    fn space2(&mut self);
    fn space3(&mut self);
    fn space4(&mut self);
    fn space5(&mut self);
    fn space6(&mut self);

    // Semantic spacing
    fn space_inline(&mut self);
    fn space_group(&mut self);
    fn space_section(&mut self);
    fn space_page(&mut self);
}

impl UiMetrics for Ui {
    fn space1(&mut self) {
        self.add_space(SPACE_1);
    }
    fn space2(&mut self) {
        self.add_space(SPACE_2);
    }
    fn space3(&mut self) {
        self.add_space(SPACE_3);
    }
    fn space4(&mut self) {
        self.add_space(SPACE_4);
    }
    fn space5(&mut self) {
        self.add_space(SPACE_5);
    }
    fn space6(&mut self) {
        self.add_space(SPACE_6);
    }
    fn space_inline(&mut self) {
        self.add_space(GAP_INLINE);
    }
    fn space_group(&mut self) {
        self.add_space(GAP_GROUP);
    }
    fn space_section(&mut self) {
        self.add_space(GAP_SECTION);
    }
    fn space_page(&mut self) {
        self.add_space(GAP_PAGE);
    }
}

pub trait UiText {
    fn section_title(&mut self, text: impl Into<String>);
    fn card_title(&mut self, text: impl Into<String>);
    fn muted_label(&mut self, text: impl Into<String>);
    fn metadata_label(&mut self, text: impl Into<String>);
    fn strong_label(&mut self, text: impl Into<String>);
}

impl UiText for Ui {
    fn section_title(&mut self, text: impl Into<String>) {
        self.label(egui::RichText::new(text.into()).size(FONT_LG).strong());
    }

    fn card_title(&mut self, text: impl Into<String>) {
        self.label(egui::RichText::new(text.into()).size(FONT_CT).strong());
    }

    fn muted_label(&mut self, text: impl Into<String>) {
        self.label(egui::RichText::new(text.into()).size(FONT_SM).weak());
    }

    fn metadata_label(&mut self, text: impl Into<String>) {
        self.label(egui::RichText::new(text.into()).size(FONT_XS).weak());
    }

    fn strong_label(&mut self, text: impl Into<String>) {
        let p = Palette::of(self.ctx());
        self.label(
            RichText::new(text.into())
                .size(FONT_MD)
                .strong()
                .color(p.text_strong),
        );
    }
}

pub trait UiInputs {
    fn text_input(&mut self, text: &mut String) -> Response;
    fn text_input_hint(&mut self, text: &mut String, hint: impl Into<egui::WidgetText>)
    -> Response;
}
impl UiInputs for Ui {
    fn text_input(&mut self, text: &mut String) -> Response {
        self.add_sized(
            [INPUT_WIDTH, INPUT_HEIGHT],
            egui::TextEdit::singleline(text).vertical_align(egui::Align::Center),
        )
    }

    fn text_input_hint(
        &mut self,
        text: &mut String,
        hint: impl Into<egui::WidgetText>,
    ) -> Response {
        self.add_sized(
            [INPUT_WIDTH, INPUT_HEIGHT],
            egui::TextEdit::singleline(text)
                .vertical_align(egui::Align::Center)
                .hint_text(hint),
        )
    }
}

pub trait UiButtons {
    fn primary_button(&mut self, text: impl Into<egui::WidgetText>) -> Response;
    fn danger_button(&mut self, text: impl Into<egui::WidgetText>) -> Response;
    fn ghost_button(&mut self, text: impl Into<egui::WidgetText>) -> Response;
}

impl UiButtons for Ui {
    fn primary_button(&mut self, text: impl Into<egui::WidgetText>) -> Response {
        let p = Palette::of(self.ctx());

        self.scope(|ui| {
            let widgets = &mut ui.style_mut().visuals.widgets;

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

            ui.button(text)
        })
        .inner
    }

    fn danger_button(&mut self, text: impl Into<egui::WidgetText>) -> Response {
        let p = Palette::of(self.ctx());

        self.scope(|ui| {
            let widgets = &mut ui.style_mut().visuals.widgets;

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

            ui.button(text)
        })
        .inner
    }

    fn ghost_button(&mut self, text: impl Into<egui::WidgetText>) -> Response {
        let p = Palette::of(self.ctx());

        self.scope(|ui| {
            let widgets = &mut ui.style_mut().visuals.widgets;

            widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
            widgets.inactive.bg_fill = Color32::TRANSPARENT;
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

            ui.button(text)
        })
        .inner
    }
}
