use eframe::egui::{self, Align, Layout, Stroke, ThemePreference};
use modern_egui::theme::{
    self, InputWidth, Palette, PanelVariant, TextColor, TextSize, UiButtons, UiInputs, UiMetrics,
    UiPanels, UiText, metrics, rgb_hex,
};

/// Gruvbox dark, hard contrast.
const GRUVBOX_DARK: Palette = Palette {
    bg: rgb_hex(0x1d2021),
    surface: rgb_hex(0x282828),
    surface_alt: rgb_hex(0x3c3836),
    elevated: rgb_hex(0x32302f),

    border: rgb_hex(0x504945),
    border_strong: rgb_hex(0x665c54),

    text: rgb_hex(0xebdbb2),
    text_strong: rgb_hex(0xfbf1c7),
    text_muted: rgb_hex(0xa89984),

    accent: rgb_hex(0x689d6a),
    accent_hover: rgb_hex(0x8ec07c),
    accent_active: rgb_hex(0x427b58),
    on_accent: rgb_hex(0x282828),

    danger: rgb_hex(0xfb4934),
    danger_hover: rgb_hex(0xfc6b5a),
    danger_active: rgb_hex(0xcc241d),
    on_danger: rgb_hex(0x1d2021),

    success: rgb_hex(0xb8bb26),
    warning: rgb_hex(0xfabd2f),
};

/// Gruvbox light.
const GRUVBOX_LIGHT: Palette = Palette {
    bg: rgb_hex(0xfbf1c7),
    surface: rgb_hex(0xf9f5d7),
    surface_alt: rgb_hex(0xebdbb2),
    elevated: rgb_hex(0xf9f5d7),

    border: rgb_hex(0xd5c4a1),
    border_strong: rgb_hex(0xbdae93),

    text: rgb_hex(0x3c3836),
    text_strong: rgb_hex(0x282828),
    text_muted: rgb_hex(0x665c54),

    accent: rgb_hex(0x427b58),
    accent_hover: rgb_hex(0x396a4c),
    accent_active: rgb_hex(0x30593f),
    on_accent: rgb_hex(0xf9f5d7),

    danger: rgb_hex(0x9d0006),
    danger_hover: rgb_hex(0xcc241d),
    danger_active: rgb_hex(0x7a0005),
    on_danger: rgb_hex(0xf9f5d7),

    success: rgb_hex(0x79740e),
    warning: rgb_hex(0xaf3a03),
};

fn main() -> eframe::Result {
    let title = "Modern egui — Gruvbox";

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(title)
            .with_inner_size([680.0, 660.0])
            .with_min_inner_size([680.0, 420.0]),
        ..Default::default()
    };

    eframe::run_native(
        title,
        options,
        Box::new(|cc| Ok(Box::new(GruvboxApp::new(cc)))),
    )
}

struct GruvboxApp {
    email: String,
    notifications: bool,
    volume: f32,
}

impl GruvboxApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        theme::apply_with(&cc.egui_ctx, GRUVBOX_DARK, GRUVBOX_LIGHT);
        cc.egui_ctx.set_theme(ThemePreference::Dark);

        Self {
            email: String::new(),
            notifications: true,
            volume: 62.0,
        }
    }

    fn header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.app_title("Gruvbox");
                ui.muted_label("Custom light and dark palettes with theme::apply_with");
            });

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let current = ui.ctx().theme();

                if ui
                    .selectable_label(current == egui::Theme::Light, "Light")
                    .clicked()
                {
                    ui.ctx().set_theme(ThemePreference::Light);
                }

                if ui
                    .selectable_label(current == egui::Theme::Dark, "Dark")
                    .clicked()
                {
                    ui.ctx().set_theme(ThemePreference::Dark);
                }
            });
        });
    }

    fn swatches(&mut self, ui: &mut egui::Ui) {
        let p = Palette::of(ui.ctx());

        let colors = [
            ("bg", p.bg),
            ("surface", p.surface),
            ("surface_alt", p.surface_alt),
            ("elevated", p.elevated),
            ("border", p.border),
            ("border_strong", p.border_strong),
            ("text", p.text),
            ("text_strong", p.text_strong),
            ("text_muted", p.text_muted),
            ("accent", p.accent),
            ("accent_hover", p.accent_hover),
            ("accent_active", p.accent_active),
            ("on_accent", p.on_accent),
            ("danger", p.danger),
            ("danger_hover", p.danger_hover),
            ("danger_active", p.danger_active),
            ("on_danger", p.on_danger),
            ("success", p.success),
            ("warning", p.warning),
        ];

        ui.section_title("Palette");
        ui.space_group();

        let card_width = 64.0;
        let gap = metrics::SPACE_2;

        let available_width = ui.available_width();

        let columns = ((available_width + gap) / (card_width + gap))
            .floor()
            .max(1.0) as usize;

        egui::Grid::new("palette_grid")
            .num_columns(columns)
            .spacing([gap, gap])
            .show(ui, |ui| {
                for (index, (name, color)) in colors.into_iter().enumerate() {
                    ui.vertical(|ui| {
                        ui.set_min_width(card_width);
                        ui.set_max_width(card_width);
                        egui::Frame::new()
                            .fill(color)
                            .stroke(Stroke::new(1.0, p.border))
                            .corner_radius(metrics::RADIUS_SM)
                            .show(ui, |ui| {
                                ui.allocate_space(egui::vec2(
                                    card_width - theme::metrics::SPACE_1,
                                    36.0,
                                ));
                            });
                        ui.metadata_label(name);
                    });

                    if (index + 1) % columns == 0 {
                        ui.end_row();
                    }
                }
            });
    }

    fn buttons(&mut self, ui: &mut egui::Ui) {
        ui.card(|ui| {
            ui.set_min_width(ui.available_width());

            ui.card_title("Buttons");
            ui.muted_label("Each variant picks its colors from the active palette.");
            ui.space_group();

            ui.horizontal(|ui| {
                let _ = ui.primary_button("Primary");
                let _ = ui.secondary_button("Secondary");
                let _ = ui.ghost_button("Ghost");
                let _ = ui.danger_button("Danger");
            });
        });
    }

    fn controls(&mut self, ui: &mut egui::Ui) {
        ui.card(|ui| {
            ui.set_min_width(ui.available_width());

            ui.card_title("Controls");
            ui.muted_label("Plain egui widgets follow the palette too.");
            ui.space_group();

            ui.text_input_hint_with_width(&mut self.email, "email@example.com", InputWidth::Fill);
            ui.space_inline();

            ui.checkbox(&mut self.notifications, "Email notifications");
            ui.add(egui::Slider::new(&mut self.volume, 0.0..=100.0).text("Volume"));
            ui.add(egui::ProgressBar::new(self.volume / 100.0).show_percentage());
            ui.space_inline();

            let _ = ui.link("A link in the accent color");
        });
    }

    fn feedback(&mut self, ui: &mut egui::Ui) {
        ui.card_with_variant(PanelVariant::Elevated, |ui| {
            ui.set_min_width(ui.available_width());

            ui.card_title("Semantic colors");
            ui.space_group();

            ui.text_colored(
                "Accent: something worth noticing",
                TextSize::Md,
                TextColor::Accent,
            );
            ui.text_colored("Success: changes saved", TextSize::Md, TextColor::Success);
            ui.text_colored(
                "Warning: disk almost full",
                TextSize::Md,
                TextColor::Warning,
            );
            ui.text_colored("Danger: connection lost", TextSize::Md, TextColor::Danger);
        });
    }
}

impl eframe::App for GruvboxApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.header(ui);
                ui.space_section();

                self.swatches(ui);
                ui.space_section();

                self.buttons(ui);
                ui.space_group();

                self.controls(ui);
                ui.space_group();

                self.feedback(ui);
            });
        });
    }
}
