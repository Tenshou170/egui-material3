# egui-material3

A Material Design 3 component library for [egui](https://github.com/emilk/egui), providing
MD3-styled widgets with comprehensive theming support.

> [!IMPORTANT]  
> **This is a fork** of [nikescar/egui-material3](https://github.com/nikescar/egui-material3), maintained as a dependency for [Hachimi-Edge](https://github.com/Tenshou170/Hachimi-Edge).
> It contains layout fixes, new components, and MD3 compliance improvements not yet in upstream.

## Changes from upstream (v0.0.10)

- **New: `MaterialNavigationRail`** — vertical destination rail with icon pill indicators, labels, and custom widths, conforming to the 2024 MD3 spec
- **New: `MaterialTextField`** — MD3 filled/outlined text input with focus indicator, correct inner margins, and reliable `desired_width` inside frame closures
- **New: `MaterialNumberField`** — numeric drag-and-type field with `.range()`, `.decimals()`, `.speed()`, `.suffix()`
- **`MaterialTabs`** — added `.width(f32)` for explicit container width; `.compact()` mode; background fill covers full width edge-to-edge in fixed mode
- **`MaterialSelect`** — added `.small()` (28dp); default width uses `available_width()`; displayed text clips before chevron; fixed dropdown z-order
- **`MaterialSlider`** — MD3-spec state-layer (20dp, 8%/12% alpha); corrected track (4dp) and thumb (20dp) dimensions; state-layer drawn behind track
- **`MaterialSwitch`** — off-state thumb size adjusted to 20dp for visual balance, and added M3-spec 40dp interaction/state-layer ripple
- **`MaterialSnackbar`** — fixed `inverseOnSurface` color key; persistent show timer; responsive width; corrected asymmetric padding when actions are omitted
- **`MaterialProgress`** — linear/circular determinate and indeterminate variants conforming to the 2024 MD3 spec, featuring custom track gaps and togglable/resizable stop indicators
- **`MaterialCheckbox`** / **`MaterialRadio`** — MD3 density and touch target sizing
- **`theme.rs`** — `get/set_global_corner_radius()` API; consistent surface role application

---

## Installation

You can use this crate either locally or using pinned git revision:

Local:

```toml
[dependencies]
egui-material3 = { path = "../egui-material3", default-features = false }
```

Pinned Git Revision:
```toml
[dependencies]
egui-material3 = { git = "https://github.com/Tenshou170/egui-material3.git", rev = "cce5f3a0a4df85b151c70c772161a57736a6181c", default-features = false } 
```

For the upstream published version:

```bash
cargo add egui-material3
```

---

## Quick Start

```rust
use egui_material3::{
    MaterialButton, MaterialCheckbox, MaterialSlider, MaterialSelect, SelectVariant,
    MaterialTextField, MaterialNumberField, MaterialTabs, tabs::tabs_primary,
    theme::get_global_color, MaterialNavigationRail, NavRailItem,
};
```

### Buttons

```rust
// Filled (primary action)
ui.add(MaterialButton::filled("Save"));

// Outlined (secondary action)
ui.add(MaterialButton::outlined("Cancel"));

// Text (tertiary / low-emphasis)
ui.add(MaterialButton::text("Learn more"));

// Filled tonal (mid-emphasis)
ui.add(MaterialButton::filled_tonal("Continue"));

// With selected state (toggle)
ui.add(MaterialButton::filled_tonal("Dark").selected(is_dark));

// Truncate long label text instead of overflowing
ui.add(MaterialButton::text("Restore Defaults").truncate());
```

### Text fields

```rust
// Filled variant (default) — MD3 56dp height, surfaceContainerHighest background
ui.add(MaterialTextField::filled(&mut my_string));

// With hint and focus lock (for URLs, paths, etc.)
let res = ui.add(
    MaterialTextField::filled(&mut config.meta_index_url)
        .lock_focus(true)
);

// Explicit width (e.g. inside a horizontal layout)
ui.add(MaterialTextField::filled(&mut text).width(200.0));

// Outlined variant
ui.add(MaterialTextField::outlined(&mut text));

// Numeric field — drag to change, or type directly
ui.add(MaterialNumberField::filled(&mut my_f32)
    .range(0.0..=100.0)
    .decimals(2));

// With suffix label
ui.add(MaterialNumberField::filled(&mut timeout_ms)
    .range(100..=30000)
    .speed(100.0)
    .suffix("ms"));
```

### Select / Dropdown

```rust
let mut selected: Option<usize> = Some(0);

// Fills available column width by default
let mut select = MaterialSelect::new(&mut selected)
    .variant(SelectVariant::Outlined)
    .placeholder("Choose...")
    .small();     // compact 28dp height for grid use

for (i, label) in choices.iter().enumerate() {
    select = select.option(i, label);
}
ui.add(select);

// When inside a grid cell, capture column width BEFORE any horizontal layout:
let col_w = ui.available_width();
ui.add(
    MaterialSelect::new(&mut selected)
        .variant(SelectVariant::Outlined)
        .width(col_w)
        .small()
);
```

### Slider

```rust
// MD3 slider — primary-colored track, round thumb, state-layer ripple
ui.add(
    MaterialSlider::new(&mut value, 0.0..=1.0)
        .show_value(false)
        .show_value_indicator(false)
        .width(available_w)
);

// With step and value indicator
ui.add(
    MaterialSlider::new(&mut value, 0.0..=20.0)
        .step(0.5)
        .show_value_indicator(true)
);
```

### Tabs

```rust
let mut tab_idx: usize = 0;

// Fixed equal-width tabs (MD3 "fixed tabs" spec)
// .width() overrides available_width() for reliable edge-to-edge rendering
ui.add(
    tabs_primary(&mut tab_idx)
        .tab("General")
        .tab("Graphics")
        .tab("Gameplay")
        .tab("Advanced")
        .width(ui.max_rect().width())  // pass window inner width explicitly
        .compact()
        .id_salt("my_tabs"),
);

// Scrollable tabs (MD3 "scrollable tabs" spec) — wrap in horizontal ScrollArea
egui::ScrollArea::horizontal().show(ui, |ui| {
    ui.add(
        tabs_primary(&mut tab_idx)
            .tab("Tab 1")
            .tab("Tab 2")
            .tab("Tab 3")
            .scrollable()
            .id_salt("scrollable_tabs"),
    );
});
```

### Checkbox

```rust
ui.add(MaterialCheckbox::new(&mut enabled, "Enable feature"));

// Disabled state
ui.add(MaterialCheckbox::new(&mut value, "").enabled(false));
```

### Navigation Rail

```rust
let mut selected = 0;
let items = &[
    NavRailItem::new("\u{e8b9}", "General"),
    NavRailItem::new("\u{eb97}", "Graphics"),
    NavRailItem::new("\u{e30f}", "Gameplay"),
    NavRailItem::new("\u{e869}", "Advanced"),
];

// Display the vertical navigation rail
let (response, changed) = MaterialNavigationRail::new(&mut selected, items)
    .width(80.0) // optional width override
    .show(ui);
```

---

## Theme System

The theme system implements the full MD3 color scheme via HCT color space.

### Applying a theme

```rust
use egui_material3::theme::get_global_color;

// Retrieve any MD3 color role at render time
let primary       = get_global_color("primary");
let surface       = get_global_color("surface");
let on_surface    = get_global_color("onSurface");
let surface_cont  = get_global_color("surfaceContainer");
```

### MD3 color roles used by this fork

| Role | Used for |
|---|---|
| `primary` | Active slider track, tab indicator, focused field border, active navigation rail label |
| `onPrimary` | Text/icon on primary-colored surfaces |
| `surfaceContainer` | Tab bar background, navigation rail background |
| `surfaceContainerHighest` | Text field background (filled variant) |
| `onSurface` | Label text, time display |
| `onSurfaceVariant` | Unselected tab labels, inactive field bottom line, inactive navigation rail elements |
| `outline` | Outlined field border (unfocused) |
| `outlineVariant` | Tab divider line |
| `secondaryContainer` | Selected dropdown item background, active navigation rail indicator pill fill |
| `onSecondaryContainer` | Active navigation rail icon color |

---

## Available Components

### Input & Selection
- **`MaterialButton`** — filled, outlined, text, filled_tonal, elevated variants; `.selected()`, `.truncate()`
- **`MaterialCheckbox`** — MD3 checkbox with enabled/disabled state
- **`MaterialSwitch`** — toggle switch with visual size and ripple improvements *(fork addition)*
- **`MaterialRadio`** / **`MaterialRadioGroup`** — radio buttons with list tile support
- **`MaterialSlider`** / **`MaterialRangeSlider`** — primary-colored track, round/handle thumb, ripple
- **`MaterialSelect`** — filled/outlined dropdown; auto-width, text clipping, `.small()` mode
- **`MaterialTextField`** — filled/outlined text input; MD3 56dp height, focus indicator *(fork addition)*
- **`MaterialNumberField`** — numeric drag-and-type field with range, decimals, suffix *(fork addition)*
- **`MaterialChip`** — filter, assist, input, suggestion chips

### Navigation
- **`MaterialTabs`** — primary/secondary variants; fixed equal-width and scrollable modes; `.width()` *(fork addition)*
- **`MaterialNavigationRail`** — vertical destination rail with icon pill indicators and labels *(fork addition)*
- **`MaterialDrawer`** — permanent, dismissible, modal, standard
- **`MaterialTopAppBar`** — standard, center-aligned, medium, large
- **`MaterialToolbar`** — flexible toolbar with action items
- **`MaterialBreadcrumbs`** — breadcrumb navigation
- **`MaterialMenu`** — context menus with nested support

### Feedback
- **`MaterialDialog`** — modal dialogs
- **`MaterialSnackbar`** — toast notifications with optional actions
- **`MaterialNotification`** — notification cards
- **`MaterialBadge`** — count/status badge indicators
- **`MaterialProgress`** — circular and linear progress indicators
- **`MaterialTooltip`** — contextual tooltips
- **`MaterialActionSheet`** — bottom sheet for action selection

### Data Display
- **`MaterialCard2`** — elevated, filled, outlined
- **`MaterialList`** — MD3 list with visual density control
- **`MaterialDataTable`** — sortable, selectable data table
- **`MaterialTimeline`** — chronological event display
- **`MaterialTreeView`** — hierarchical tree with expand/collapse

### Media & Layout
- **`MaterialCarousel`** — horizontal carousel with drag support
- **`MaterialImageList`** — standard, masonry, woven image lists
- **`MaterialLayoutGrid`** — grid layout with tile bars
- **`MaterialFab`** — floating action button (primary, secondary, tertiary, surface, branded)

### Icons
- **`MaterialIcon`** — Material Symbols font rendering
- **`MaterialSymbol`** — outlined, rounded, sharp variants

---

## Optional Features

| Feature | Description |
|---|---|
| `ondemand` | Online image support for `MaterialImageList` |
| `spreadsheet` | Full spreadsheet widget with SQLite backend |
| `svg_solar` | ~1,200 Solar UI/UX icons |
| `svg_noto` | ~3,600 Noto emoji |
| `svg_twemoji` | ~3,700 Twitter emoji |
| `svg_emoji` | Enables all three SVG icon sets |

```toml
egui-material3 = { path = "../egui-material3", default-features = false }
egui-material3 = { path = "../egui-material3", default-features = false, features = ["ondemand"] }
```

---

## Material Design 3 References

- [Material Design 3 — m3.material.io](https://m3.material.io/)
- [MD3 Component specs](https://m3.material.io/components)
- [MD3 Color system](https://m3.material.io/styles/color/system/overview)
- [MD3 Typography](https://m3.material.io/styles/typography/overview)

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-Apache-2.0](LICENSE-Apache-2.0))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

Upstream crate by [Woojae Park (@nikescar)](https://github.com/nikescar/egui-material3).
