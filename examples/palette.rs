use eframe::egui::{self, Align, Button, Color32, Layout, RichText, Stroke, ThemePreference};

use egui_phosphor::regular as ph;

use modern_egui::prelude::*;
use modern_egui::theme::{self, Palette, metrics};

fn main() -> eframe::Result {
    let title = "Modern egui — Theme Demo";

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(title)
            .with_inner_size([1180.0, 820.0])
            .with_min_inner_size([720.0, 560.0]),
        ..Default::default()
    };

    eframe::run_native(
        title,
        options,
        Box::new(|cc| Ok(Box::new(DemoApp::new(cc)))),
    )
}

struct DemoApp {
    username: String,
    email: String,
    search: String,

    enabled: bool,
    notifications: bool,
    remember_me: bool,

    volume: f32,
    progress: f32,

    selected_tab: usize,
    selected_plan: usize,

    counter: i32,
}

impl DemoApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);

        cc.egui_ctx.set_fonts(fonts);

        theme::apply(&cc.egui_ctx);

        cc.egui_ctx.set_theme(ThemePreference::Dark);

        Self {
            username: "Mario Rossi".to_owned(),
            email: "mario@example.com".to_owned(),
            search: String::new(),

            enabled: true,
            notifications: true,
            remember_me: false,

            volume: 62.0,
            progress: 0.68,

            selected_tab: 0,
            selected_plan: 1,

            counter: 12,
        }
    }

    // -------------------------------------------------------------------------
    // Header
    // -------------------------------------------------------------------------

    fn header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.app_title("Modern egui");
                ui.muted_label("Theme & component showcase");
            });

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let current_theme = ui.ctx().theme();

                if ui
                    .selectable_label(current_theme == egui::Theme::Dark, (ph::MOON, "Dark"))
                    .clicked()
                {
                    ui.ctx().set_theme(ThemePreference::Dark);
                }

                if ui
                    .selectable_label(current_theme == egui::Theme::Light, (ph::SUN, "Light"))
                    .clicked()
                {
                    ui.ctx().set_theme(ThemePreference::Light);
                }

                ui.separator();

                ui.metadata_label("Theme");
            });
        });
    }

    // -------------------------------------------------------------------------
    // Palette
    // -------------------------------------------------------------------------

    fn palette_section(&mut self, ui: &mut egui::Ui) {
        let p = Palette::of(ui.ctx());

        ui.section_title("Color palette");
        ui.space_inline();

        ui.muted_label(
            "Colors are read directly from Palette::of(ctx) \
     and change together with the theme.",
        );

        ui.space_group();

        let card_width = metrics::EM * 12.5;
        let gap = metrics::SPACE_2;

        let available_width = ui.available_width();

        let columns = ((available_width + gap) / (card_width + gap))
            .floor()
            .max(1.0) as usize;

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

        egui::Grid::new("palette_grid")
            .num_columns(columns)
            .spacing([gap, gap])
            .show(ui, |ui| {
                for (index, (name, color)) in colors.into_iter().enumerate() {
                    self.color_cell(ui, name, color, card_width);

                    if (index + 1) % columns == 0 {
                        ui.end_row();
                    }
                }
            });
    }

    fn color_cell(&self, ui: &mut egui::Ui, name: &str, color: Color32, width: f32) {
        let p = Palette::of(ui.ctx());

        let content_width = width - metrics::SPACE_3 * 2.0;

        let swatch_size = metrics::EM * 1.8;

        let response = ui
            .interactive_card(|ui| {
                ui.set_min_width(content_width);
                ui.set_max_width(content_width);

                ui.horizontal(|ui| {
                    let (rect, _) = ui.allocate_exact_size(
                        egui::vec2(swatch_size, swatch_size),
                        egui::Sense::hover(),
                    );

                    ui.painter().rect_filled(rect, metrics::RADIUS_SM, color);

                    ui.space2();

                    ui.vertical(|ui| {
                        ui.text_colored(name, TextSize::Sm, TextColor::Strong);

                        ui.label(
                            RichText::new(format!(
                                "#{:02X}{:02X}{:02X}",
                                color.r(),
                                color.g(),
                                color.b(),
                            ))
                            .monospace()
                            .size(metrics::FONT_XS)
                            .color(p.text_muted),
                        );
                    });
                });
            })
            .response;

        response.on_hover_text(format!(
            "{}\n#{:02X}{:02X}{:02X}\nrgb({}, {}, {})",
            name,
            color.r(),
            color.g(),
            color.b(),
            color.r(),
            color.g(),
            color.b(),
        ));
    }

    // -------------------------------------------------------------------------
    // Typography
    // -------------------------------------------------------------------------

    fn typography_section(&mut self, ui: &mut egui::Ui) {
        ui.section_title("Typography");
        ui.space_inline();

        ui.app_title("2XL — Application title");
        ui.page_title("XL — Page title");
        ui.section_title("LG — Section title");
        ui.card_title("CT — Card title");
        ui.text("MD — Body text", TextSize::Md);
        ui.muted_label("SM — Secondary text");
        ui.metadata_label("XS — Metadata");

        ui.space_group();

        ui.horizontal_wrapped(|ui| {
            ui.text_colored("Accent", TextSize::Md, TextColor::Accent);
            ui.separator();
            ui.text_colored("Success", TextSize::Md, TextColor::Success);
            ui.separator();
            ui.text_colored("Warning", TextSize::Md, TextColor::Warning);
            ui.separator();
            ui.text_colored("Danger", TextSize::Md, TextColor::Danger);
        });
    }

    // -------------------------------------------------------------------------
    // Buttons
    // -------------------------------------------------------------------------

    fn buttons_section(&mut self, ui: &mut egui::Ui) {
        ui.section_title("Buttons");
        ui.space_inline();

        ui.horizontal_wrapped(|ui| {
            let _ = ui.primary_button("Primary");
            let _ = ui.secondary_button("Secondary");
            let _ = ui.ghost_button("Ghost");
            let _ = ui.danger_button("Danger");
            let _ = ui.ghost_button(ph::X);

            ui.add_enabled(false, Button::new("Disabled"));
        });

        ui.space_group();

        ui.horizontal(|ui| {
            if ui.button("−").clicked() {
                self.counter -= 1;
            }

            ui.strong_label(self.counter.to_string());

            if ui.button("+").clicked() {
                self.counter += 1;
            }
        });
    }

    // -------------------------------------------------------------------------
    // Inputs
    // -------------------------------------------------------------------------

    fn inputs_section(&mut self, ui: &mut egui::Ui) {
        ui.section_title("Inputs");
        ui.space_inline();

        egui::Grid::new("input_grid")
            .num_columns(2)
            .spacing([metrics::GAP_SECTION, metrics::GAP_GROUP])
            .show(ui, |ui| {
                ui.label("Username");
                ui.text_input(&mut self.username);
                ui.end_row();

                ui.label("Email");
                ui.text_input_hint(&mut self.email, "email@example.com");
                ui.end_row();

                ui.label("Search");
                ui.text_input_hint(&mut self.search, "Search...");
                ui.end_row();
            });

        ui.space_group();

        ui.checkbox(&mut self.enabled, "Feature enabled");
        ui.checkbox(&mut self.notifications, "Enable notifications");
        ui.checkbox(&mut self.remember_me, "Remember me");

        ui.space_group();

        ui.add(
            egui::Slider::new(&mut self.volume, 0.0..=100.0)
                .text("Volume")
                .suffix("%"),
        );

        ui.space_inline();

        ui.muted_label(
            "Try focus, hover and click on the inputs \
     to check the border and accent colors.",
        );
    }

    // -------------------------------------------------------------------------
    // Selection
    // -------------------------------------------------------------------------

    fn selection_section(&mut self, ui: &mut egui::Ui) {
        ui.section_title("Selection & navigation");
        ui.space_inline();

        ui.horizontal_wrapped(|ui| {
            for (index, label) in ["Overview", "Activity", "Settings"].iter().enumerate() {
                if ui
                    .selectable_label(self.selected_tab == index, *label)
                    .clicked()
                {
                    self.selected_tab = index;
                }
            }
        });

        ui.space_group();

        ui.muted_label(format!(
            "Selected tab: {}",
            ["Overview", "Activity", "Settings",][self.selected_tab],
        ));

        ui.space_group();

        egui::ComboBox::from_label("Plan")
            .selected_text(match self.selected_plan {
                0 => "Free",
                1 => "Pro",
                _ => "Enterprise",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.selected_plan, 0, "Free");
                ui.selectable_value(&mut self.selected_plan, 1, "Pro");
                ui.selectable_value(&mut self.selected_plan, 2, "Enterprise");
            });
    }

    // -------------------------------------------------------------------------
    // Status
    // -------------------------------------------------------------------------

    fn status_section(&mut self, ui: &mut egui::Ui) {
        let p = Palette::of(ui.ctx());

        ui.section_title("Progress & semantic colors");
        ui.space_inline();

        ui.add(
            egui::ProgressBar::new(self.progress)
                .show_percentage()
                .desired_width(metrics::EM * 28.0),
        );

        ui.add(egui::Slider::new(&mut self.progress, 0.0..=1.0).text("Progress"));

        ui.space_group();

        ui.horizontal_wrapped(|ui| {
            self.status_badge(ui, "Success", p.success);
            self.status_badge(ui, "Warning", p.warning);
            self.status_badge(ui, "Danger", p.danger);
            self.status_badge(ui, "Accent", p.accent);
        });
    }

    fn status_badge(&self, ui: &mut egui::Ui, text: &str, color: Color32) {
        egui::Frame::new()
            .fill(color.gamma_multiply(0.15))
            .stroke(Stroke::new(1.0, color.gamma_multiply(0.8)))
            .corner_radius(metrics::RADIUS_LG)
            .inner_margin(egui::Margin::symmetric(
                metrics::SPACE_3 as i8,
                metrics::SPACE_1 as i8,
            ))
            .show(ui, |ui| {
                ui.text_colored(text, TextSize::Sm, TextColor::Custom(color));
            });
    }

    // -------------------------------------------------------------------------
    // Surfaces
    // -------------------------------------------------------------------------

    fn surfaces_section(&mut self, ui: &mut egui::Ui) {
        ui.section_title("Surface hierarchy");
        ui.space_inline();

        ui.muted_label(
            "Comparison between bg, surface, \
     surface_alt and elevated.",
        );

        ui.space_group();

        if ui.available_width() >= metrics::EM * 50.0 {
            ui.columns(3, |columns| {
                surface_card(
                    &mut columns[0],
                    "Surface",
                    "Main card above the background.",
                    PanelVariant::Surface,
                );

                surface_card(
                    &mut columns[1],
                    "Surface alt",
                    "Controls, hover states and secondary surfaces.",
                    PanelVariant::SurfaceAlt,
                );

                surface_card(
                    &mut columns[2],
                    "Elevated",
                    "Windows, popups and raised content.",
                    PanelVariant::Elevated,
                );
            });
        } else {
            surface_card(
                ui,
                "Surface",
                "Main card above the background.",
                PanelVariant::Surface,
            );
            ui.space_group();

            surface_card(
                ui,
                "Surface alt",
                "Controls, hover states and secondary surfaces.",
                PanelVariant::SurfaceAlt,
            );
            ui.space_group();

            surface_card(
                ui,
                "Elevated",
                "Windows, popups and raised content.",
                PanelVariant::Elevated,
            );
        }
    }

    // -------------------------------------------------------------------------
    // Table
    // -------------------------------------------------------------------------

    fn table_section(&mut self, ui: &mut egui::Ui) {
        ui.section_title("Table / striped rows");
        ui.space_inline();

        egui::Grid::new("demo_table")
            .striped(true)
            .min_col_width(metrics::EM * 8.0)
            .spacing([metrics::GAP_SECTION, metrics::GAP_INLINE])
            .show(ui, |ui| {
                ui.strong_label("Project".to_uppercase());
                ui.strong_label("Status".to_uppercase());
                ui.strong_label("Progress".to_uppercase());
                ui.end_row();

                ui.label("Modern egui");
                ui.text_colored("Active", TextSize::Md, TextColor::Success);
                ui.label("68%");
                ui.end_row();

                ui.label("Dashboard");
                ui.text_colored("Review", TextSize::Md, TextColor::Warning);
                ui.label("42%");
                ui.end_row();

                ui.label("Old UI");
                ui.text_colored("Deprecated", TextSize::Md, TextColor::Danger);
                ui.label("100%");
                ui.end_row();

                ui.label("Documentation");
                ui.text_colored("Draft", TextSize::Md, TextColor::Accent);
                ui.label("31%");
                ui.end_row();
            });
    }
}

// -----------------------------------------------------------------------------
// eframe App
// -----------------------------------------------------------------------------

impl eframe::App for DemoApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let p = Palette::of(ui.ctx());

        // ---------------------------------------------------------------------
        // Top bar
        // ---------------------------------------------------------------------

        egui::Panel::top("demo_top_bar")
            .frame(
                egui::Frame::new()
                    .fill(p.surface)
                    .inner_margin(metrics::PANEL_PADDING),
            )
            .show(ui, |ui| {
                self.header(ui);
            });

        // ---------------------------------------------------------------------
        // Sidebar
        // ---------------------------------------------------------------------

        egui::Panel::left("demo_sidebar")
            .resizable(true)
            .default_size(metrics::SIDEBAR_WIDTH)
            .size_range(metrics::SIDEBAR_MIN_WIDTH..=metrics::SIDEBAR_MAX_WIDTH)
            .frame(
                egui::Frame::new()
                    .fill(p.surface)
                    .stroke(Stroke::new(1.0, p.border))
                    .inner_margin(metrics::PANEL_PADDING),
            )
            .show(ui, |ui| {
                ui.metadata_label("THEME");
                ui.space_inline();
                ui.strong_label(match ui.ctx().theme() {
                    egui::Theme::Dark => "Dark mode",
                    egui::Theme::Light => "Light mode",
                });
                ui.space_group();
                ui.separator();

                ui.space_group();
                ui.metadata_label("QUICK COLORS");
                ui.space_inline();
                mini_color(ui, "Accent", p.accent);
                mini_color(ui, "Success", p.success);
                mini_color(ui, "Warning", p.warning);
                mini_color(ui, "Danger", p.danger);
                ui.space_group();
                ui.separator();

                ui.space_group();
                ui.metadata_label("BACKGROUND");
                ui.space_inline();
                ui.label(
                    RichText::new(format!("#{:02X}{:02X}{:02X}", p.bg.r(), p.bg.g(), p.bg.b(),))
                        .monospace()
                        .size(metrics::FONT_SM)
                        .color(p.text),
                );
            });

        // ---------------------------------------------------------------------
        // Content
        // ---------------------------------------------------------------------

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(p.bg)
                    .inner_margin(metrics::PANEL_PADDING),
            )
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        // Palette
                        self.palette_section(ui);
                        ui.space_page();

                        // Surfaces
                        self.surfaces_section(ui);
                        ui.space_page();

                        // Typography + Buttons
                        if ui.available_width() >= metrics::EM * 50.0 {
                            ui.columns(2, |columns| {
                                demo_card(&mut columns[0], |ui| {
                                    self.typography_section(ui);
                                });

                                demo_card(&mut columns[1], |ui| {
                                    self.buttons_section(ui);
                                });
                            });
                        } else {
                            demo_card(ui, |ui| {
                                self.typography_section(ui);
                            });

                            ui.space_section();

                            demo_card(ui, |ui| {
                                self.buttons_section(ui);
                            });
                        }

                        ui.space_page();

                        // Inputs + Selection
                        if ui.available_width() >= metrics::EM * 50.0 {
                            ui.columns(2, |columns| {
                                demo_card(&mut columns[0], |ui| {
                                    self.inputs_section(ui);
                                });

                                demo_card(&mut columns[1], |ui| {
                                    self.selection_section(ui);
                                });
                            });
                        } else {
                            demo_card(ui, |ui| {
                                self.inputs_section(ui);
                            });

                            ui.space_section();

                            demo_card(ui, |ui| {
                                self.selection_section(ui);
                            });
                        }

                        ui.space_page();

                        // Status
                        demo_card(ui, |ui| {
                            self.status_section(ui);
                        });

                        ui.space_page();

                        // Table
                        demo_card(ui, |ui| {
                            self.table_section(ui);
                        });
                    });
            });
    }
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

fn demo_card(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    ui.card(|ui| {
        ui.set_min_width(ui.available_width());

        add_contents(ui);
    });
}

fn surface_card(ui: &mut egui::Ui, title: &str, description: &str, variant: PanelVariant) {
    ui.card_with_variant(variant, |ui: &mut egui::Ui| {
        ui.set_min_width(ui.available_width());

        ui.card_title(title);
        ui.space_inline();
        ui.muted_label(description);
    });
}

fn mini_color(ui: &mut egui::Ui, name: &str, color: Color32) {
    ui.horizontal(|ui| {
        let side = metrics::EM;
        let (rect, _) = ui.allocate_exact_size(egui::vec2(side, side), egui::Sense::hover());
        ui.painter().rect_filled(rect, metrics::RADIUS_SM, color);
        ui.text(name, TextSize::Sm);
    });
}
