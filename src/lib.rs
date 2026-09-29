//! A small modern theme and UI helper library for [egui].
//!
//! `modern-egui` gives an egui application a clean, contemporary look with a
//! single call to [`theme::apply`], and adds [`egui::Ui`] extension traits so
//! that typography, spacing, buttons and inputs stay consistent across the
//! whole interface.
//!
//! # Quick start
//!
//! ```no_run
//! use eframe::egui;
//! use modern_egui::theme::{self, UiButtons, UiMetrics, UiText};
//!
//! struct MyApp;
//!
//! impl MyApp {
//!     fn new(cc: &eframe::CreationContext<'_>) -> Self {
//!         // Install the modern dark and light styles once.
//!         theme::apply(&cc.egui_ctx);
//!         Self
//!     }
//! }
//!
//! impl eframe::App for MyApp {
//!     fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
//!         egui::CentralPanel::default().show(ui, |ui| {
//!             ui.section_title("Account");
//!             ui.muted_label("Manage your personal information.");
//!             ui.space_group();
//!             if ui.primary_button("Save").clicked() {
//!                 // ...
//!             }
//!         });
//!     }
//! }
//! ```
//!
//! # Overview
//!
//! Everything lives in the [`theme`] module:
//!
//! - [`theme::apply`] installs the theme on an [`egui::Context`];
//! - [`theme::Palette`] holds the semantic colors of the dark and light themes;
//! - [`theme::metrics`] defines the design tokens (font sizes, spacing,
//!   control sizes, corner radii, layout sizes);
//! - [`theme::UiText`], [`theme::UiMetrics`], [`theme::UiButtons`] and
//!   [`theme::UiInputs`] extend [`egui::Ui`] with semantic helpers;
//! - [`theme::StyledButton`] and [`theme::StyledText`] are the themed widgets
//!   behind the [`theme::UiButtons`] and [`theme::UiText`] helpers.
//!
//! The crate depends only on `egui`, so it works with any egui integration,
//! not just eframe.
//!
//! [egui]: https://github.com/emilk/egui

pub mod theme;
