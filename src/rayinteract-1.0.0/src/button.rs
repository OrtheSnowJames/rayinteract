use raylib::prelude::*;

pub struct Button {
    bounds: Rectangle,
    label: String,
    background_color: Color,
    hover_color: Color,
    pressed_color: Color,
    border_color: Color,
    text_color: Color,
    font_size: i32,
    is_hovered: bool,
    is_pressed: bool,
    animation_progress: f32,
    padding: f32,
    corner_radius: f32,
    enabled: bool,
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, label: &str) -> Self {
        Self {
            bounds: Rectangle::new(x, y, width, height),
            label: label.to_string(),
            background_color: Color::LIGHTGRAY,
            hover_color: Color::new(200, 200, 200, 255),
            pressed_color: Color::DARKGRAY,
            border_color: Color::BLACK,
            text_color: Color::BLACK,
            font_size: 20,
            is_hovered: false,
            is_pressed: false,
            animation_progress: 0.0,
            padding: 5.0,
            corner_radius: 5.0,
            enabled: true,
        }
    }

    pub fn set_colors(
        &mut self,
        background: Color,
        hover: Color,
        pressed: Color,
        border: Color,
        text: Color,
    ) {
        self.background_color = background;
        self.hover_color = hover;
        self.pressed_color = pressed;
        self.border_color = border;
        self.text_color = text;
    }

    pub fn set_font_size(&mut self, size: i32) {
        self.font_size = size;
    }

    pub fn set_corner_radius(&mut self, radius: f32) {
        self.corner_radius = radius;
    }

    pub fn set_padding(&mut self, padding: f32) {
        self.padding = padding;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        if !self.enabled {
            self.is_hovered = false;
            self.is_pressed = false;
            return;
        }

        let mouse_pos = rl.get_mouse_position();
        let old_hovered = self.is_hovered;
        self.is_hovered = self.bounds.check_collision_point_rec(mouse_pos);

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
    
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let current_color = if !self.enabled {
            self.background_color.fade(0.5) 
    } else {
        let base_color = self.background_color;
        let hover_color = self.hover_color;
        let pressed_color = self.pressed_color;

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
                ((pressed_color.r as f32 - hover_color.r as f32) * t + hover_color.r as f32) as u8,
                ((pressed_color.g as f32 - hover_color.g as f32) * t + hover_color.g as f32) as u8,
                ((pressed_color.b as f32 - hover_color.b as f32) * t + hover_color.b as f32) as u8,
                ((pressed_color.a as f32 - hover_color.a as f32) * t + hover_color.a as f32) as u8,
            )
        }
    };

    // Draw button background with rounded corners
    d.draw_rectangle_rounded(self.bounds, self.corner_radius, 8, current_color);

    // Draw border
    let border_thickness = if self.is_pressed { 3.0 } else { 2.0 };
    d.draw_rectangle_rounded_lines(
        self.bounds,
        self.corner_radius,
        8,
        border_thickness,
        self.border_color,
    );

    // Calculate text position for centering
    let text_width = d.measure_text(&self.label, self.font_size);
    let text_x = self.bounds.x + (self.bounds.width - text_width as f32) / 2.0;
    let text_y = self.bounds.y + (self.bounds.height - self.font_size as f32) / 2.0;

    // Draw text with slight offset when pressed
    let (text_offset_x, text_offset_y) = if self.is_pressed {
        (1.0, 1.0)
    } else {
        (0.0, 0.0)
    };

    let text_color = if self.enabled {
        self.text_color
    } else {
        self.text_color.fade(0.5) // Using the new .fade() method
    };

    d.draw_text(
        &self.label,
        (text_x + text_offset_x) as i32,
        (text_y + text_offset_y) as i32,
        self.font_size,
        text_color,
    );
    }

    pub fn is_clicked(&self, rl: &RaylibHandle) -> bool {
        self.enabled
            && self.is_hovered
            && rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
    }

    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }

    pub fn is_hovered(&self) -> bool {
        self.is_hovered
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }

    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_string();
    }
}

