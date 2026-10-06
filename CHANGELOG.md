# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
While the crate is in the `0.x` series, breaking changes bump the minor version.

## [Unreleased]

### Added

- `theme::apply_with`, which installs the theme with custom dark and light
  palettes instead of the built-in ones. `theme::apply` is now a shorthand
  for it.
- `theme::rgb_hex`, a `const fn` that builds an opaque color from a
  `0xRRGGBB` literal, handy for defining a `Palette` as a constant.
- `gruvbox` example: two complete custom palettes installed with
  `theme::apply_with`.
- `UiLayouts` trait with `center_row`, a row of widgets centered horizontally
  in the available width.
- `InputWidth` is re-exported from the `theme` module.
- `Palette` and `rgb_hex` are re-exported from the `prelude` module.
- `Variant` derives `Debug`, `Clone`, `Copy`, `PartialEq` and `Eq`;
  `TextSize` and `TextColor` derive `PartialEq` and `Eq`.

### Changed

- The `quickstart` example uses a full-width email input and a centered row
  of buttons.

## [0.2.2] - 2026-10-06

### Added

- `InputWidth` enum (`Standard`, `Fixed`, `Fill`) to choose the width of a
  themed text input.
- `UiInputs::text_input_with_width` and `UiInputs::text_input_hint_with_width`,
  variants of the existing helpers that take an `InputWidth`.
- `UiText::app_title` and `UiText::page_title`, title helpers for the two
  largest text sizes.
- `UiText::text` and `UiText::text_colored`, generic helpers for any
  combination of `TextSize` and `TextColor`.

### Changed

- The `palette` example shows icons from `egui-phosphor`, added as a
  dev-dependency only: the library itself does not depend on any icon font.
- The README has a new "Icons" section on using icon fonts with the themed
  widgets.
- The `palette` example uses the `UiText` helpers instead of hand-built
  `RichText` labels.
- The `TextSize::Xl` and `TextSize::Xxl` docs now describe them as page and
  application title sizes, matching the new helpers.

## [0.2.1] - 2026-10-02

### Added

- `UiPanels` extension trait with `card`, `card_with_variant` and
  `interactive_card`, for themed containers that share the same border,
  corner radius and padding.
- `PanelVariant` enum (`Surface`, `SurfaceAlt`, `Elevated`) to select the fill
  and border colors of a card from the palette.

### Changed

- The `palette` example builds its cards with the new `UiPanels` methods.
- The README "Project status" section no longer names a specific version.

## [0.2.0] - 2026-09-29

### Added

- `StyledText` widget, which renders a label with a semantic size and color
  resolved against the active palette.
- `TextSize` and `TextColor` enums, including `TextSize::value()` and
  `TextColor::Custom` for explicit colors.
- `StyledButton` and `Variant` are re-exported from `theme`, next to the
  other public types.

### Changed

- **Breaking:** the `UiText` methods (`section_title`, `card_title`,
  `strong_label`, `muted_label`, `metadata_label`) now return `egui::Response`
  instead of `()`.
- **Breaking:** the `UiText` presets are built on `StyledText`: muted labels
  use the palette's `text_muted` color and titles use `text_strong`, instead
  of egui's weak and strong text styles.

## [0.1.0] - 2026-09-28

### Added

- `theme::apply` to install the modern dark and light styles on an egui
  context.
- `Palette` with semantic colors for both themes.
- Design tokens in `theme::metrics`, derived from a single `EM` base unit.
- `Ui` extension traits: `UiText`, `UiMetrics`, `UiButtons` and `UiInputs`.
- `StyledButton` widget with the `Variant` enum (primary, secondary, ghost,
  danger).
- `quickstart` and `palette` examples.

[Unreleased]: https://github.com/marc0x71/modern-egui/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/marc0x71/modern-egui/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/marc0x71/modern-egui/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/marc0x71/modern-egui/releases/tag/v0.1.0
