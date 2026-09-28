use eframe::egui::{self, IntoAtoms, Response, RichText, Ui};

use crate::theme::{Palette, buttons::StyledButton, metrics::*};

/// Spacing helpers for [`egui::Ui`] based on the theme's metric scale.
pub trait UiMetrics {
    /// Adds the smallest spacing step from the raw spacing scale.
    fn space1(&mut self);

    /// Adds the second spacing step from the raw spacing scale.
    fn space2(&mut self);

    /// Adds the third spacing step from the raw spacing scale.
    fn space3(&mut self);

    /// Adds the fourth spacing step from the raw spacing scale.
    fn space4(&mut self);

    /// Adds the fifth spacing step from the raw spacing scale.
    fn space5(&mut self);

    /// Adds the largest spacing step from the raw spacing scale.
    fn space6(&mut self);

    /// Adds a small semantic gap between closely related elements.
    fn space_inline(&mut self);

    /// Adds spacing between elements belonging to the same logical group.
    fn space_group(&mut self);

    /// Adds spacing between distinct sections of a view.
    fn space_section(&mut self);

    /// Adds spacing between major page-level blocks.
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

/// Typography helpers for applying common semantic text roles to [`egui::Ui`].
pub trait UiText {
    /// Adds a prominent section title using the theme's section typography.
    fn section_title(&mut self, text: impl Into<String>);

    /// Adds a card or panel title using the theme's card-title typography.
    fn card_title(&mut self, text: impl Into<String>);

    /// Adds secondary text using the theme's muted text styling.
    fn muted_label(&mut self, text: impl Into<String>);

    /// Adds low-emphasis metadata using the smallest text size.
    fn metadata_label(&mut self, text: impl Into<String>);

    /// Adds emphasized body text using the theme's strong text color.
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

/// Convenience helpers for themed single-line text inputs.
pub trait UiInputs {
    /// Adds a single-line text input using the theme's standard input size.
    fn text_input(&mut self, text: &mut String) -> Response;

    /// Adds a single-line text input with placeholder text using the theme's
    /// standard input size.
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

/// Convenience helpers for semantic button variants.
pub trait UiButtons {
    /// Adds a primary action button using the current theme's accent colors.
    ///
    /// The button automatically uses the palette's normal, hovered, and active
    /// accent states.
    fn primary_button<'a>(&mut self, text: impl IntoAtoms<'a>) -> egui::Response;

    /// Adds a secondary action button with filled but subdued styling.
    ///
    /// Secondary buttons are intended for ordinary actions that should have less
    /// visual emphasis than primary actions.
    fn secondary_button<'a>(&mut self, text: impl IntoAtoms<'a>) -> egui::Response;

    /// Adds a destructive action button using the current theme's danger colors.
    ///
    /// The button automatically uses the palette's normal, hovered, and active
    /// danger states.
    fn danger_button<'a>(&mut self, text: impl IntoAtoms<'a>) -> egui::Response;

    /// Adds a low-emphasis button with a transparent background at rest.
    ///
    /// The button uses themed surface and border colors while hovered or active.
    fn ghost_button<'a>(&mut self, text: impl IntoAtoms<'a>) -> egui::Response;
}

impl UiButtons for Ui {
    fn primary_button<'a>(&mut self, text: impl IntoAtoms<'a>) -> egui::Response {
        self.add(StyledButton::primary(text))
    }

    fn secondary_button<'a>(&mut self, text: impl IntoAtoms<'a>) -> egui::Response {
        self.add(StyledButton::secondary(text))
    }

    fn danger_button<'a>(&mut self, text: impl IntoAtoms<'a>) -> egui::Response {
        self.add(StyledButton::danger(text))
    }

    fn ghost_button<'a>(&mut self, text: impl IntoAtoms<'a>) -> egui::Response {
        self.add(StyledButton::ghost(text))
    }
}
