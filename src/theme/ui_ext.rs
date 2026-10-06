//! Extension traits for [`egui::Ui`].
//!
//! Bring the traits into scope (they are re-exported from
//! [`theme`](crate::theme)) to use semantic helpers on any `Ui`:
//! [`UiText`] for typography, [`UiMetrics`] for spacing, [`UiButtons`] for
//! button variants, [`UiInputs`] for text fields and [`UiPanels`] for cards.

use egui::{self, Color32, InnerResponse, IntoAtoms, Response, Stroke, Ui};

use crate::theme::{
    Palette,
    buttons::StyledButton,
    metrics::{self, *},
    text::{StyledText, TextColor, TextSize},
};

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
///
/// These methods are semantic presets built on top of [`StyledText`].
/// Use [`text`](Self::text) or [`text_colored`](Self::text_colored) for a
/// different combination of size and color, and `StyledText` directly when
/// more control is required.
pub trait UiText {
    /// Adds the application title using the largest text size.
    fn app_title(&mut self, text: impl Into<String>) -> egui::Response;

    /// Adds a page title using the theme's heading typography.
    fn page_title(&mut self, text: impl Into<String>) -> egui::Response;

    /// Adds a prominent section title using the theme's section typography.
    fn section_title(&mut self, text: impl Into<String>) -> egui::Response;

    /// Adds a card or panel title using the theme's card-title typography.
    fn card_title(&mut self, text: impl Into<String>) -> egui::Response;

    /// Adds secondary text using the theme's muted text styling.
    fn muted_label(&mut self, text: impl Into<String>) -> egui::Response;

    /// Adds low-emphasis metadata using the smallest text size.
    fn metadata_label(&mut self, text: impl Into<String>) -> egui::Response;

    /// Adds emphasized body text using the theme's strong text color.
    fn strong_label(&mut self, text: impl Into<String>) -> egui::Response;

    /// Adds text with the given size and the default body text color.
    fn text(&mut self, text: impl Into<String>, size: TextSize) -> Response;

    /// Adds text with the given size and semantic color.
    fn text_colored(
        &mut self,
        text: impl Into<String>,
        size: TextSize,
        color: TextColor,
    ) -> Response;
}

impl UiText for egui::Ui {
    fn app_title(&mut self, text: impl Into<String>) -> egui::Response {
        self.add(
            StyledText::new(text)
                .size(TextSize::Xxl)
                .color(TextColor::Strong),
        )
    }

    fn page_title(&mut self, text: impl Into<String>) -> egui::Response {
        self.add(
            StyledText::new(text)
                .size(TextSize::Xl)
                .color(TextColor::Strong),
        )
    }

    fn section_title(&mut self, text: impl Into<String>) -> egui::Response {
        self.add(
            StyledText::new(text)
                .size(TextSize::Lg)
                .color(TextColor::Strong),
        )
    }

    fn card_title(&mut self, text: impl Into<String>) -> egui::Response {
        self.add(
            StyledText::new(text)
                .size(TextSize::CardTitle)
                .color(TextColor::Strong),
        )
    }

    fn muted_label(&mut self, text: impl Into<String>) -> egui::Response {
        self.add(
            StyledText::new(text)
                .size(TextSize::Sm)
                .color(TextColor::Muted),
        )
    }

    fn metadata_label(&mut self, text: impl Into<String>) -> egui::Response {
        self.add(
            StyledText::new(text)
                .size(TextSize::Xs)
                .color(TextColor::Muted),
        )
    }

    fn strong_label(&mut self, text: impl Into<String>) -> egui::Response {
        self.add(
            StyledText::new(text)
                .size(TextSize::Md)
                .color(TextColor::Strong),
        )
    }

    fn text(&mut self, text: impl Into<String>, size: TextSize) -> Response {
        self.add(StyledText::new(text).size(size))
    }

    fn text_colored(
        &mut self,
        text: impl Into<String>,
        size: TextSize,
        color: TextColor,
    ) -> Response {
        self.add(StyledText::new(text).size(size).color(color))
    }
}

/// Width of a themed single-line text input.
///
/// The height is always [`INPUT_HEIGHT`]; only the
/// width changes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputWidth {
    /// The theme's standard width, [`INPUT_WIDTH`].
    Standard,
    /// An explicit width in logical points.
    Fixed(f32),
    /// All the width still available in the current layout, as reported by
    /// [`Ui::available_width`] when the input is added.
    ///
    /// In a horizontal layout this is the space left after the widgets
    /// already placed on the row.
    Fill,
}

/// Convenience helpers for themed single-line text inputs.
///
/// Every input uses the theme's standard height and centers its text
/// vertically. The width defaults to [`InputWidth::Standard`]; use the
/// `*_with_width` methods to choose a different one.
pub trait UiInputs {
    /// Adds a single-line text input using the theme's standard input size.
    fn text_input(&mut self, text: &mut String) -> Response;

    /// Adds a single-line text input with the given [`InputWidth`] and the
    /// theme's standard input height.
    fn text_input_with_width(&mut self, text: &mut String, width: InputWidth) -> Response;

    /// Adds a single-line text input with placeholder text using the theme's
    /// standard input size.
    ///
    /// The hint is shown only while `text` is empty.
    fn text_input_hint(&mut self, text: &mut String, hint: impl Into<egui::WidgetText>)
    -> Response;

    /// Adds a single-line text input with placeholder text, the given
    /// [`InputWidth`] and the theme's standard input height.
    ///
    /// The hint is shown only while `text` is empty.
    fn text_input_hint_with_width(
        &mut self,
        text: &mut String,
        hint: impl Into<egui::WidgetText>,
        width: InputWidth,
    ) -> Response;
}

fn add_text_input(
    ui: &mut Ui,
    text: &mut String,
    hint: Option<egui::WidgetText>,
    width: InputWidth,
) -> Response {
    let width = match width {
        InputWidth::Standard => metrics::INPUT_WIDTH,
        InputWidth::Fill => ui.available_width(),
        InputWidth::Fixed(w) => w,
    };

    let mut input = egui::TextEdit::singleline(text).vertical_align(egui::Align::Center);

    if let Some(hint) = hint {
        input = input.hint_text(hint);
    }

    ui.add_sized([width, metrics::INPUT_HEIGHT], input)
}

impl UiInputs for Ui {
    fn text_input(&mut self, text: &mut String) -> Response {
        add_text_input(self, text, None, InputWidth::Standard)
    }

    fn text_input_hint(
        &mut self,
        text: &mut String,
        hint: impl Into<egui::WidgetText>,
    ) -> Response {
        add_text_input(self, text, Some(hint.into()), InputWidth::Standard)
    }

    fn text_input_with_width(&mut self, text: &mut String, width: InputWidth) -> Response {
        add_text_input(self, text, None, width)
    }

    fn text_input_hint_with_width(
        &mut self,
        text: &mut String,
        hint: impl Into<egui::WidgetText>,
        width: InputWidth,
    ) -> Response {
        add_text_input(self, text, Some(hint.into()), width)
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

/// Background level of a card, mapped to the surface colors of the [`Palette`].
///
/// Each variant selects both the fill and the border color, so that the card
/// stays readable against the layer it is meant to sit on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelVariant {
    /// Default card background, placed directly above the window background.
    ///
    /// Uses [`Palette::surface`] with the regular [`Palette::border`].
    Surface,
    /// Secondary background for nested or less prominent content.
    ///
    /// Uses [`Palette::surface_alt`] with the regular [`Palette::border`].
    SurfaceAlt,
    /// Raised background for content that must stand out from the cards
    /// around it.
    ///
    /// Uses [`Palette::elevated`] with [`Palette::border_strong`], which
    /// marks the card edge more clearly than the regular border.
    Elevated,
}

impl PanelVariant {
    /// Returns the fill color of this variant in the given palette.
    fn into_color(self, palette: Palette) -> Color32 {
        match self {
            PanelVariant::Surface => palette.surface,
            PanelVariant::SurfaceAlt => palette.surface_alt,
            PanelVariant::Elevated => palette.elevated,
        }
    }
    /// Returns the border color of this variant in the given palette.
    fn into_border_color(self, palette: Palette) -> Color32 {
        match self {
            PanelVariant::Surface | PanelVariant::SurfaceAlt => palette.border,
            PanelVariant::Elevated => palette.border_strong,
        }
    }
}

/// Themed containers for grouping related content.
///
/// All cards share the same geometry: a 1 px border, [`metrics::RADIUS_MD`]
/// corners and [`metrics::PANEL_PADDING`] inner margin. Colors are read from
/// the [`Palette`] of the current context, so cards follow the active theme.
///
/// A card is only as large as its content. To make it fill the available
/// width, call `ui.set_min_width(ui.available_width())` inside the closure.
///
/// # Examples
///
/// ```
/// use modern_egui::theme::UiPanels;
/// use modern_egui::theme::ui_ext::PanelVariant;
///
/// # fn show(ui: &mut egui::Ui) {
/// ui.card(|ui| {
///     ui.label("Default card");
/// });
///
/// ui.card_with_variant(PanelVariant::Elevated, |ui| {
///     ui.label("Raised card");
/// });
/// # }
/// ```
pub trait UiPanels {
    /// Shows a card with the default [`PanelVariant::Surface`] background.
    ///
    /// Shorthand for [`card_with_variant`](Self::card_with_variant) with
    /// [`PanelVariant::Surface`].
    ///
    /// Returns the value produced by `add_contents` together with the
    /// [`Response`] of the whole card, padding included.
    fn card<R>(&mut self, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R>;

    /// Shows a card that highlights itself while the pointer is over it.
    ///
    /// At rest the card looks like [`card`](Self::card); while hovered it is
    /// painted with the [`PanelVariant::Elevated`] colors instead.
    ///
    /// The returned [`Response`] covers the whole card, padding included, but
    /// only senses hovering. To react to clicks, upgrade it with
    /// [`Response::interact`].
    ///
    /// # Examples
    ///
    /// ```
    /// use modern_egui::theme::UiPanels;
    ///
    /// # fn show(ui: &mut egui::Ui) {
    /// let card = ui.interactive_card(|ui| {
    ///     ui.label("Click me");
    /// });
    ///
    /// if card.response.interact(egui::Sense::click()).clicked() {
    ///     // handle the click
    /// }
    /// # }
    /// ```
    fn interactive_card<R>(&mut self, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R>;

    /// Shows a card with the fill and border colors of the given `variant`.
    ///
    /// Returns the value produced by `add_contents` together with the
    /// [`Response`] of the whole card, padding included.
    fn card_with_variant<R>(
        &mut self,
        variant: PanelVariant,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R>;
}

impl UiPanels for Ui {
    fn card_with_variant<R>(
        &mut self,
        variant: PanelVariant,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        let p = Palette::of(self.ctx());

        egui::Frame::new()
            .fill(variant.into_color(p))
            .stroke(Stroke::new(1.0, variant.into_border_color(p)))
            .corner_radius(metrics::RADIUS_MD)
            .inner_margin(metrics::PANEL_PADDING)
            .show(self, add_contents)
    }

    fn card<R>(&mut self, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        self.card_with_variant(PanelVariant::Surface, add_contents)
    }

    fn interactive_card<R>(&mut self, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        // ref: https://docs.rs/crate/egui/latest/source/src/containers/frame.rs#74
        let p = Palette::of(self.ctx());
        let mut frame = egui::Frame::new()
            .fill(PanelVariant::Surface.into_color(p))
            .stroke(Stroke::new(1.0, PanelVariant::Surface.into_border_color(p)))
            .corner_radius(metrics::RADIUS_MD)
            .inner_margin(metrics::PANEL_PADDING)
            .begin(self);

        let inner = add_contents(&mut frame.content_ui);

        let response = frame.allocate_space(self);

        if response.hovered() {
            frame.frame.fill = PanelVariant::Elevated.into_color(p);
            frame.frame.stroke = Stroke::new(1.0, PanelVariant::Elevated.into_border_color(p));
        }
        frame.paint(self);

        InnerResponse::new(inner, response)
    }
}

/// Layout helpers for arranging groups of widgets on an [`egui::Ui`].
pub trait UiLayouts {
    /// Adds a horizontal row of widgets centered in the available width.
    ///
    /// `id_salt` must be unique among the centered rows of the same `Ui`.
    fn center_row<R>(
        &mut self,
        id_salt: impl std::hash::Hash + std::fmt::Debug,
        add_contents: impl FnOnce(&mut egui::Ui) -> R,
    ) -> InnerResponse<R>;
}

impl UiLayouts for Ui {
    fn center_row<R>(
        &mut self,
        id_salt: impl std::hash::Hash + std::fmt::Debug,
        add_contents: impl FnOnce(&mut egui::Ui) -> R,
    ) -> InnerResponse<R> {
        let id = self.id().with(id_salt);
        let last_width: Option<f32> = self.data(|d| d.get_temp(id));

        let response = self.horizontal(|ui| {
            let available = ui.available_width();

            let margin = match last_width {
                Some(w) if available.is_finite() => ((available - w) / 2.0).max(0.0),
                _ => 0.0,
            };
            ui.add_space(margin);

            let inner = ui.scope(|ui| add_contents(ui));

            let width = inner.response.rect.width();
            ui.data_mut(|d| d.insert_temp(id, width));

            if last_width.is_none_or(|w| (w - width).abs() > 0.1) {
                // the measure has changed force repaint
                ui.ctx().request_repaint();
            }

            inner
        });

        response.inner
    }
}
