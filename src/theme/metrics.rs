use eframe::egui::{self, FontFamily, FontId, Style, TextStyle};

/// Base unit used to derive the theme's typography and layout metrics.
pub const EM: f32 = 14.0;

// Typography

/// Extra-small font size, intended for metadata and low-emphasis labels.
pub const FONT_XS: f32 = EM * 0.80;

/// Small font size, intended for secondary text.
pub const FONT_SM: f32 = EM * 0.90;

/// Default body font size.
pub const FONT_MD: f32 = EM;

/// Card-title font size.
pub const FONT_CT: f32 = EM * 1.14;

/// Section-title font size.
pub const FONT_LG: f32 = EM * 1.28;

/// Heading font size.
pub const FONT_XL: f32 = EM * 1.47;

/// Largest font size, intended for application or page titles.
pub const FONT_2XL: f32 = EM * 1.73;

// Spacing
/// Smallest spacing step in the raw spacing scale.
pub const SPACE_1: f32 = EM * 0.27;

/// Second spacing step in the raw spacing scale.
pub const SPACE_2: f32 = EM * 0.53;

/// Third spacing step in the raw spacing scale.
pub const SPACE_3: f32 = EM * 0.80;

/// Fourth spacing step in the raw spacing scale.
pub const SPACE_4: f32 = EM * 1.07;

/// Fifth spacing step in the raw spacing scale.
pub const SPACE_5: f32 = EM * 1.33;

/// Largest spacing step in the raw spacing scale.
pub const SPACE_6: f32 = EM * 1.60;

// Controls
/// Default minimum height for interactive controls.
pub const CONTROL_HEIGHT: f32 = EM * 2.1;

/// Height for larger interactive controls.
pub const CONTROL_HEIGHT_LG: f32 = EM * 2.6;

/// Horizontal padding used by buttons.
pub const BUTTON_PAD_X: f32 = EM * 0.67;

/// Vertical padding used by buttons.
pub const BUTTON_PAD_Y: f32 = EM * 0.40;

/// Standard height used by themed single-line text inputs.
pub const INPUT_HEIGHT: f32 = EM * 2.0;

/// Standard width used by themed single-line text inputs.
pub const INPUT_WIDTH: f32 = EM * 18.0;

// Radius
/// Small corner radius.
pub const RADIUS_SM: f32 = EM * 0.27;

/// Medium corner radius.
pub const RADIUS_MD: f32 = EM * 0.53;

/// Large corner radius.
pub const RADIUS_LG: f32 = EM * 0.80;

// Gap
/// Spacing between closely related inline elements.
pub const GAP_INLINE: f32 = SPACE_2;

/// Spacing between elements belonging to the same logical group.
pub const GAP_GROUP: f32 = SPACE_3;

/// Spacing between distinct sections.
pub const GAP_SECTION: f32 = SPACE_4;

/// Spacing between major page-level blocks.
pub const GAP_PAGE: f32 = SPACE_5;

// Layouts
/// Numeric value used to derive the standard panel padding.
pub const PANEL_PADDING_VALUE: f32 = EM * 1.07;

/// Standard inner margin used by panels and cards.
pub const PANEL_PADDING: egui::Margin = egui::Margin::same(PANEL_PADDING_VALUE as i8);

/// Default sidebar width.
pub const SIDEBAR_WIDTH: f32 = EM * 14.0;

/// Minimum recommended sidebar width.
pub const SIDEBAR_MIN_WIDTH: f32 = EM * 11.0;

/// Maximum recommended sidebar width.
pub const SIDEBAR_MAX_WIDTH: f32 = EM * 21.0;

/// Applies the theme's typography, spacing, and control metrics to an egui style.
///
/// This configures the standard text styles, item spacing, button padding,
/// and minimum interactive control height.
pub fn apply(style: &mut Style) {
    style.text_styles = [
        (
            TextStyle::Heading,
            FontId::new(FONT_XL, FontFamily::Proportional),
        ),
        (
            TextStyle::Body,
            FontId::new(FONT_MD, FontFamily::Proportional),
        ),
        (
            TextStyle::Button,
            FontId::new(FONT_MD, FontFamily::Proportional),
        ),
        (
            TextStyle::Small,
            FontId::new(FONT_XS, FontFamily::Proportional),
        ),
        (
            TextStyle::Monospace,
            FontId::new(FONT_SM, FontFamily::Monospace),
        ),
    ]
    .into();

    style.spacing.item_spacing = egui::vec2(SPACE_2, SPACE_2);

    style.spacing.button_padding = egui::vec2(BUTTON_PAD_X, BUTTON_PAD_Y);

    style.spacing.interact_size.y = CONTROL_HEIGHT;
}
