use eframe::egui;
use modern_egui::theme::{self, UiButtons, UiInputs, UiMetrics, UiText};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        // A compact window that fits the content below.
        viewport: egui::ViewportBuilder::default().with_inner_size([300.0, 155.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My app",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

#[derive(Default)]
struct MyApp {
    email: String,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Install the modern dark and light styles.
        theme::apply(&cc.egui_ctx);
        cc.egui_ctx.set_theme(egui::ThemePreference::Dark);
        Self::default()
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.section_title("Account");
            ui.muted_label("Manage your personal information.");
            ui.space_group();

            ui.text_input_hint(&mut self.email, "email@example.com");
            ui.space_inline();

            ui.horizontal(|ui| {
                if ui.primary_button("Save").clicked() { /* ... */ }
                if ui.ghost_button("Cancel").clicked() { /* ... */ }
                if ui.danger_button("Delete").clicked() { /* ... */ }
            });
        });
    }
}
