use eframe::egui::{self, FontFamily, FontId, Style, TextStyle};

pub const EM: f32 = 14.0;

// Typography
pub const FONT_XS: f32 = EM * 0.80;
pub const FONT_SM: f32 = EM * 0.90;
pub const FONT_MD: f32 = EM;
pub const FONT_CT: f32 = EM * 1.14;
pub const FONT_LG: f32 = EM * 1.28;
pub const FONT_XL: f32 = EM * 1.47;
pub const FONT_2XL: f32 = EM * 1.73;

// Spacing
pub const SPACE_1: f32 = EM * 0.27;
pub const SPACE_2: f32 = EM * 0.53;
pub const SPACE_3: f32 = EM * 0.80;
pub const SPACE_4: f32 = EM * 1.07;
pub const SPACE_5: f32 = EM * 1.33;
pub const SPACE_6: f32 = EM * 1.60;

// Controls
pub const CONTROL_HEIGHT: f32 = EM * 2.1;
pub const CONTROL_HEIGHT_LG: f32 = EM * 2.6;
pub const BUTTON_PAD_X: f32 = EM * 0.67;
pub const BUTTON_PAD_Y: f32 = EM * 0.40;
pub const INPUT_HEIGHT: f32 = EM * 2.0;
pub const INPUT_WIDTH: f32 = EM * 18.0;

// Radius
pub const RADIUS_SM: f32 = EM * 0.27;
pub const RADIUS_MD: f32 = EM * 0.53;
pub const RADIUS_LG: f32 = EM * 0.80;

/// Gap
pub const GAP_INLINE: f32 = SPACE_2;
pub const GAP_GROUP: f32 = SPACE_3;
pub const GAP_SECTION: f32 = SPACE_4;
pub const GAP_PAGE: f32 = SPACE_5;

// Layouts
pub const PANEL_PADDING_VALUE: f32 = EM * 1.07;

pub const PANEL_PADDING: egui::Margin = egui::Margin::same(PANEL_PADDING_VALUE as i8);

pub const SIDEBAR_WIDTH: f32 = EM * 14.0; // 210
pub const SIDEBAR_MIN_WIDTH: f32 = EM * 11.0; // 165
pub const SIDEBAR_MAX_WIDTH: f32 = EM * 21.0; // 315

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
