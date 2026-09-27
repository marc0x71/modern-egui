use eframe::egui::{self, Align, Button, Color32, Layout, RichText, Stroke, ThemePreference};

use modern_egui::theme::{self, Palette, UiButtons, UiInputs, UiMetrics, UiText, metrics};

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
        let p = Palette::of(ui.ctx());

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(
                    RichText::new("Modern egui")
                        .color(p.text_strong)
                        .size(metrics::FONT_2XL)
                        .strong(),
                );

                ui.label(
                    RichText::new("Theme & component showcase")
                        .size(metrics::FONT_SM)
                        .color(p.text_muted),
                );
            });

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let current_theme = ui.ctx().theme();

                if ui
                    .selectable_label(current_theme == egui::Theme::Dark, "🌙 Dark")
                    .clicked()
                {
                    ui.ctx().set_theme(ThemePreference::Dark);
                }

                if ui
                    .selectable_label(current_theme == egui::Theme::Light, "☀ Light")
                    .clicked()
                {
                    ui.ctx().set_theme(ThemePreference::Light);
                }

                ui.separator();

                ui.label(
                    RichText::new("Theme")
                        .size(metrics::FONT_XS)
                        .color(p.text_muted),
                );
            });
        });
    }

    // -------------------------------------------------------------------------
    // Palette
    // -------------------------------------------------------------------------

    fn palette_section(&mut self, ui: &mut egui::Ui) {
        let p = Palette::of(ui.ctx());

        section_title(ui, "Color palette");

        ui.label(
            RichText::new(
                "I colori vengono letti direttamente da Palette::of(ctx) \
                 e cambiano insieme al tema.",
            )
            .size(metrics::FONT_SM)
            .color(p.text_muted),
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

        let response = egui::Frame::new()
            .fill(p.surface)
            .stroke(Stroke::new(1.0, p.border))
            .corner_radius(metrics::RADIUS_MD)
            .inner_margin(metrics::SPACE_3)
            .show(ui, |ui| {
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
                        ui.label(
                            RichText::new(name)
                                .size(metrics::FONT_SM)
                                .strong()
                                .color(p.text_strong),
                        );

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
        let p = Palette::of(ui.ctx());

        section_title(ui, "Typography");

        typography_sample(
            ui,
            "2XL — Application title",
            metrics::FONT_2XL,
            p.text_strong,
        );

        typography_sample(ui, "XL — Heading", metrics::FONT_XL, p.text_strong);

        typography_sample(ui, "LG — Section title", metrics::FONT_LG, p.text_strong);

        typography_sample(ui, "CT — Card title", metrics::FONT_CT, p.text_strong);

        typography_sample(ui, "MD — Body text", metrics::FONT_MD, p.text);

        typography_sample(ui, "SM — Secondary text", metrics::FONT_SM, p.text_muted);

        typography_sample(ui, "XS — Metadata", metrics::FONT_XS, p.text_muted);

        ui.space_group();

        ui.horizontal_wrapped(|ui| {
            ui.label(
                RichText::new("Accent")
                    .size(metrics::FONT_MD)
                    .color(p.accent),
            );

            ui.separator();

            ui.label(
                RichText::new("Success")
                    .size(metrics::FONT_MD)
                    .color(p.success),
            );

            ui.separator();

            ui.label(
                RichText::new("Warning")
                    .size(metrics::FONT_MD)
                    .color(p.warning),
            );

            ui.separator();

            ui.label(
                RichText::new("Danger")
                    .size(metrics::FONT_MD)
                    .color(p.danger),
            );
        });
    }

    // -------------------------------------------------------------------------
    // Buttons
    // -------------------------------------------------------------------------

    fn buttons_section(&mut self, ui: &mut egui::Ui) {
        let p = Palette::of(ui.ctx());

        section_title(ui, "Buttons");

        ui.horizontal_wrapped(|ui| {
            let _ = ui.button("Default");
            let _ = ui.primary_button("Primary");
            let _ = ui.ghost_button("Ghost");
            let _ = ui.danger_button("Danger");

            ui.add_enabled(false, Button::new("Disabled"));
        });

        ui.space_group();

        ui.horizontal(|ui| {
            if ui.button("−").clicked() {
                self.counter -= 1;
            }

            ui.label(
                RichText::new(self.counter.to_string())
                    .strong()
                    .size(metrics::FONT_MD)
                    .color(p.text_strong),
            );

            if ui.button("+").clicked() {
                self.counter += 1;
            }
        });
    }

    // -------------------------------------------------------------------------
    // Inputs
    // -------------------------------------------------------------------------

    fn inputs_section(&mut self, ui: &mut egui::Ui) {
        let p = Palette::of(ui.ctx());

        section_title(ui, "Inputs");

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

        ui.label(
            RichText::new(
                "Prova focus, hover e click sugli input \
                 per controllare border e accent.",
            )
            .size(metrics::FONT_SM)
            .color(p.text_muted),
        );
    }

    // -------------------------------------------------------------------------
    // Selection
    // -------------------------------------------------------------------------

    fn selection_section(&mut self, ui: &mut egui::Ui) {
        let p = Palette::of(ui.ctx());

        section_title(ui, "Selection & navigation");

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

        ui.label(
            RichText::new(format!(
                "Selected tab: {}",
                ["Overview", "Activity", "Settings",][self.selected_tab],
            ))
            .size(metrics::FONT_SM)
            .color(p.text_muted),
        );

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

        section_title(ui, "Progress & semantic colors");

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
                ui.label(
                    RichText::new(text)
                        .size(metrics::FONT_SM)
                        .color(color)
                        .strong(),
                );
            });
    }

    // -------------------------------------------------------------------------
    // Surfaces
    // -------------------------------------------------------------------------

    fn surfaces_section(&mut self, ui: &mut egui::Ui) {
        let p = Palette::of(ui.ctx());

        section_title(ui, "Surface hierarchy");
        ui.label(
            RichText::new(
                "Confronto fra bg, surface, \
                 surface_alt ed elevated.",
            )
            .size(metrics::FONT_SM)
            .color(p.text_muted),
        );

        ui.space_group();

        if ui.available_width() >= metrics::EM * 50.0 {
            ui.columns(3, |columns| {
                surface_card(
                    &mut columns[0],
                    "Surface",
                    "Card principale sopra \
                     al background.",
                    p.surface,
                );

                surface_card(
                    &mut columns[1],
                    "Surface alt",
                    "Controlli, hover e \
                     superfici secondarie.",
                    p.surface_alt,
                );

                surface_card(
                    &mut columns[2],
                    "Elevated",
                    "Finestre, popup e \
                     contenuti sopraelevati.",
                    p.elevated,
                );
            });
        } else {
            surface_card(
                ui,
                "Surface",
                "Card principale sopra \
                 al background.",
                p.surface,
            );
            ui.space_group();

            surface_card(
                ui,
                "Surface alt",
                "Controlli, hover e \
                 superfici secondarie.",
                p.surface_alt,
            );
            ui.space_group();

            surface_card(
                ui,
                "Elevated",
                "Finestre, popup e \
                 contenuti sopraelevati.",
                p.elevated,
            );
        }
    }

    // -------------------------------------------------------------------------
    // Table
    // -------------------------------------------------------------------------

    fn table_section(&mut self, ui: &mut egui::Ui) {
        let p = Palette::of(ui.ctx());

        section_title(ui, "Table / striped rows");

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
                ui.label(RichText::new("Active").color(p.success));
                ui.label("68%");
                ui.end_row();

                ui.label("Dashboard");
                ui.label(RichText::new("Review").color(p.warning));
                ui.label("42%");
                ui.end_row();

                ui.label("Old UI");
                ui.label(RichText::new("Deprecated").color(p.danger));
                ui.label("100%");
                ui.end_row();

                ui.label("Documentation");
                ui.label(RichText::new("Draft").color(p.accent));
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
                sidebar_label(ui, "THEME");
                ui.space_inline();
                ui.label(
                    RichText::new(match ui.ctx().theme() {
                        egui::Theme::Dark => "Dark mode",
                        egui::Theme::Light => "Light mode",
                    })
                    .size(metrics::FONT_MD)
                    .color(p.text_strong),
                );
                ui.space_group();
                ui.separator();

                ui.space_group();
                sidebar_label(ui, "QUICK COLORS");
                ui.space_inline();
                mini_color(ui, "Accent", p.accent);
                mini_color(ui, "Success", p.success);
                mini_color(ui, "Warning", p.warning);
                mini_color(ui, "Danger", p.danger);
                ui.space_group();
                ui.separator();

                ui.space_group();
                sidebar_label(ui, "BACKGROUND");
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

fn section_title(ui: &mut egui::Ui, title: &str) {
    let p = Palette::of(ui.ctx());

    ui.label(
        RichText::new(title)
            .size(metrics::FONT_LG)
            .strong()
            .color(p.text_strong),
    );

    ui.space_inline();
}

fn sidebar_label(ui: &mut egui::Ui, text: &str) {
    let p = Palette::of(ui.ctx());

    ui.label(
        RichText::new(text)
            .size(metrics::FONT_XS)
            .strong()
            .color(p.text_muted),
    );
}

fn typography_sample(ui: &mut egui::Ui, text: &str, size: f32, color: Color32) {
    ui.label(RichText::new(text).size(size).color(color));
}

fn demo_card(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    let p = Palette::of(ui.ctx());

    egui::Frame::new()
        .fill(p.surface)
        .stroke(Stroke::new(1.0, p.border))
        .corner_radius(metrics::RADIUS_MD)
        .inner_margin(metrics::PANEL_PADDING)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());

            add_contents(ui);
        });
}

fn surface_card(ui: &mut egui::Ui, title: &str, description: &str, fill: Color32) {
    let p = Palette::of(ui.ctx());

    egui::Frame::new()
        .fill(fill)
        .stroke(Stroke::new(1.0, p.border))
        .corner_radius(metrics::RADIUS_MD)
        .inner_margin(metrics::PANEL_PADDING)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());

            ui.label(
                RichText::new(title)
                    .size(metrics::FONT_CT)
                    .strong()
                    .color(p.text_strong),
            );

            ui.space_inline();

            ui.label(
                RichText::new(description)
                    .size(metrics::FONT_SM)
                    .color(p.text_muted),
            );
        });
}

fn mini_color(ui: &mut egui::Ui, name: &str, color: Color32) {
    let p = Palette::of(ui.ctx());

    ui.horizontal(|ui| {
        let side = metrics::EM;
        let (rect, _) = ui.allocate_exact_size(egui::vec2(side, side), egui::Sense::hover());
        ui.painter().rect_filled(rect, metrics::RADIUS_SM, color);
        ui.label(RichText::new(name).size(metrics::FONT_SM).color(p.text));
    });
}
