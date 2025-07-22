use crate::style::Style;
use raylib::prelude::*;

pub struct Button {
    pub bounds: Rectangle,
    pub label: String,
    pub style: Style,
    pub is_hovered: bool,
    pub is_pressed: bool,
    pub animation_progress: f32,
    pub enabled: bool,
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, label: &str) -> Self {
        Self {
            bounds: Rectangle::new(x, y, width, height),
            label: label.to_string(),
            style: Style::default(),
            is_hovered: false,
            is_pressed: false,
            animation_progress: 0.0,
            enabled: true,
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn set_colors(
        &mut self,
        background: Color,
        hover: Color,
        pressed: Color,
        border: Color,
        text: Color,
    ) {
        self.style.background_color = background;
        self.style.hover_color = hover;
        self.style.pressed_color = pressed;
        self.style.border_color = border;
        self.style.text_color = text;
    }

    pub fn update(&mut self, mouse: Vector2, rl: &RaylibHandle) {
        if !self.enabled {
            self.is_hovered = false;
            self.is_pressed = false;
            return;
        }

        self.is_hovered = self.bounds.check_collision_point_rec(mouse);

        // Update pressed state
        if self.is_hovered {
            if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                self.is_pressed = true;
            } else {
                self.is_pressed = false;
            }
        } else {
            self.is_pressed = false;
        }

        // Update animation
        let target_progress = if self.is_pressed {
            1.0
        } else if self.is_hovered {
            0.5
        } else {
            0.0
        };

        let animation_speed = 8.0;
        if self.animation_progress < target_progress {
            self.animation_progress += rl.get_frame_time() * animation_speed;
            if self.animation_progress > target_progress {
                self.animation_progress = target_progress;
            }
        } else if self.animation_progress > target_progress {
            self.animation_progress -= rl.get_frame_time() * animation_speed;
            if self.animation_progress < target_progress {
                self.animation_progress = target_progress;
            }
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw) {
        let current_color = if !self.enabled {
            self.style.disabled_color
        } else {
            let base_color = self.style.background_color;
            let hover_color = self.style.hover_color;
            let pressed_color = self.style.pressed_color;

            if self.animation_progress <= 0.5 {
                // Implement our own color interpolation
                let t = self.animation_progress * 2.0;
                Color::new(
                    ((hover_color.r as f32 - base_color.r as f32) * t + base_color.r as f32) as u8,
                    ((hover_color.g as f32 - base_color.g as f32) * t + base_color.g as f32) as u8,
                    ((hover_color.b as f32 - base_color.b as f32) * t + base_color.b as f32) as u8,
                    ((hover_color.a as f32 - base_color.a as f32) * t + base_color.a as f32) as u8,
                )
            } else {
                let t = (self.animation_progress - 0.5) * 2.0;
                Color::new(
                    ((pressed_color.r as f32 - hover_color.r as f32) * t + hover_color.r as f32)
                        as u8,
                    ((pressed_color.g as f32 - hover_color.g as f32) * t + hover_color.g as f32)
                        as u8,
                    ((pressed_color.b as f32 - hover_color.b as f32) * t + hover_color.b as f32)
                        as u8,
                    ((pressed_color.a as f32 - hover_color.a as f32) * t + hover_color.a as f32)
                        as u8,
                )
            }
        };

        // Draw button background with rounded corners
        d.draw_rectangle_rounded(self.bounds, self.style.corner_radius, 8, current_color);

        // Draw border
        let border_color = if self.is_pressed {
            self.style.border_color_pressed
        } else if self.is_hovered {
            self.style.border_color_hover
        } else {
            self.style.border_color
        };

        d.draw_rectangle_rounded_lines(self.bounds, self.style.corner_radius, 8, border_color);

        // Calculate text position for centering
        let text_width = unsafe {
            raylib::ffi::MeasureText(self.label.as_ptr() as *const i8, self.style.font_size)
        };
        let text_x = self.bounds.x + (self.bounds.width - text_width as f32) / 2.0;
        let text_y = self.bounds.y + (self.bounds.height - self.style.font_size as f32) / 2.0;

        // Check if text fits, if not truncate with ellipsis
        let display_text = if text_width as f32 > self.bounds.width - self.style.padding * 2.0 {
            let mut truncated = self.label.clone();
            while unsafe {
                raylib::ffi::MeasureText(
                    (truncated.clone() + "...").as_ptr() as *const i8,
                    self.style.font_size,
                )
            } as f32
                > self.bounds.width - self.style.padding * 2.0
            {
                truncated.pop();
                if truncated.is_empty() {
                    break;
                }
            }
            truncated + "..."
        } else {
            self.label.clone()
        };

        // Recalculate position for truncated text
        let final_text_width = unsafe {
            raylib::ffi::MeasureText(display_text.as_ptr() as *const i8, self.style.font_size)
        };
        let final_text_x = self.bounds.x + (self.bounds.width - final_text_width as f32) / 2.0;

        // Draw text with slight offset when pressed
        let (text_offset_x, text_offset_y) = if self.is_pressed {
            (1.0, 1.0)
        } else {
            (0.0, 0.0)
        };

        let text_color = if self.enabled {
            if self.is_pressed {
                self.style.text_color_pressed
            } else if self.is_hovered {
                self.style.text_color_hover
            } else {
                self.style.text_color
            }
        } else {
            self.style.text_color_disabled
        };

        d.draw_text(
            &display_text,
            (final_text_x + text_offset_x) as i32,
            (text_y + text_offset_y) as i32,
            self.style.font_size,
            text_color,
        );
    }

    pub fn is_clicked(&self, rl: &RaylibHandle) -> bool {
        self.enabled
            && self.is_hovered
            && rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
    }
}

