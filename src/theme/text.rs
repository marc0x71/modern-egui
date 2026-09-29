//! Themed text widget.
//!
//! [`StyledText`] renders a label with a semantic [`TextSize`] and
//! [`TextColor`] resolved against the active [`Palette`]. The
//! [`UiText`](crate::theme::UiText) helpers are presets built on top of it.

use crate::theme::{Palette, metrics};

/// Semantic text sizes provided by the modern-egui typography scale.
///
/// Each variant maps to one of the font-size tokens defined in [`metrics`].
#[derive(Debug, Clone, Copy)]
pub enum TextSize {
    /// Extra-small text, intended for metadata and low-emphasis information.
    Xs,
    /// Small text, intended for secondary or supporting content.
    Sm,
    /// Default body text size.
    Md,
    /// Text size intended for card and panel titles.
    CardTitle,
    /// Large text, intended for section titles.
    Lg,
    /// Extra-large text, intended for headings.
    Xl,
    /// Largest text size, intended for page or application titles.
    Xxl,
}

impl TextSize {
    /// Returns the font size, in egui points, associated with this text size.
    pub const fn value(self) -> f32 {
        match self {
            Self::Xs => metrics::FONT_XS,
            Self::Sm => metrics::FONT_SM,
            Self::Md => metrics::FONT_MD,
            Self::CardTitle => metrics::FONT_CT,
            Self::Lg => metrics::FONT_LG,
            Self::Xl => metrics::FONT_XL,
            Self::Xxl => metrics::FONT_2XL,
        }
    }
}

/// Semantic color roles available to [`StyledText`].
///
/// Semantic variants are resolved against the currently active [`Palette`],
/// allowing text to automatically adapt to light and dark themes.
#[derive(Debug, Clone, Copy, Default)]
pub enum TextColor {
    #[default]
    /// Uses the default body text color.
    Text,
    /// Uses the high-emphasis text color.
    Strong,
    /// Uses the muted, low-emphasis text color.
    Muted,
    /// Uses the theme accent color.
    Accent,
    /// Uses the semantic success color.
    Success,
    /// Uses the semantic warning color.
    Warning,
    /// Uses the semantic danger color.
    Danger,
    /// Uses an explicitly specified color instead of a semantic theme color.
    Custom(egui::Color32),
}

/// A themed text widget with configurable size, color, and emphasis.
///
/// `StyledText` uses semantic typography and color tokens from the active
/// modern-egui theme while still allowing explicit customization.
///
/// # Example
///
/// ```
/// use modern_egui::theme::{StyledText, TextColor, TextSize};
///
/// # fn show(ui: &mut egui::Ui) {
/// ui.add(
///     StyledText::new("Settings")
///         .size(TextSize::Lg)
///         .color(TextColor::Strong)
/// );
/// # }
/// ```
pub struct StyledText {
    text: String,
    size: TextSize,
    color: TextColor,
}

impl StyledText {
    /// Creates themed text using the default body size and text color.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            size: TextSize::Md,
            color: TextColor::default(),
        }
    }

    /// Sets the semantic font size used to render the text.
    #[must_use]
    pub fn size(mut self, size: TextSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the semantic or custom color used to render the text.
    #[must_use]
    pub fn color(mut self, color: TextColor) -> Self {
        self.color = color;
        self
    }
}

impl egui::Widget for StyledText {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let p = Palette::of(ui.ctx());

        let color = match self.color {
            TextColor::Text => p.text,
            TextColor::Strong => p.text_strong,
            TextColor::Muted => p.text_muted,
            TextColor::Accent => p.accent,
            TextColor::Success => p.success,
            TextColor::Warning => p.warning,
            TextColor::Danger => p.danger,
            TextColor::Custom(color) => color,
        };

        let mut text = egui::RichText::new(self.text).size(self.size.value());

        text = text.color(color);

        ui.label(text)
    }
}
