//! Material Design 3 Navigation Rail
//!
//! Navigation rails provide ergonomic movement between primary destinations in
//! apps with medium or large-width layouts. In portrait-phone contexts they are
//! also used as a compact vertical alternative to top tabs.
//!
//! # M3 Spec (2024)
//! - Rail width: 80dp
//! - Item slot height: 72dp (icon 24dp + 4dp gap + 12sp label)
//! - Active indicator pill: 56dp × 32dp
//! - Items stacked from top with 12dp top padding (not evenly distributed)
//!
//! # M3 Color Roles
//! - **surfaceContainer**: Rail background
//! - **secondaryContainer**: Active indicator pill fill
//! - **onSecondaryContainer**: Active icon color
//! - **primary**: Active label color
//! - **onSurfaceVariant**: Inactive icon and label

use crate::get_global_color;
use egui::{Color32, FontId, Pos2, Rect, Response, Sense, Ui, Vec2};

/// A single destination in a `MaterialNavigationRail`.
pub struct NavRailItem {
    /// Unicode icon character (from Material Symbols / NF font).
    pub icon: &'static str,
    /// Short label shown below the icon (should be ≤ 12 chars).
    pub label: &'static str,
}

impl NavRailItem {
    pub const fn new(icon: &'static str, label: &'static str) -> Self {
        Self { icon, label }
    }
}

/// Material Design 3 Navigation Rail.
///
/// Renders a vertical strip of icon+label destinations. Items are stacked from
/// the top (with `TOP_PADDING` before the first slot), matching the M3 spec for
/// phone portrait layouts.
///
/// # Example
/// ```ignore
/// let mut selected = 0usize;
/// let items = &[
///     NavRailItem::new("\u{e8b8}", "General"),
///     NavRailItem::new("\u{eb97}", "Graphics"),
/// ];
/// let (_, changed) = MaterialNavigationRail::new(&mut selected, items).show(ui);
/// ```
pub struct MaterialNavigationRail<'a> {
    selected: &'a mut usize,
    items: &'a [NavRailItem],
    /// Override the rail width (defaults to `Self::WIDTH`).
    width: Option<f32>,
}

impl<'a> MaterialNavigationRail<'a> {
    /// M3 spec navigation rail width: 80dp.
    pub const WIDTH: f32 = 80.0;

    /// Height of each destination slot: icon (24dp) + gap (4dp) + label (12sp ≈ 16dp) + touch padding = 72dp.
    const SLOT_H: f32 = 72.0;

    /// Padding above the first item.
    const TOP_PADDING: f32 = 12.0;

    /// Active indicator pill dimensions (M3 spec: 56dp × 32dp).
    const INDICATOR_W: f32 = 56.0;
    const INDICATOR_H: f32 = 32.0;

    /// Gap between bottom of indicator pill and top of label text.
    const ICON_LABEL_GAP: f32 = 4.0;

    /// Icon size in dp.
    const ICON_SIZE: f32 = 24.0;

    /// Label font size in sp.
    const LABEL_SIZE: f32 = 12.0;

    pub fn new(selected: &'a mut usize, items: &'a [NavRailItem]) -> Self {
        Self { selected, items, width: None }
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }

    /// Render the rail. Returns `(response, changed)`.
    /// The response rect covers the full rail area so the caller can use it
    /// for layout purposes.
    pub fn show(self, ui: &mut Ui) -> (Response, bool) {
        let rail_w = self.width.unwrap_or(Self::WIDTH);

        // Height: top padding + (n slots) — but never more than available height.
        let items_h = self.items.len() as f32 * Self::SLOT_H;
        let needed_h = Self::TOP_PADDING + items_h;
        let avail_h = ui.available_height();
        let rail_h = needed_h.max(avail_h); // always fill available height for the background
        let _ = rail_h;

        let desired = Vec2::new(rail_w, avail_h);
        let (rail_rect, rail_response) = ui.allocate_exact_size(desired, Sense::hover());

        let primary        = get_global_color("primary");
        let on_surface_v   = get_global_color("onSurfaceVariant");
        let secondary_c    = get_global_color("secondaryContainer");
        let on_secondary_c = get_global_color("onSecondaryContainer");
        let surface_c      = get_global_color("surfaceContainer");

        // Rail background — full rect.
        ui.painter().rect_filled(rail_rect, 0.0, surface_c);

        // A thin right-side divider to visually separate rail from content.
        let divider_color = Color32::from_rgba_unmultiplied(
            on_surface_v.r(), on_surface_v.g(), on_surface_v.b(), 30,
        );
        ui.painter().line_segment(
            [rail_rect.right_top(), rail_rect.right_bottom()],
            egui::Stroke::new(1.0, divider_color),
        );

        let icon_font  = FontId::proportional(Self::ICON_SIZE);
        let label_font = FontId::proportional(Self::LABEL_SIZE);

        let mut changed = false;

        for (i, item) in self.items.iter().enumerate() {
            // Slot top = top of rail + top padding + (i × slot height).
            // Clamp so items never render outside the visible rail rect.
            let slot_top = rail_rect.min.y + Self::TOP_PADDING + i as f32 * Self::SLOT_H;
            if slot_top + Self::SLOT_H > rail_rect.max.y + 1.0 {
                break; // not enough space — skip remaining items
            }

            let slot_rect = Rect::from_min_size(
                Pos2::new(rail_rect.min.x, slot_top),
                Vec2::new(rail_w, Self::SLOT_H),
            );

            let item_id  = ui.id().with(("nav_rail_item", i));
            let item_res = ui.interact(slot_rect, item_id, Sense::click());
            let is_selected = *self.selected == i;
            let is_hovered  = item_res.hovered();

            // ── Active indicator pill ──────────────────────────────────────
            // Centered horizontally; vertically placed in the upper ~56% of the slot
            // (icon occupies the center of the pill, label sits below).
            let pill_center_y = slot_top + Self::SLOT_H * 0.42;
            let indicator_rect = Rect::from_center_size(
                Pos2::new(slot_rect.center().x, pill_center_y),
                Vec2::new(Self::INDICATOR_W, Self::INDICATOR_H),
            );

            if is_selected {
                ui.painter().rect_filled(
                    indicator_rect,
                    Self::INDICATOR_H / 2.0,
                    secondary_c,
                );
            } else if is_hovered {
                let hover_col = Color32::from_rgba_unmultiplied(
                    on_surface_v.r(), on_surface_v.g(), on_surface_v.b(), 20,
                );
                ui.painter().rect_filled(
                    indicator_rect,
                    Self::INDICATOR_H / 2.0,
                    hover_col,
                );
            }

            // ── Icon ──────────────────────────────────────────────────────
            let icon_color = if is_selected {
                on_secondary_c
            } else if is_hovered {
                primary
            } else {
                on_surface_v
            };
            ui.painter().text(
                indicator_rect.center(),
                egui::Align2::CENTER_CENTER,
                item.icon,
                icon_font.clone(),
                icon_color,
            );

            // ── Label ─────────────────────────────────────────────────────
            let label_color = if is_selected { primary } else { on_surface_v };
            let label_y = indicator_rect.max.y + Self::ICON_LABEL_GAP;
            ui.painter().text(
                Pos2::new(slot_rect.center().x, label_y),
                egui::Align2::CENTER_TOP,
                item.label,
                label_font.clone(),
                label_color,
            );

            // ── Interaction ───────────────────────────────────────────────
            if item_res.clicked() && !is_selected {
                *self.selected = i;
                changed = true;
            }
        }

        (rail_response, changed)
    }
}
