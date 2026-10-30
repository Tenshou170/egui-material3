# egui-material3

Material Design 3 for [egui](https://github.com/emilk/egui) — a component library plus a
theming engine that models the *whole* MD3 system, not just its colours.

`egui 0.33` · `edition 2021` · `MIT OR Apache-2.0`

![Widget gallery](resources/screenshot.png)

*Widget gallery plus the per-component story windows, from the crate's own examples.*

---

## What you get

- **~90 widget types across 44 modules** — buttons, icon buttons, FABs, switches, sliders,
  selects, text fields, chips, tabs, list tiles, cards, dialogs, menus, snackbars, drawers,
  navigation rail, top/bottom app bars, breadcrumbs, data tables, timelines, tree views,
  carousels, image lists, layout grids, badges, progress indicators, tooltips, dashboards.
- **A real token system.** Colour roles, a 15-step type scale, a per-component shape scale,
  six elevation levels, the full state-layer set and density-aware spacing. Widgets render
  from those tokens — colours, shape, state layers, type sizes — and your own widgets can read
  the same ones.
- **HCT colour schemes.** Load a [Material Theme Builder](https://material-foundation.github.io/material-theme-builder/)
  JSON export as-is, or generate one from a single seed colour. Light/dark, three contrast
  levels, and per-role manual overrides are supported.
- **Theme files that also style type and shape.** A theme JSON may carry per-role font sizes
  and per-component corner radii, so a brand can shift the type scale or rounding without
  touching code.
- **Material Symbols + emoji.** Icon widgets render glyphs from a Material Symbols font you
  register once; optional SVG sets add ~1,200 Solar icons and ~7,270 Noto/Twemoji emoji.
- **Desktop and touch.** `Density::Touch` switches interactive targets, button padding and
  list metrics to 48 dp for Android; `Density::Desktop` keeps the 40 dp desktop rhythm.
- **Runs on top of plain egui.** No custom context, no retained-mode tree: widgets are egui
  widgets, layouts are egui layouts, and anything not covered here still works.

---

## Quick start

```toml
# Cargo.toml
[dependencies]
egui-material3 = { path = "../egui-material3" }
eframe = "0.33"
```

```rust
// src/main.rs
use eframe::egui;
use egui_material3::{
    theme::{
        apply_theme_auto, load_fonts, load_themes, setup_local_fonts,
        setup_local_fonts_from_bytes, setup_local_theme, update_window_background,
    },
    MaterialButton, MaterialCheckbox, MaterialSwitch, MaterialSlider, MaterialTabs,
};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([960.0, 640.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Material App",
        options,
        Box::new(|cc| {
            // 1. Optional text font. Omit to keep egui's default proportional font.
            setup_local_fonts(Some("assets/Inter.ttf"));

            // 2. Icon font. The name must contain "MaterialSymbols" so it is registered as a
            //    *fallback* after the text font instead of replacing it.
            setup_local_fonts_from_bytes(
                "MaterialSymbolsOutlined",
                include_bytes!("../assets/MaterialSymbolsOutlined.ttf"),
            );

            // 3. Theme: a JSON file, or `None` for the built-in default scheme.
            setup_local_theme(Some("assets/material-theme.json"));

            // 4. Install everything into the egui context.
            load_fonts(&cc.egui_ctx);
            load_themes();
            apply_theme_auto(&cc.egui_ctx); // or apply_theme(ctx, Some(|| ThemeMode::Dark))
            update_window_background(&cc.egui_ctx);

            Ok(Box::<App>::default())
        }),
    )
}

#[derive(Default)]
struct App {
    tab: usize,
    enabled: bool,
    volume: f32,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(
                MaterialTabs::primary(&mut self.tab)
                    .tab("General")
                    .tab("Playback")
                    .width(ui.available_width()),
            );

            ui.add_space(16.0);
            match self.tab {
                0 => {
                    ui.add(MaterialSwitch::new(&mut self.enabled).text("Enabled"));
                    ui.add(MaterialCheckbox::new(&mut self.enabled, "Same flag, checkbox"));
                }
                _ => {
                    ui.add(MaterialSlider::new(&mut self.volume, 0.0..=100.0));
                }
            }

            ui.add_space(16.0);
            if ui.add(MaterialButton::filled("Save")).clicked() {
                // …
            }
        });
    }
}
```

Icons are glyphs of a Material Symbols font, so **register that font to use `MaterialIcon`,
`MaterialIconButton`, `MaterialFab`, nav-rail items, list-tile leading icons, etc.** The
repository ships one at
`resources/MaterialSymbolsOutlined[FILL,GRAD,opsz,wght].ttf`; the crate itself does not embed
it, because the licence text belongs to the consumer's distribution.

---

## Theming engine

### Colour: HCT schemes, not hand-picked palettes

Schemes are generated in the HCT colour space, so tone mapping, contrast and colour-vision
behaviour follow the MD3 spec rather than linear RGB interpolation. A theme JSON from Material
Theme Builder is accepted with all six scheme variants:

- `light`, `light-medium-contrast`, `light-high-contrast`
- `dark`, `dark-medium-contrast`, `dark-high-contrast`

```rust
use egui_material3::theme::{
    apply_theme_auto, set_contrast_level, set_theme_mode, ContrastLevel, ThemeMode,
};

set_theme_mode(ThemeMode::Dark);
set_contrast_level(ContrastLevel::High);
apply_theme_auto(&ctx); // re-apply after changing either setting
```

`ThemeMode::Auto` resolves through `detect_os_theme()` unless you supply your own detector:

```rust
use egui_material3::theme::{apply_theme, detect_os_theme, ThemeMode};

// Auto + the crate's own OS probe
apply_theme(&ctx, Some(|| detect_os_theme()));

// Auto + your own probe (e.g. an Android night-mode flag)
apply_theme(&ctx, Some(|| if is_night() { ThemeMode::Dark } else { ThemeMode::Light }));
```

Themes can also be swapped at runtime from an in-memory string — handy for a theme editor:

```rust
use egui_material3::theme::{apply_theme_auto, load_theme_from_json_str};

load_theme_from_json_str(&json)?;
apply_theme_auto(&ctx);
```

### Reading colour roles

```rust
use egui_material3::theme::{get_global_color, COLOR_ROLE_NAMES};

let primary        = get_global_color("primary");
let on_surface     = get_global_color("onSurface");
let container      = get_global_color("surfaceContainerHigh");
let outline        = get_global_color("outlineVariant");

// Every role the engine knows about:
for role in COLOR_ROLE_NAMES {
    println!("{role} = {:?}", get_global_color(role));
}
```

Names are the camelCase MD3 role names (`onPrimaryContainer`, `surfaceContainerLowest`,
`inverseSurface`, `scrim`, …). Roles absent from the active scheme fall back to a sensible
neighbour rather than panicking, so custom or partial theme files stay usable.

### Type scale

15 roles from `display-large` to `label-small`, installed into `Style::text_styles` by
`apply_theme` under `m3/<role>` names.

| Role | Size | Line height | Emphasis |
|---|---|---|---|
| `display-large` / `-medium` / `-small` | 57 / 45 / 36 | 64 / 52 / 44 | regular |
| `headline-large` / `-medium` / `-small` | 32 / 28 / 24 | 40 / 36 / 32 | regular |
| `title-large` / `-medium` / `-small` | 22 / 16 / 14 | 28 / 24 / 20 | medium |
| `body-large` / `-medium` / `-small` | 16 / 14 / 12 | 24 / 20 / 16 | regular |
| `label-large` / `-medium` / `-small` | 14 / 12 / 11 | 20 / 16 / 16 | medium |

```rust
use egui_material3::theme::typography::{self, TypeRole};
use egui::RichText;

// Preferred: goes through Style::text_styles, so Style::scale is honoured.
ui.label(RichText::new("Headline").text_style(TypeRole::HeadlineSmall.text_style()));

// Painter-level text: a resolved FontId (not affected by Style::scale).
ui.painter().text(
    pos,
    egui::Align2::LEFT_TOP,
    "Overlay",
    typography::font_id(TypeRole::LabelMedium),
    egui::Color32::WHITE,
);

let line = TypeRole::BodyLarge.line_height();      // 24.0
let bold = TypeRole::TitleMedium.is_emphasized();  // true — render with RichText::strong()
```

### Shape scale

Ten component slots, each with an MD3 default radius. Window-scale surfaces (dialog, sheet,
menu, drawer, snackbar) follow the global corner-radius override, while in-content components
(button, card, chip, FAB, text field) keep their own shape role.

```rust
use egui_material3::theme::{component_radius, set_global_corner_radius, set_global_shape_scale, ShapeComponent};

let card_r = component_radius(ShapeComponent::Card);   // 12.0 by default
let fab_r  = component_radius(ShapeComponent::Fab);    // 16.0

set_global_shape_scale(1.25);                  // scale every slot proportionally
set_global_corner_radius(Some(20.0));          // uniform override for floating surfaces
set_global_corner_radius(None);                // back to per-component MD3 values
```

| Slot | JSON key | Default |
|---|---|---|
| Button | `button` | 20.0 (full for text/icon) |
| FAB | `fab` | 16.0 |
| Card | `card` | 12.0 |
| Dialog | `dialog` | 28.0 |
| Menu | `menu` | 4.0 |
| Sheet | `sheet` | 28.0 |
| Text field | `text-field` | 4.0 |
| Chip | `chip` | 8.0 |
| Drawer | `drawer` | 16.0 |
| Snackbar | `snackbar` | 4.0 |

### Elevation, state layers, spacing, density

```rust
use egui_material3::theme::{elevation, shadow, state_layer, blend_state_layer, spacing, Density};

let s = shadow(elevation::LEVEL_2);            // egui::Shadow for a raised card
let pressed = state_layer::PRESSED;            // 0.10
let hover = state_layer::HOVER;                // 0.08
let disabled = state_layer::DISABLED_CONTENT;  // 0.38

// Porter-Duff source-over blend of an overlay on a base colour.
let hot = blend_state_layer(surface, on_surface, hover);

// Density-aware metrics.
let target = spacing::interact_size(Density::Touch);        // 48 × 48 dp
let padding = spacing::button_padding(Density::Touch);      // 24 × 12 dp
let row = spacing::list_row_height(2);                      // 72.0 (one line: 56.0)
let dense_row = spacing::list_row_height_dense(2);           // 64.0
```

`set_global_density(Density::Touch)` also rewrites egui's `Spacing::interact_size`, button
padding and list metrics inside `apply_theme`, so stock egui widgets line up with the
Material ones.

### Theme file format

A Material Theme Builder export, optionally extended with `typography` and `shapes`:

```json
{
  "description": "My brand",
  "seed": "#769CDF",
  "coreColors": { "primary": "#769CDF" },
  "extendedColors": [],
  "schemes": {
    "light": { "primary": "#415F91", "onPrimary": "#FFFFFF", "surface": "#F9F9FF" },
    "dark":  { "primary": "#AAC7FF", "onPrimary": "#0A305F", "surface": "#111318" }
  },
  "palettes": {},
  "typography": { "m3/label-large": 13.0, "m3/body-medium": 13.5 },
  "shapes": { "card": 16.0, "dialog": 24.0, "button": 12.0 }
}
```

- `typography` keys are role keys in sp; roles you omit keep the M3 spec size.
- `shapes` keys are slot keys in dp; slots you omit keep the shape scale.
- Both maps are optional, so plain Material Theme Builder exports load unchanged.

### Writing a theme-aware widget

```rust
use egui_material3::theme::{
    blend_state_layer, component_radius, get_global_color, state_layer, typography::TypeRole,
    ShapeComponent,
};

fn my_badge(ui: &mut egui::Ui, text: &str) {
    let role = TypeRole::LabelSmall;
    let font = ui
        .style()
        .text_styles
        .get(&role.text_style())
        .cloned()
        .unwrap_or_else(|| role.font_id());

    let galley = ui.painter().layout_no_wrap(text.to_owned(), font, egui::Color32::WHITE);
    let size = galley.size() + egui::vec2(16.0, 8.0);
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());

    let container = get_global_color("secondaryContainer");
    ui.painter()
        .rect_filled(rect, component_radius(ShapeComponent::Chip), container);

    let on_container = get_global_color("onSecondaryContainer");
    let fg = blend_state_layer(on_container, on_container, state_layer::HOVER);
    ui.painter().galley(rect.min + egui::vec2(8.0, 4.0), galley, fg);
}
```

---

## Components

### Actions

- **`MaterialButton`** — `filled`, `filled_tonal`, `elevated`, `outlined`, `text`; `.selected()`,
  `.truncate()`, `.image()` / `.image_and_text()` for icons.
- **`MaterialIconButton`** — `standard`, `filled`, `filled_tonal`, `outlined`, `toggle`
  (bound to `&mut bool`); `.size()`, `.enabled()`.
- **`MaterialFab`** — `primary`, `secondary`, `tertiary`, `surface`, `branded`; sizes
  small/regular/large; icon or extended label.
- **`MaterialActionSheet`** — bottom sheet with grouped actions and cancel.

### Selection & input

- **`MaterialCheckbox`** — checked, indeterminate, error and disabled states.
- **`MaterialRadio` / `MaterialRadioGroup` / `RadioListTile`** — standalone, grouped, and as a
  list tile with configurable control affinity.
- **`MaterialSwitch`** — track/thumb/outline resolved per state; optional on/off icons;
  selection handle ripple.
- **`MaterialSlider` / `MaterialRangeSlider`** — primary track, secondary track value, step
  snapping, size variants, custom thumb shapes.
- **`MaterialSelect`** — filled/outlined dropdown with floating label, placeholder, per-option
  text, text clipping, `.small()` compact mode.
- **`MaterialTextField`** — filled/outlined, floating label, hint, multiline, helper/error text,
  leading/trailing widgets, MD3 56 dp field height.
- **`MaterialNumberField`** — numeric drag-and-type field with range, decimals and suffix.
- **`MaterialChip`** — assist, filter, input, suggestion; elevated and soft-disabled variants.

### Navigation

- **`MaterialTabs`** — primary/secondary indicator; fixed equal-width or `.scrollable()`;
  `.width()` for edge-to-edge rendering, `.compact()` for tighter label metrics, icons per tab.
- **`MaterialNavigationRail`** — vertical destinations with pill indicators and labels, compact
  and standard widths.
- **`MaterialDrawer`** — permanent, standard, dismissible and modal variants with sections,
  headers and 3-line items.
- **`MaterialTopAppBar`** — regular, center-aligned, medium, large; scroll-reactive elevation.
- **`MaterialBottomAppBar`** — actions plus an optional docked FAB.
- **`MaterialToolbar`** — flexible toolbar with grouped action items.
- **`MaterialBreadcrumbs`** — collapsing breadcrumb trail with separators and click handling.
- **`MaterialMenu` / `MenuItem`** — anchored context menus with nested submenus and per-item
  icons, shortcuts and enabled state.

### Containment & feedback

- **`MaterialCard2`** — elevated, filled, outlined; header/cover/content/actions slots.
- **`MaterialDialog`** — modal dialog with icon, title, content and text/filled-tonal/filled
  actions (MD3 order: dismiss left, confirm right), focus trap and escape handling.
- **`MaterialSnackbar`** — auto-dismissing toasts with action button and position control.
- **`MaterialNotification`** — inline notification/banner cards with icon, title and close.
- **`MaterialBadge`** — count and dot badges, drawn on a target rect or as a standalone widget.
- **`MaterialProgress`** — linear/circular, determinate/indeterminate, buffer and four-colour.
- **`MaterialTooltip`** — rich tooltips with position and max-width control.

### Data display

- **`MaterialList` / `ListItem`** — one/two/three-line tiles, leading and trailing slots,
  visual density control.
- **`MaterialDataTable`** — sortable, selectable columns, custom cell widgets, lazy
  `DataTableSource` for large data sets.
- **`MaterialTimeline`** — chronological items with positioned dots and opposite content.
- **`MaterialTreeView`** — expandable nodes with icons and selection state.
- **`MaterialDashCounter`** — metric cards with primary total and sub-counters.
- **`MaterialSpreadsheet`** *(feature `spreadsheet`)* — full grid widget backed by SQLite.

### Media & layout

- **`MaterialCarousel`** — horizontal cards with drag/snap, custom extents and padding.
- **`MaterialImageList`** — standard, masonry and woven layouts, local or remote sources.
- **`MaterialLayoutGrid`** — column grid with gutters, spans, tile bars and a debug overlay.
- **`MaterialIcon` / `MaterialSymbol`** — Material Symbols glyphs in outlined/rounded/sharp.

---

## Examples

```bash
cargo run --example widget_gallery_example   # all widgets, one window
cargo run --example nobel_prizes_example     # themed app with runtime theme loading
cargo run --example stories                  # per-component story windows
cargo run --example svg_icon_demo --features svg_emoji
```

### Buttons and icon buttons

```rust
use egui_material3::{icon_button_filled, icon_button_standard, MaterialButton};

if ui.add(MaterialButton::filled("Save")).clicked() { /* … */ }
if ui.add(MaterialButton::filled_tonal("Preview")).clicked() { /* … */ }
if ui.add(MaterialButton::outlined("Cancel")).clicked() { /* … */ }
if ui.add(MaterialButton::text("Learn more")).clicked() { /* … */ }

ui.add(icon_button_standard("\u{e5d2}"));            // refresh
ui.add(icon_button_filled("\u{e5ca}"));              // add

let mut pinned = true;
ui.add(MaterialIconButton::toggle("\u{e838}", &mut pinned));
```

### FAB

```rust
use egui_material3::{FabSize, MaterialFab};

if ui.add(MaterialFab::primary().icon("\u{e5ca}")).clicked() { /* … */ }
ui.add(MaterialFab::tertiary().icon("\u{e8b0}").size(FabSize::Small));
ui.add(MaterialFab::secondary().icon("\u{e3c9}").text("Compose")); // extended FAB
```

### Text fields

```rust
use egui_material3::{MaterialNumberField, MaterialTextField, TextFieldVariant};

ui.add(
    MaterialTextField::outlined(&mut self.name)
        .hint_text("Character name")
        .width(240.0),
);
ui.add(
    MaterialTextField::filled(&mut self.notes)
        .multiline(true)
        .variant(TextFieldVariant::Filled),
);
ui.add(
    MaterialNumberField::new(&mut self.volume)
        .range(0.0..=100.0)
        .decimals(0)
        .suffix("%"),
);
```

### Select

```rust
use egui_material3::{MaterialSelect, SelectVariant};

ui.add(
    MaterialSelect::new(&mut self.quality)   // Option<usize>
        .label("Quality")
        .variant(SelectVariant::Outlined)
        .option(0, "Low")
        .option(1, "Medium")
        .option(2, "High")
        .width(220.0),
);
```

### Switches, checkboxes, radios

```rust
use egui_material3::{MaterialCheckbox, MaterialRadio, MaterialSwitch};

ui.add(MaterialSwitch::new(&mut self.enabled).text("Enable feature"));
ui.add(MaterialSwitch::new(&mut self.show_fps).with_icons('\u{e8b6}', '\u{e8b7}'));

ui.add(MaterialCheckbox::new(&mut self.accepted, "I understand"));
ui.add(MaterialCheckbox::new(&mut self.partial, "Partial").indeterminate(true));

ui.add(MaterialRadio::new(&mut self.mode, 0usize, "Automatic"));
ui.add(MaterialRadio::new(&mut self.mode, 1usize, "Manual"));
```

### Sliders

```rust
use egui_material3::{MaterialRangeSlider, MaterialSlider, RangeValues};

ui.add(MaterialSlider::new(&mut self.volume, 0.0..=100.0).text("Volume"));
ui.add(MaterialSlider::new(&mut self.blur, 0.0..=10.0).step(0.5).width(280.0));

let mut range = RangeValues { start: 10.0, end: 60.0 };
ui.add(MaterialRangeSlider::new(&mut range, 0.0..=100.0));
```

### Tabs

```rust
use egui_material3::{tabs_primary, tabs_secondary};

// Fixed equal-width tabs (MD3 "fixed tabs"): pass the inner width explicitly so the
// bar can render edge to edge.
ui.add(
    tabs_primary(&mut self.tab)
        .tab("General")
        .tab("Graphics")
        .tab("Gameplay")
        .width(ui.available_width())
        .id_salt("settings_tabs"),
);

// Scrollable tabs (MD3 "scrollable tabs")
egui::ScrollArea::horizontal().show(ui, |ui| {
    ui.add(
        tabs_secondary(&mut self.sub_tab)
            .tab("Overview")
            .tab("Details")
            .tab("History")
            .scrollable()
            .id_salt("detail_tabs"),
    );
});
```

### Cards

```rust
use egui_material3::{elevated_card2, filled_card2, outlined_card2};

ui.add(
    elevated_card2()
        .header("Build info", Some("v0.30.0"))
        .content(|ui| {
            ui.label("Compiled with rustc 1.90");
        })
        .actions(|ui| {
            if ui.add(MaterialButton::text("Copy")).clicked() { /* … */ }
        }),
);
```

### Dialogs

```rust
use egui_material3::MaterialDialog;

let mut open = true;
let mut confirmed = false;

MaterialDialog::new(egui::Id::new("delete_confirm"), "Delete profile?", &mut open)
    .content(|ui| {
        ui.label("This removes the profile and its saved state.");
    })
    .text_action("Cancel", || {})
    .filled_action("Delete", || confirmed = true)
    .show(ctx);
```

### Snackbar and notifications

```rust
use egui_material3::{MaterialNotification, MaterialSnackbar, SnackbarPosition};

// Snackbars render where you add them, so anchor them at the screen edge.
egui::Area::new(egui::Id::new("snackbar"))
    .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -16.0))
    .show(ctx, |ui| {
        ui.add(
            MaterialSnackbar::new("Settings saved")
                .position(SnackbarPosition::Bottom)
                .auto_dismiss(Some(std::time::Duration::from_secs(4)))
                .action("Undo", || { /* … */ })
                .show(), // marks it visible and starts the auto-dismiss timer
        );
    });

ui.add(
    MaterialNotification::new()
        .title("Update available")
        .text("Restart to apply version 0.30.1.")
        .icon("\u{e5d8}")
        .closeable(true),
);
```

### Lists

```rust
use egui_material3::{ListItem, MaterialList};

ui.add(
    MaterialList::new()
        .dividers(true)
        .item(
            ListItem::new("General")
                .secondary_text("Language, startup behaviour")
                .leading_icon("\u{e8b8}")
                .trailing_icon("\u{e5cc}"),
        )
        .item(
            ListItem::new("Advanced")
                .secondary_text("Debug overlays, logging")
                .leading_icon("\u{e30f}")
                .trailing_text("12"),
        ),
);
```

### Navigation rail and top app bar

```rust
use egui_material3::{MaterialNavigationRail, MaterialTopAppBar, NavRailItem};

ui.add(MaterialTopAppBar::center_aligned("Settings").scrolled(self.scrolled));

let items = [
    NavRailItem { icon: "\u{e88a}", label: "Home" },
    NavRailItem { icon: "\u{e8b8}", label: "Settings" },
    NavRailItem { icon: "\u{e30f}", label: "Advanced" },
];
let (_response, changed) = MaterialNavigationRail::new(&mut self.dest, &items).show(ui);
if changed { /* destination switched */ }
```

### Drawer

```rust
use egui_material3::{permanent_drawer, modal_drawer};

permanent_drawer(&mut self.drawer_open)
    .width(280.0)
    .item("Home", Some("\u{e88a}"), self.page == 0)
    .item("Settings", Some("\u{e8b8}"), self.page == 1)
    .show(ctx);
```

### Data table

```rust
use egui_material3::{MaterialDataTable, SortDirection};

let mut table = MaterialDataTable::new()
    .column("Name", 180.0, false)
    .column("Score", 90.0, true)
    .sort_by(1, SortDirection::Descending);

for (name, score) in &self.rows {
    table = table.row(|row| row.cell(name.clone()).cell(score.to_string()));
}

let response = table.show(ui);
if let Some(row) = response.selected_rows.iter().position(|&sel| sel) {
    self.selected = Some(row);
}
```

### Progress, badge, tooltip

```rust
use egui_material3::{show_tooltip_on_hover, MaterialBadge, MaterialProgress};

ui.add(MaterialProgress::linear().value(0.42).max(1.0));
ui.add(MaterialProgress::circular().indeterminate(true));

ui.add(MaterialBadge::new("12").color(BadgeColor::Error));
show_tooltip_on_hover(ui, &response, "Refresh", TooltipPosition::Top);
```

### Timeline and tree view

```rust
use egui_material3::{MaterialTimeline, MaterialTreeView, TimelineItem, TimelinePosition, TreeViewItem};

ui.add(
    MaterialTimeline::new()
        .position(TimelinePosition::Right)
        .item(TimelineItem::new().content("Hooked IL2CPP").opposite_content("10:02"))
        .item(TimelineItem::new().content("Loaded translations").opposite_content("10:03")),
);

let items = [TreeViewItem::new("umamusume", "Game hooks")
    .icon("\u{e2c7}")
    .child(TreeViewItem::new("captions", "Caption suppression"))
    .child(TreeViewItem::new("camera", "Free camera"))];
let mut state = TreeViewState::new();
ui.add(MaterialTreeView::new(&items, &mut state));
```

---

## Optional features

All features are off by default; the base crate only depends on `egui`, `serde`, `serde_json`,
`resvg` and `image`.

| Feature | Adds |
|---|---|
| `ondemand` | Runtime font/image download (`ureq`) for `MaterialImageList` and `setup_google_fonts` |
| `spreadsheet` | `MaterialSpreadsheet` with a SQLite backend (`rusqlite` native, `sqlite-wasm` on the web) |
| `svg_solar` | ~1,200 Solar UI/UX icons |
| `svg_noto` | ~3,583 Noto emoji |
| `svg_twemoji` | ~3,689 Twemoji glyphs |
| `svg_emoji` | Enables `svg_solar` + `svg_noto` + `svg_twemoji` |

```toml
egui-material3 = { path = "../egui-material3", default-features = false, features = ["svg_solar"] }
egui-material3 = { path = "../egui-material3", default-features = false, features = ["ondemand", "spreadsheet"] }
```

`resources/` (icon fonts, SVG sets, sample theme JSON and CSVs) is excluded from the published
package to keep it small: when using a path dependency the assets are available, otherwise the
`build.rs` download step fetches the requested SVG set from GitHub once.

---

## Notes and gotchas

- **Fixed tabs need a width.** Without `.width(...)`, a fixed tab bar sizes to its content; pass
  the container width to get the spec'd equal-width, edge-to-edge bar.
- **`Style::scale` and type roles.** `TypeRole::text_style()` participates in UI scaling;
  `typography::font_id()` returns a literal `FontId` and does not. Use the former in layouts,
  the latter at painter level.
- **Icon font naming.** Any font registered with a name containing `MaterialSymbols` is appended
  as a fallback; other fonts are inserted first. This is what lets CJK/latin text render while
  icon glyphs still resolve.
- **`apply_theme(ctx, None)` needs a type annotation.** The detector closure type cannot be
  inferred from `None`, so use `apply_theme_auto(ctx)` (or `apply_theme(ctx, Some(|| …))`).
- **`apply_theme` rewrites `Style::text_styles`.** If you fold a GUI scale into the style, re-apply
  `style.scale(..)` after calling `apply_theme`, or text snaps back to 1×.
- **Window vs. in-content rounding.** `set_global_corner_radius` only affects dialog, sheet, menu,
  drawer and snackbar surfaces — that is the "window rounding" knob, not a global radius.
- **Density is global.** `set_global_density` changes target sizes for the whole context, so set
  it once at startup (Android → `Touch`, desktop → `Desktop`).

---

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-Apache-2.0](LICENSE-Apache-2.0))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

Material Symbols and the Noto/Twemoji sets carry their own licences; see `resources/`.
