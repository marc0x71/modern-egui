# modern-egui

[![Crates.io](https://img.shields.io/crates/v/modern-egui.svg)](https://crates.io/crates/modern-egui)
[![Documentation](https://docs.rs/modern-egui/badge.svg)](https://docs.rs/modern-egui)
[![License](https://img.shields.io/crates/l/modern-egui.svg)](LICENSE-MIT)

A small modern theme and UI helper library for [egui](https://github.com/emilk/egui).

`modern-egui` gives your egui application a clean, contemporary look with a single
function call, and adds a handful of `Ui` extension methods so that typography,
spacing, buttons, inputs and cards stay consistent across the whole interface.

<p>
<img width="49%" alt="modern-egui dark theme" src="https://github.com/user-attachments/assets/dd67c716-bb51-45a9-8844-29d9ec9483f8" />
<img width="49%" alt="modern-egui light theme" src="https://github.com/user-attachments/assets/f544b3ff-8cec-41a6-b95e-ab5465d76120" />
</p>

## Features

- **One-line setup**: `theme::apply(ctx)` configures both the dark and the light theme.
- **Semantic color palette**: backgrounds, surfaces, borders, text, accent, danger,
  success and warning colors, tuned separately for dark and light mode
  (the light accent meets WCAG AA contrast for text on accent).
- **Custom palettes**: `theme::apply_with(ctx, dark, light)` installs your own
  colors in place of the built-in ones, for both the themed helpers and
  egui's own widgets.
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
- **Layout helpers**: arrangements that egui does not offer out of the box,
  such as a row of widgets centered horizontally.
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
use modern_egui::theme::{self, InputWidth, UiButtons, UiInputs, UiLayouts, UiMetrics, UiText};

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

            ui.text_input_hint_with_width(&mut self.email, "email@example.com", InputWidth::Fill);
            ui.space_inline();

            ui.center_row("my_buttons", |ui| {
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

To use your own colors, call `theme::apply_with(&ctx, dark, light)` instead:
it does the same three things with the palettes you pass in. See
[Custom palettes](#custom-palettes).

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
if neither `theme::apply` nor `theme::apply_with` has been called.

#### Custom palettes

A `Palette` is a plain struct with public fields, so a custom one can start
from a built-in palette and override only some colors:

```rust
use modern_egui::theme::{self, Palette, rgb_hex};

let dark = Palette {
    accent: rgb_hex(0x689d6a),
    accent_hover: rgb_hex(0x8ec07c),
    accent_active: rgb_hex(0x427b58),
    on_accent: rgb_hex(0x282828),
    ..Palette::dark()
};

theme::apply_with(&cc.egui_ctx, dark, Palette::light());
```

`rgb_hex` is a `const fn`, so a whole palette can also be declared as a
`const`. When you change `accent` or `danger`, set the matching `*_hover`,
`*_active` and `on_*` colors too: they are not derived automatically.

`apply_with` can be called again later to switch palettes at runtime. It
reinstalls the metrics as well, so any change made to them is overwritten.

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
| `app_title(text)`      | `Xxl` (`FONT_2XL`)      | `Strong` |
| `page_title(text)`     | `Xl` (`FONT_XL`)        | `Strong` |
| `section_title(text)`  | `Lg` (`FONT_LG`)        | `Strong` |
| `card_title(text)`     | `CardTitle` (`FONT_CT`) | `Strong` |
| `strong_label(text)`   | `Md` (`FONT_MD`)        | `Strong` |
| `muted_label(text)`    | `Sm` (`FONT_SM`)        | `Muted`  |
| `metadata_label(text)` | `Xs` (`FONT_XS`)        | `Muted`  |

These are presets built on top of [`StyledText`](#styled-text). For any other
combination of size and color, two generic helpers take them as arguments:

| Method                            | Size    | Color           |
|-----------------------------------|---------|-----------------|
| `text(text, size)`                | `size`  | `Text`          |
| `text_colored(text, size, color)` | `size`  | `color`         |

```rust
use modern_egui::theme::{TextColor, TextSize, UiText};

ui.text("Last sync: 2 minutes ago", TextSize::Sm);
ui.text_colored("Connected", TextSize::Md, TextColor::Success);
```

Use the `StyledText` widget directly when you need more control.

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

**`UiInputs`** — single-line text fields with the standard `INPUT_HEIGHT`

| Method                                               | Description                                            |
|------------------------------------------------------|--------------------------------------------------------|
| `text_input(&mut text)`                              | Plain text field, `INPUT_WIDTH` wide                   |
| `text_input_with_width(&mut text, width)`            | Plain text field with the given `InputWidth`           |
| `text_input_hint(&mut text, hint)`                   | Text field with placeholder, `INPUT_WIDTH` wide        |
| `text_input_hint_with_width(&mut text, hint, width)` | Text field with placeholder and the given `InputWidth` |

`InputWidth` selects the width of the field:

| Variant      | Width                                         |
|--------------|-----------------------------------------------|
| `Standard`   | `INPUT_WIDTH`, the default                    |
| `Fixed(f32)` | An explicit width in logical points           |
| `Fill`       | All the width available in the current layout |

```rust
use modern_egui::theme::{InputWidth, UiInputs};

ui.text_input_with_width(&mut self.zip_code, InputWidth::Fixed(80.0));
ui.text_input_hint_with_width(&mut self.search, "Search...", InputWidth::Fill);
```

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
use modern_egui::theme::{UiPanels, UiText, PanelVariant};

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

**`UiLayouts`** — arrangements of several widgets

| Method                              | Description                                     |
|-------------------------------------|-------------------------------------------------|
| `center_row(id_salt, add_contents)` | Row of widgets centered in the available width  |

```rust
use modern_egui::theme::{UiButtons, UiLayouts};

ui.center_row("dialog_actions", |ui| {
    if ui.primary_button("Save").clicked() { /* ... */ }
    if ui.ghost_button("Cancel").clicked() { /* ... */ }
});
```

`center_row` returns an `egui::InnerResponse`: the value produced by the
closure plus the `Response` of the row content. `id_salt` must be unique among
the centered rows of the same `Ui`. If the content is wider than the available
width, the row starts at the left edge.

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
| `Xl`        | `FONT_XL`   | Page titles                      |
| `Xxl`       | `FONT_2XL`  | Application titles               |

`TextSize::value()` returns the size in points, if you need it elsewhere.

**`TextColor`** selects a palette role: `Text` (default), `Strong`, `Muted`,
`Accent`, `Success`, `Warning` and `Danger`. `Custom(Color32)` uses an
explicit color instead, which does not change with the theme.

### Icons

`modern-egui` does not depend on any icon font. In egui an icon is just a
character drawn with a font that contains its glyph, so any icon font
registered as a fallback of the `Proportional` family works with the themed
buttons and labels.

For example, with [egui-phosphor](https://crates.io/crates/egui-phosphor):

```toml
[dependencies]
egui-phosphor = "0.14"
```

Register the font once, in your app constructor:

```rust
let mut fonts = egui::FontDefinitions::default();
egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
cc.egui_ctx.set_fonts(fonts);

theme::apply(&cc.egui_ctx);
```

Then use the icon constants in the text of buttons and labels:

```rust
use egui_phosphor::regular as ph;

// Icon and text in the same string.
ui.danger_button(format!("{} Delete", ph::TRASH));

// Icon and text as separate atoms, spaced by the button gap.
ui.primary_button((ph::FLOPPY_DISK, "Save"));
ui.secondary_button(("Next", ph::ARROW_RIGHT));

// Icon only.
ui.ghost_button(ph::X);

// Labels take a single string.
ui.muted_label(format!("{} Saved", ph::CHECK));
```

A few things to keep in mind:

- Separate atoms work with the widgets built on `egui::Button` (buttons,
  checkboxes, radio buttons, selectable labels). Labels and the `UiText`
  helpers take a single string, so put the icon in it with `format!`.
- If you also install a text font, add it to the same `FontDefinitions`
  before calling `set_fonts`: a second call replaces the first one.
- An icon has the same size as the text around it. Wrap it in a `RichText`
  to change it, e.g. `egui::RichText::new(ph::X).size(metrics::FONT_LG)`.
- The fallback is registered for the `Proportional` family only, so icons do
  not show up in monospace text.

## Examples

The repository ships with three runnable examples:

```sh
# The Quick start application shown above
cargo run --example quickstart

# A showcase of the palette, typography, buttons, inputs, status badges,
# surfaces and tables, in both dark and light mode
cargo run --example palette

# Two custom Gruvbox-inspired palettes installed with theme::apply_with
cargo run --example gruvbox
```

The `palette` example also shows icons from `egui-phosphor`, which is a
dev-dependency only.

## Project status

`modern-egui` is at an early stage: the API may change between releases.
Feedback and suggestions are welcome through
[issues](https://github.com/marc0x71/modern-egui/issues).

## License

`modern-egui` is dual-licensed under the [MIT License](LICENSE-MIT) and the [Apache License 2.0](LICENSE-APACHE). You may choose either.

## A note about AI

I used AI to help me write some of the tests and documentation for this project. Writing tests can be a bit tedious, and English isn't my native language, so AI has been a useful tool to speed things up and improve the documentation.

I still review, adapt, and run the generated tests, but I prefer to be transparent about how AI was used in this project.
