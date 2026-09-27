use eframe::egui::{Color32, Context, Id, Theme};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Palette {
    /// Window and panel background — the furthest-back layer.
    pub bg: Color32,
    /// Cards and side panels sitting on top of `bg`.
    pub surface: Color32,
    /// Hover state for surfaces, and background of inputs.
    pub surface_alt: Color32,
    /// Popups, menus, modals.
    pub elevated: Color32,

    /// Hairline borders and separators.
    pub border: Color32,
    /// Border of a control the pointer is over.
    pub border_strong: Color32,

    /// Primary text.
    pub text: Color32,
    /// Emphasised text
    pub text_strong: Color32,
    /// Secondary text, placeholders, disabled labels.
    pub text_muted: Color32,

    /// Brand colour: primary buttons, selection, links, focus ring.
    pub accent: Color32,
    /// Accent under the pointer.
    pub accent_hover: Color32,
    /// Accent while pressed.
    pub accent_active: Color32,
    /// Text drawn on top of the accent.
    pub on_accent: Color32,

    /// Destructive actions.
    pub danger: Color32,
    pub danger_hover: Color32,
    pub danger_active: Color32,
    /// Text drawn on top of `danger`.
    pub on_danger: Color32,

    pub success: Color32,
    pub warning: Color32,
}

impl Palette {
    pub const fn dark() -> Self {
        Self {
            bg: Color32::from_rgb(17, 18, 23),
            surface: Color32::from_rgb(25, 27, 34),
            surface_alt: Color32::from_rgb(34, 37, 46),
            elevated: Color32::from_rgb(31, 33, 41),

            border: Color32::from_rgb(45, 48, 59),
            border_strong: Color32::from_rgb(66, 70, 84),

            text: Color32::from_rgb(232, 234, 240),
            text_strong: Color32::from_rgb(250, 251, 254),
            text_muted: Color32::from_rgb(143, 149, 166),

            accent: Color32::from_rgb(59, 130, 246),
            accent_hover: Color32::from_rgb(96, 165, 250),
            accent_active: Color32::from_rgb(37, 99, 235),
            on_accent: Color32::from_rgb(255, 255, 255),

            danger: Color32::from_rgb(226, 78, 84),
            danger_hover: Color32::from_rgb(240, 97, 103),
            danger_active: Color32::from_rgb(198, 62, 68),
            on_danger: Color32::from_rgb(255, 255, 255),

            success: Color32::from_rgb(64, 190, 130),
            warning: Color32::from_rgb(230, 170, 70),
        }
    }

    pub const fn light() -> Self {
        Self {
            bg: Color32::from_rgb(244, 245, 248),
            surface: Color32::from_rgb(255, 255, 255),
            surface_alt: Color32::from_rgb(238, 240, 245),
            elevated: Color32::from_rgb(255, 255, 255),

            border: Color32::from_rgb(222, 225, 232),
            border_strong: Color32::from_rgb(196, 201, 212),

            text: Color32::from_rgb(28, 31, 40),
            text_strong: Color32::from_rgb(9, 11, 17),
            text_muted: Color32::from_rgb(106, 112, 128),

            // Blue 600 / 700 / 800, one step darker than the dark theme's.
            // Blue 500 with white text only reaches 3.7:1, which fails WCAG AA
            // for body text; blue 600 reaches 5.2:1. On a light background both
            // hover and press go darker — there is no "lighter" available.
            accent: Color32::from_rgb(37, 99, 235),
            accent_hover: Color32::from_rgb(29, 78, 216),
            accent_active: Color32::from_rgb(30, 64, 175),
            on_accent: Color32::from_rgb(255, 255, 255),

            danger: Color32::from_rgb(211, 57, 63),
            danger_hover: Color32::from_rgb(226, 74, 80),
            danger_active: Color32::from_rgb(184, 46, 52),
            on_danger: Color32::from_rgb(255, 255, 255),

            success: Color32::from_rgb(34, 154, 96),
            warning: Color32::from_rgb(190, 130, 30),
        }
    }

    pub fn store(self, ctx: &Context, theme: Theme) {
        ctx.data_mut(|d| d.insert_temp(Self::id(theme), self));
    }

    fn id(theme: Theme) -> Id {
        Id::new(("modern_egui_palette", theme))
    }

    pub fn of(ctx: &Context) -> Self {
        let theme = ctx.theme();
        ctx.data(|d| d.get_temp::<Self>(Self::id(theme)))
            .unwrap_or_else(|| Self::for_theme(theme))
    }

    pub fn for_theme(theme: Theme) -> Self {
        match theme {
            Theme::Dark => Self::dark(),
            Theme::Light => Self::light(),
        }
    }
}
