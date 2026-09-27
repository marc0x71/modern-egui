# modern-egui

A small modern theme and UI helper library for [egui](https://github.com/emilk/egui).

`modern-egui` gives your egui application a clean, contemporary look with a single
function call, and adds a handful of `Ui` extension methods so that typography,
spacing, buttons and inputs stay consistent across the whole interface.

<img width="1897" height="1326" alt="screenshot-20260927-195742" src="https://github.com/user-attachments/assets/24fc25ee-01b6-4c59-a557-d6d6e2fe89aa" />

## Features

- **One-line setup**: `theme::apply(ctx)` configures both the dark and the light theme.
- **Semantic color palette**: backgrounds, surfaces, borders, text, accent, danger,
  success and warning colors, tuned separately for dark and light mode
  (the light accent meets WCAG AA contrast for text on accent).
- **Design tokens**: a single `EM` base unit drives the font scale, spacing scale,
  control sizes, corner radii and layout sizes.
- **`Ui` extension traits**: semantic helpers such as `ui.section_title(..)`,
  `ui.space_section()`, `ui.primary_button(..)` and `ui.text_input_hint(..)`.
- **Automatic theme switching**: helpers read the active palette at draw time,
  so they follow `ctx.set_theme(..)` without extra code.

## Installation

The crate is not published on crates.io yet. Add it as a git dependency:

```toml
[dependencies]
eframe = "0.36"
modern-egui = { git = "https://github.com/marc0x71/modern-egui" }
```

`modern-egui` currently targets **eframe/egui 0.36** and Rust **edition 2024**.

## Quick start

```rust
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
```

## Usage

### Applying the theme

`theme::apply(&ctx)` should be called once, typically in your app constructor. It:

1. stores the dark and light `Palette` in the egui context memory;
2. maps the palette onto egui's `Visuals` (widget states, panels, windows,
   selection, hyperlinks, text cursor, error and warning colors);
3. applies the typography and spacing metrics to every style.

After that you can switch themes as usual with `ctx.set_theme(..)`.

### Palette

`Palette` exposes the semantic colors of the current theme:

| Group      | Fields                                                        |
|------------|---------------------------------------------------------------|
| Surfaces   | `bg`, `surface`, `surface_alt`, `elevated`                    |
| Borders    | `border`, `border_strong`                                     |
| Text       | `text`, `text_strong`, `text_muted`                           |
| Accent     | `accent`, `accent_hover`, `accent_active`, `on_accent`        |
| Danger     | `danger`, `danger_hover`, `danger_active`, `on_danger`        |
| Status     | `success`, `warning`                                          |

Read the palette of the active theme inside your UI code:

```rust
let p = theme::Palette::of(ui.ctx());
ui.colored_label(p.success, "Connected");
```

You can also get a specific palette with `Palette::dark()`, `Palette::light()`
or `Palette::for_theme(theme)`. `Palette::of` falls back to the built-in palette
if `theme::apply` has not been called.

### Metrics

All design tokens live in `theme::metrics` and derive from `EM = 14.0`:

| Category   | Constants                                                                 |
|------------|---------------------------------------------------------------------------|
| Font sizes | `FONT_XS`, `FONT_SM`, `FONT_MD`, `FONT_CT`, `FONT_LG`, `FONT_XL`, `FONT_2XL` |
| Spacing    | `SPACE_1` … `SPACE_6`                                                     |
| Gaps       | `GAP_INLINE`, `GAP_GROUP`, `GAP_SECTION`, `GAP_PAGE`                       |
| Controls   | `CONTROL_HEIGHT`, `CONTROL_HEIGHT_LG`, `BUTTON_PAD_X`, `BUTTON_PAD_Y`, `INPUT_HEIGHT`, `INPUT_WIDTH` |
| Radius     | `RADIUS_SM`, `RADIUS_MD`, `RADIUS_LG`                                      |
| Layout     | `PANEL_PADDING`, `SIDEBAR_WIDTH`, `SIDEBAR_MIN_WIDTH`, `SIDEBAR_MAX_WIDTH` |

```rust
use modern_egui::theme::metrics;

egui::Panel::left("nav")
    .resizable(true)
    .default_size(metrics::SIDEBAR_WIDTH)
    .size_range(metrics::SIDEBAR_MIN_WIDTH..=metrics::SIDEBAR_MAX_WIDTH)
    .show(ui, |ui| { /* ... */ });
```

### `Ui` extensions

Bring the traits into scope to use the helpers on any `egui::Ui`.

**`UiText`** — typography roles

| Method                 | Result                                   |
|------------------------|------------------------------------------|
| `section_title(text)`  | Large, bold title (`FONT_LG`)            |
| `card_title(text)`     | Bold card heading (`FONT_CT`)            |
| `strong_label(text)`   | Body-size text in `text_strong` color    |
| `muted_label(text)`    | Small, weak secondary text (`FONT_SM`)   |
| `metadata_label(text)` | Extra-small, weak text (`FONT_XS`)       |

**`UiMetrics`** — vertical spacing

| Method                                           | Space added      |
|--------------------------------------------------|------------------|
| `space1()` … `space6()`                          | `SPACE_1` … `SPACE_6` |
| `space_inline()`                                 | `GAP_INLINE`     |
| `space_group()`                                  | `GAP_GROUP`      |
| `space_section()`                                | `GAP_SECTION`    |
| `space_page()`                                   | `GAP_PAGE`       |

Prefer the semantic methods (`space_group`, `space_section`, …) over the raw scale:
they describe *why* the space is there.

**`UiButtons`** — button variants (all return `egui::Response`)

| Method                | Use for                                  |
|-----------------------|------------------------------------------|
| `primary_button(text)`| The main action of a view                |
| `ghost_button(text)`  | Secondary, low-emphasis actions          |
| `danger_button(text)` | Destructive actions                      |

**`UiInputs`** — fixed-size single-line text fields (`INPUT_WIDTH` × `INPUT_HEIGHT`)

| Method                        | Description                         |
|-------------------------------|-------------------------------------|
| `text_input(&mut text)`       | Plain text field                    |
| `text_input_hint(&mut text, hint)` | Text field with placeholder    |

## Examples

The repository ships with a runnable showcase of the palette, typography,
buttons, inputs, status badges, surfaces and tables, in both dark and light mode:

```sh
cargo run --example palette
```

## Project status

`modern-egui` is at an early stage (`0.1.0`): the API may change between releases.
Feedback and suggestions are welcome through
[issues](https://github.com/marc0x71/modern-egui/issues).

## License

`modern-egui` is dual-licensed under the [MIT License](LICENSE-MIT) and the [Apache License 2.0](LICENSE-APACHE). You may choose either.

## A note about AI

I used AI to help me write some of the tests and documentation for this project. Writing tests can be a bit tedious, and English isn't my native language, so AI has been a useful tool to speed things up and improve the documentation.

I still review, adapt, and run the generated tests, but I prefer to be transparent about how AI was used in this project.
