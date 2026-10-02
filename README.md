# modern-egui

[![Crates.io](https://img.shields.io/crates/v/modern-egui.svg)](https://crates.io/crates/modern-egui)
[![Documentation](https://docs.rs/modern-egui/badge.svg)](https://docs.rs/modern-egui)
[![License](https://img.shields.io/crates/l/modern-egui.svg)](LICENSE-MIT)

A small modern theme and UI helper library for [egui](https://github.com/emilk/egui).

`modern-egui` gives your egui application a clean, contemporary look with a single
function call, and adds a handful of `Ui` extension methods so that typography,
spacing, buttons, inputs and cards stay consistent across the whole interface.

<img width="1900" height="1324" alt="screenshot-20260928-095816" src="https://github.com/user-attachments/assets/c5e42447-f606-4487-9783-e9a4ca3ce8d8" />

## Features

- **One-line setup**: `theme::apply(ctx)` configures both the dark and the light theme.
- **Semantic color palette**: backgrounds, surfaces, borders, text, accent, danger,
  success and warning colors, tuned separately for dark and light mode
  (the light accent meets WCAG AA contrast for text on accent).
- **Design tokens**: a single `EM` base unit drives the font scale, spacing scale,
  control sizes, corner radii and layout sizes.
- **`Ui` extension traits**: semantic helpers such as `ui.section_title(..)`,
  `ui.space_section()`, `ui.primary_button(..)` and `ui.text_input_hint(..)`.
- **Styled buttons**: four button variants (primary, secondary, ghost, danger)
  available both as one-line helpers and as a `StyledButton` widget with the
  most common `egui::Button` builder options.
- **Styled text**: a `StyledText` widget that combines the typography scale
  with semantic text colors, used by the `UiText` helpers.
- **Cards**: themed containers with the shared border, radius and padding,
  in three surface levels plus a variant that highlights itself on hover.
- **Automatic theme switching**: helpers read the active palette at draw time,
  so they follow `ctx.set_theme(..)` without extra code.

## Installation

Add `modern-egui` to your `Cargo.toml` next to eframe:

```toml
[dependencies]
eframe = "0.36"
modern-egui = "0.2"
```

Or with cargo:

```sh
cargo add eframe modern-egui
```

`modern-egui` currently targets **eframe/egui 0.36** and requires Rust 1.95 or newer (same MSRV as egui 0.36).

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

**`UiText`** — typography roles (all return `egui::Response`)

| Method                 | Size                    | Color    |
|------------------------|-------------------------|----------|
| `section_title(text)`  | `Lg` (`FONT_LG`)        | `Strong` |
| `card_title(text)`     | `CardTitle` (`FONT_CT`) | `Strong` |
| `strong_label(text)`   | `Md` (`FONT_MD`)        | `Strong` |
| `muted_label(text)`    | `Sm` (`FONT_SM`)        | `Muted`  |
| `metadata_label(text)` | `Xs` (`FONT_XS`)        | `Muted`  |

These are presets built on top of [`StyledText`](#styled-text): use the
widget directly when you need a different combination.

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

| Method                   | Use for                                               |
|--------------------------|-------------------------------------------------------|
| `primary_button(text)`   | The main action of a view                             |
| `secondary_button(text)` | Ordinary actions: filled, but quiet                   |
| `ghost_button(text)`     | Tertiary actions, no fill until hovered (toolbars, lists) |
| `danger_button(text)`    | Destructive actions                                   |

`text` accepts anything that implements `egui::IntoAtoms`, the same input as
`egui::Button::new`, so a plain `&str` or a `RichText` both work.

**`UiInputs`** — fixed-size single-line text fields (`INPUT_WIDTH` × `INPUT_HEIGHT`)

| Method                        | Description                         |
|-------------------------------|-------------------------------------|
| `text_input(&mut text)`       | Plain text field                    |
| `text_input_hint(&mut text, hint)` | Text field with placeholder    |

**`UiPanels`** — cards for grouping related content

| Method                                     | Description                                        |
|--------------------------------------------|----------------------------------------------------|
| `card(add_contents)`                       | Card with the default `Surface` background         |
| `card_with_variant(variant, add_contents)` | Card with the colors of the given `PanelVariant`   |
| `interactive_card(add_contents)`           | `Surface` card that turns `Elevated` while hovered |

All cards share a 1 px border, `RADIUS_MD` corners and `PANEL_PADDING` inner
margin. `PanelVariant` selects the fill from the palette, together with a
matching border color:

| Variant      | Fill          | Use for                                   |
|--------------|---------------|-------------------------------------------|
| `Surface`    | `surface`     | Regular cards above the window background |
| `SurfaceAlt` | `surface_alt` | Nested or less prominent content          |
| `Elevated`   | `elevated`    | Content that must stand out               |

```rust
use modern_egui::theme::{UiPanels, UiText};
use modern_egui::theme::ui_ext::PanelVariant;

ui.card(|ui| {
    ui.card_title("Profile");
    ui.muted_label("Visible to other users.");
});

ui.card_with_variant(PanelVariant::SurfaceAlt, |ui| {
    ui.muted_label("Nothing to show yet.");
});
```

Every method returns an `egui::InnerResponse`: the value produced by the
closure plus the `Response` of the whole card. A card is only as large as its
content; call `ui.set_min_width(ui.available_width())` inside the closure to
make it fill the available width.

`interactive_card` only senses hovering. To make it clickable, upgrade its
response:

```rust
let card = ui.interactive_card(|ui| {
    ui.card_title("Open project");
});

if card.response.interact(egui::Sense::click()).clicked() {
    // handle the click
}
```

### Styled buttons

The `UiButtons` helpers are shortcuts for the `StyledButton` widget. Use the
widget directly when you need to customize the button: it offers the
`egui::Button` builder methods that do not conflict with the variant colors
(`min_size`, `small`, `corner_radius`,
`sense`, `selected`, `shortcut_text`, `wrap`, `truncate`, `gap`) and applies
the variant colors when it is added to the `Ui`.

```rust
use modern_egui::theme::{metrics, StyledButton};

// A wider primary button, e.g. for a dialog footer.
let save = ui.add(
    StyledButton::primary("Save")
        .min_size(egui::vec2(120.0, metrics::CONTROL_HEIGHT)),
);

// A ghost button used as a toggle in a toolbar.
if ui.add(StyledButton::ghost("Grid").selected(self.grid_view)).clicked() {
    self.grid_view = !self.grid_view;
}
```

Every variant has its own constructor (`primary`, `secondary`, `ghost`,
`danger`); `StyledButton::new(text, Variant::…)` is also available when the
variant is chosen at runtime. Buttons use `RADIUS_SM` corners by default.

### Styled text

`StyledText` renders a label with a semantic size and color from the theme.
Semantic colors are resolved against the active `Palette` when the widget is
drawn, so the text follows theme switches automatically.

```rust
use modern_egui::theme::{StyledText, TextColor, TextSize};

ui.add(
    StyledText::new("Settings")
        .size(TextSize::Lg)
        .color(TextColor::Strong),
);

// Semantic colors work well for inline status messages.
ui.add(StyledText::new("Saved").size(TextSize::Sm).color(TextColor::Success));
```

By default the text uses `TextSize::Md` and `TextColor::Text`.

**`TextSize`** maps to the font-size tokens in `metrics`:

| Variant     | Token       | Intended for                     |
|-------------|-------------|----------------------------------|
| `Xs`        | `FONT_XS`   | Metadata, low-emphasis details   |
| `Sm`        | `FONT_SM`   | Secondary or supporting content  |
| `Md`        | `FONT_MD`   | Body text (default)              |
| `CardTitle` | `FONT_CT`   | Card and panel titles            |
| `Lg`        | `FONT_LG`   | Section titles                   |
| `Xl`        | `FONT_XL`   | Headings                         |
| `Xxl`       | `FONT_2XL`  | Page or application titles       |

`TextSize::value()` returns the size in points, if you need it elsewhere.

**`TextColor`** selects a palette role: `Text` (default), `Strong`, `Muted`,
`Accent`, `Success`, `Warning` and `Danger`. `Custom(Color32)` uses an
explicit color instead, which does not change with the theme.

## Examples

The repository ships with two runnable examples:

```sh
# The Quick start application shown above
cargo run --example quickstart

# A showcase of the palette, typography, buttons, inputs, status badges,
# surfaces and tables, in both dark and light mode
cargo run --example palette
```

## Project status

`modern-egui` is at an early stage: the API may change between releases.
Feedback and suggestions are welcome through
[issues](https://github.com/marc0x71/modern-egui/issues).

## License

`modern-egui` is dual-licensed under the [MIT License](LICENSE-MIT) and the [Apache License 2.0](LICENSE-APACHE). You may choose either.

## A note about AI

I used AI to help me write some of the tests and documentation for this project. Writing tests can be a bit tedious, and English isn't my native language, so AI has been a useful tool to speed things up and improve the documentation.

I still review, adapt, and run the generated tests, but I prefer to be transparent about how AI was used in this project.
