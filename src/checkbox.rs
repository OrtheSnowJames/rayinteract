use raylib::prelude::*;
use crate::style::Style;

pub struct Checkbox {
    pub bounds: Rectangle,
    pub is_checked: bool,
    pub style: Style,
    pub label: String,
    pub is_hovered: bool,
    pub animation_progress: f32,
    pub is_clicked: bool,
}

impl Checkbox {
    pub fn new(x: f32, y: f32, size: f32, label: &str) -> Self {
        Self {
            bounds: Rectangle::new(x, y, size, size),
            is_checked: false,
            style: Style::default(),
            label: label.to_string(),
            is_hovered: false,
            animation_progress: 0.0,
            is_clicked: false,
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn set_colors(
        &mut self,
        background: Color,
        check: Color,
        border: Color,
        hover: Color,
        label: Color,
    ) {
        self.style.background_color = background;
        self.style.check_color = check;
        self.style.border_color = border;
        self.style.hover_color = hover;
        self.style.text_color = label;
    }

    pub fn update(&mut self, mouse: Vector2, rl: &RaylibHandle) {
        self.is_hovered = self.bounds.check_collision_point_rec(mouse);

        // Handle click animation
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && self.is_hovered {
            self.is_clicked = true;
            self.is_checked = !self.is_checked;
            self.animation_progress = 0.0;
        }

        // Update animation
        let animation_speed = 4.0;
        if self.is_checked && self.animation_progress < 1.0 {
            self.animation_progress += rl.get_frame_time() * animation_speed;
            if self.animation_progress > 1.0 {
                self.animation_progress = 1.0;
            }
        } else if !self.is_checked && self.animation_progress > 0.0 {
            self.animation_progress -= rl.get_frame_time() * animation_speed;
            if self.animation_progress < 0.0 {
                self.animation_progress = 0.0;
            }
        }

        // Reset click state
        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            self.is_clicked = false;
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw) {
        // Draw background
        let background_color = if self.is_hovered {
            self.style.hover_color
        } else {
            self.style.background_color
        };
        d.draw_rectangle_rec(self.bounds, background_color);

        // Draw border
        let border_thickness = if self.is_clicked { 3.0 } else { self.style.border_thickness };
        d.draw_rectangle_lines_ex(self.bounds, border_thickness, self.style.border_color);

        // Draw check mark with animation
        if self.animation_progress > 0.0 {
            let padding = self.bounds.width * 0.2;
            let check_bounds = Rectangle::new(
                self.bounds.x + padding,
                self.bounds.y + padding,
                self.bounds.width - padding * 2.0,
                self.bounds.height - padding * 2.0,
            );

            // Draw check mark
            let center_x = check_bounds.x + check_bounds.width / 2.0;
            let center_y = check_bounds.y + check_bounds.height / 2.0;
            let size = check_bounds.width / 2.0 * self.animation_progress;

            // Draw custom check mark
            let points = [
                Vector2::new(center_x - size, center_y),
                Vector2::new(center_x, center_y + size),
                Vector2::new(center_x + size, center_y - size),
            ];

            let check_color = Color::new(
                self.style.check_color.r,
                self.style.check_color.g,
                self.style.check_color.b,
                (self.style.check_color.a as f32 * self.animation_progress) as u8,
            );

            d.draw_line_ex(points[0], points[1], 2.0, check_color);
            d.draw_line_ex(points[1], points[2], 2.0, check_color);
        }

        // Draw label with proper alignment
        let label_x = self.bounds.x + self.bounds.width + 10.0;
        let label_y = self.bounds.y + (self.bounds.height - self.style.font_size as f32) / 2.0;

        d.draw_text(
            &self.label,
            label_x as i32,
            label_y as i32,
            self.style.font_size,
            self.style.text_color,
        );
    }

    pub fn toggle(&mut self) {
        self.is_checked = !self.is_checked;
        self.animation_progress = if self.is_checked { 0.0 } else { 1.0 };
    }
}
