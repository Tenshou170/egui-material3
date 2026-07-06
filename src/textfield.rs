use crate::theme::get_global_color;
use egui::{
    Color32, CornerRadius, Frame, Margin, Response, Stroke, TextEdit, Ui, Widget, emath::Numeric,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextFieldVariant {
    Filled,
    Outlined,
}

pub struct MaterialTextField<'a> {
    text: &'a mut String,
    hint_text: Option<String>,
    variant: TextFieldVariant,
    multiline: bool,
    enabled: bool,
    width: Option<f32>,
    corner_radius: Option<CornerRadius>,
    id: Option<egui::Id>,
    lock_focus: bool,
}

impl<'a> MaterialTextField<'a> {
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
            hint_text: None,
            variant: TextFieldVariant::Filled,
            multiline: false,
            enabled: true,
            width: None,
            corner_radius: None,
            id: None,
            lock_focus: false,
        }
    }

    pub fn filled(text: &'a mut String) -> Self {
        Self::new(text).variant(TextFieldVariant::Filled)
    }

    pub fn outlined(text: &'a mut String) -> Self {
        Self::new(text).variant(TextFieldVariant::Outlined)
    }

    pub fn hint_text(mut self, hint_text: impl Into<String>) -> Self {
        self.hint_text = Some(hint_text.into());
        self
    }

    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
    }

    pub fn variant(mut self, variant: TextFieldVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn corner_radius(mut self, cr: CornerRadius) -> Self {
        self.corner_radius = Some(cr);
        self
    }

    pub fn id(mut self, id: egui::Id) -> Self {
        self.id = Some(id);
        self
    }

    pub fn desired_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn lock_focus(mut self, lock_focus: bool) -> Self {
        self.lock_focus = lock_focus;
        self
    }
}

impl<'a> Widget for MaterialTextField<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let primary = get_global_color("primary");
        let surface_container_highest = get_global_color("surfaceContainerHighest");
        let on_surface = get_global_color("onSurface");
        let on_surface_variant = get_global_color("onSurfaceVariant");
        let outline = get_global_color("outline");

        let cr_val = self.corner_radius.unwrap_or_else(|| {
            CornerRadius::same(4)
        });
        
        let actual_cr = if self.variant == TextFieldVariant::Filled {
            CornerRadius { nw: cr_val.nw, ne: cr_val.ne, sw: 0, se: 0 }
        } else {
            cr_val
        };

        let id = self.id.unwrap_or_else(|| ui.next_auto_id());
        let is_focused = ui.memory(|m| m.has_focus(id));

        let mut bg_fill = Color32::TRANSPARENT;
        let mut stroke = Stroke::NONE;
        
        if self.enabled {
            match self.variant {
                TextFieldVariant::Filled => {
                    bg_fill = surface_container_highest;
                }
                TextFieldVariant::Outlined => {
                    stroke = Stroke::new(1.0, if is_focused { primary } else { outline });
                }
            }
        } else {
            match self.variant {
                TextFieldVariant::Filled => {
                    bg_fill = on_surface.linear_multiply(0.04);
                }
                TextFieldVariant::Outlined => {
                    stroke = Stroke::new(1.0, on_surface.linear_multiply(0.12));
                }
            }
        }

        let frame = Frame::NONE
            .fill(bg_fill)
            // MD3 filled text field: 56dp spec is for floating-label variant.
            // Without a label, 36dp (6+24+6) is the correct compact height.
            .inner_margin(Margin { left: 16, right: 16, top: 6, bottom: 6 })
            .corner_radius(actual_cr);

        let frame = if self.variant == TextFieldVariant::Outlined {
            frame.stroke(stroke)
        } else {
            frame
        };

        // Cap to available_width so the frame never pushes the grid column wider.
        if self.width.is_none() {
            ui.set_max_width(ui.available_width());
        }

        let frame_res = frame.show(ui, |ui| {
            let mut text_edit = if self.multiline {
                TextEdit::multiline(self.text)
            } else {
                TextEdit::singleline(self.text)
            };

            if let Some(w) = self.width {
                // Caller supplied an explicit total width including margins.
                // Subtract left(16) + right(16) to get inner content width.
                let inner_w = (w - 32.0).max(0.0);
                ui.set_max_width(inner_w);
                text_edit = text_edit.desired_width(inner_w);
            } else {
                // We are inside the frame's content area — available_width()
                // already reflects the space after inner_margin is applied.
                // Fill it fully with no additional subtraction.
                text_edit = text_edit.desired_width(ui.available_width());
            }

            // M3 spec: single-line text field height is 56dp; with top:10+bottom:10
            // inner margin that leaves 36dp for the text itself. min_size ensures
            // the TextEdit never collapses below a legible line height.
            text_edit = text_edit.min_size(egui::vec2(0.0, 24.0));

            text_edit = text_edit.id(id).frame(false);
            
            if let Some(hint) = &self.hint_text {
                text_edit = text_edit.hint_text(hint);
            }
            if !self.enabled {
                text_edit = text_edit.interactive(false);
            }
            if self.lock_focus {
                text_edit = text_edit.lock_focus(true);
            }
            
            let text_color = if self.enabled { on_surface } else { on_surface.linear_multiply(0.38) };
            text_edit = text_edit.text_color(text_color);

            ui.add(text_edit)
        });

        if self.variant == TextFieldVariant::Filled {
            let rect = frame_res.response.rect;
            let bottom_left = rect.left_bottom();
            let bottom_right = rect.right_bottom();
            let line_color = if !self.enabled {
                on_surface.linear_multiply(0.38)
            } else if is_focused {
                primary
            } else {
                on_surface_variant
            };
            let line_width = 1.0;
            
            // Adjust the coordinates to avoid overflowing the frame
            let adjusted_left = egui::pos2(bottom_left.x, bottom_left.y - line_width / 2.0);
            let adjusted_right = egui::pos2(bottom_right.x, bottom_right.y - line_width / 2.0);
            
            ui.painter().line_segment([adjusted_left, adjusted_right], Stroke::new(line_width, line_color));
        }

        let mut resp = frame_res.inner;
        resp.rect = frame_res.response.rect;



        if resp.clicked() {
            resp.request_focus();
        }

        resp
    }
}

pub struct MaterialNumberField<'a, Num: Numeric> {
    value: &'a mut Num,
    variant: TextFieldVariant,
    enabled: bool,
    width: Option<f32>,
    corner_radius: Option<CornerRadius>,
    range: Option<std::ops::RangeInclusive<Num>>,
    decimals: Option<usize>,
    speed: Option<f64>,
    suffix: Option<String>,
}

impl<'a, Num: Numeric> MaterialNumberField<'a, Num> {
    pub fn new(value: &'a mut Num) -> Self {
        Self {
            value,
            variant: TextFieldVariant::Filled,
            enabled: true,
            width: None,
            corner_radius: None,
            range: None,
            decimals: None,
            speed: None,
            suffix: None,
        }
    }

    pub fn filled(value: &'a mut Num) -> Self {
        Self::new(value).variant(TextFieldVariant::Filled)
    }

    pub fn outlined(value: &'a mut Num) -> Self {
        Self::new(value).variant(TextFieldVariant::Outlined)
    }

    pub fn variant(mut self, variant: TextFieldVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn range(mut self, range: std::ops::RangeInclusive<Num>) -> Self {
        self.range = Some(range);
        self
    }

    pub fn decimals(mut self, decimals: usize) -> Self {
        self.decimals = Some(decimals);
        self
    }

    pub fn speed(mut self, speed: f64) -> Self {
        self.speed = Some(speed);
        self
    }

    pub fn suffix(mut self, suffix: impl ToString) -> Self {
        self.suffix = Some(suffix.to_string());
        self
    }
}

impl<'a, Num: Numeric> Widget for MaterialNumberField<'a, Num> {
    fn ui(self, ui: &mut Ui) -> Response {
        let primary = get_global_color("primary");
        let surface_container_highest = get_global_color("surfaceContainerHighest");
        let on_surface = get_global_color("onSurface");
        let _on_surface_variant = get_global_color("onSurfaceVariant");
        let outline = get_global_color("outline");

        let cr_val = self.corner_radius.unwrap_or_else(|| {
            CornerRadius::same(4)
        });
        
        let id = ui.next_auto_id();
        let is_focused = ui.memory(|m| m.has_focus(id));

        let actual_cr = if is_focused {
            CornerRadius::ZERO
        } else if self.variant == TextFieldVariant::Filled {
            CornerRadius { nw: cr_val.nw, ne: cr_val.ne, sw: 0, se: 0 }
        } else {
            cr_val
        };

        let mut bg_fill = Color32::TRANSPARENT;
        let mut stroke = Stroke::NONE;
        
        if self.enabled {
            match self.variant {
                TextFieldVariant::Filled => {
                    bg_fill = surface_container_highest;
                }
                TextFieldVariant::Outlined => {
                    stroke = Stroke::new(1.0, if is_focused { primary } else { outline });
                }
            }
        } else {
            match self.variant {
                TextFieldVariant::Filled => {
                    bg_fill = on_surface.linear_multiply(0.04);
                }
                TextFieldVariant::Outlined => {
                    stroke = Stroke::new(1.0, on_surface.linear_multiply(0.12));
                }
            }
        }

        let frame = Frame::NONE
            .fill(bg_fill)
            // Compact margins for inline number fields — tighter than full text fields.
            .inner_margin(Margin { left: 8, right: 8, top: 4, bottom: 4 })
            .corner_radius(actual_cr);

        let frame = if self.variant == TextFieldVariant::Outlined {
            frame.stroke(stroke)
        } else {
            frame
        };

        // Cap to available_width so the frame never pushes the grid column wider.
        if self.width.is_none() {
            ui.set_max_width(ui.available_width());
        }

        let frame_res = frame.show(ui, |ui| {
            if let Some(w) = self.width {
                ui.set_min_width(w);
            }

            // text parsing logic
            let mut text_state: String = ui.data_mut(|d| {
                d.get_temp(id).unwrap_or_else(|| {
                    if let Some(decimals) = self.decimals {
                        format!("{:.*}", decimals, self.value.to_f64())
                    } else {
                        // Default to 1 decimal so whole-number floats show "1.0" not "1"
                        format!("{:.1}", self.value.to_f64())
                    }
                })
            });

            ui.push_id(id, |ui| {
                if !self.enabled {
                    ui.disable();
                }
                
                ui.visuals_mut().widgets.inactive.bg_stroke = Stroke::NONE;
                ui.visuals_mut().widgets.inactive.bg_fill = Color32::TRANSPARENT;
                ui.visuals_mut().widgets.hovered.bg_stroke = Stroke::NONE;
                ui.visuals_mut().widgets.hovered.bg_fill = Color32::TRANSPARENT;
                ui.visuals_mut().widgets.active.bg_stroke = Stroke::NONE;
                ui.visuals_mut().widgets.active.bg_fill = Color32::TRANSPARENT;
                ui.visuals_mut().widgets.noninteractive.bg_stroke = Stroke::NONE;
                ui.visuals_mut().widgets.noninteractive.bg_fill = Color32::TRANSPARENT;

                let te_res = ui.add(TextEdit::singleline(&mut text_state)
                    .id(id)
                    .frame(false)
                    .margin(egui::Vec2::ZERO)
                    .min_size(egui::vec2(0.0, 20.0))
                    .desired_width(f32::INFINITY)
                    .horizontal_align(egui::Align::Center)
                    .vertical_align(egui::Align::Center)
                    .interactive(self.enabled));
                
                if te_res.changed() || te_res.lost_focus() {
                    // Try parsing
                    if let Ok(val) = text_state.parse::<f64>() {
                        let mut final_val = val;
                        // apply range
                        if let Some(range) = &self.range {
                            final_val = final_val.clamp(range.start().to_f64(), range.end().to_f64());
                        }
                        *self.value = Num::from_f64(final_val);
                    }
                }

                if te_res.has_focus() {
                    ui.data_mut(|d| d.insert_temp(id, text_state.clone()));
                } else {
                    ui.data_mut(|d| d.remove_temp::<String>(id));
                }

                if let Some(s) = &self.suffix {
                    ui.add_space(4.0);
                    ui.label(s);
                }

                te_res
            }).inner
        });

        let mut resp = frame_res.inner;
        resp.rect = frame_res.response.rect;



        if resp.clicked() {
            resp.request_focus();
        }

        resp
    }
}
