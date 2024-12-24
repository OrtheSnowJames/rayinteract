use raylib::prelude::*;

pub struct TextField {
    bounds: Rectangle,
    text: String,
    max_length: usize,
    background_color: Color,
    border_color: Color,
    text_color: Color,
    font_size: i32,
    is_active: bool,
    cursor_position: usize,
    cursor_blink_timer: f32,
}
impl TextField {
 

    pub fn new(x: f32, y: f32, width: f32, height: f32, max_length: usize) -> Self {
        Self {
            bounds: Rectangle::new(x, y, width, height),
            text: String::new(),
            max_length,
            background_color: Color::WHITE,
            border_color: Color::BLACK,
            text_color: Color::BLACK,
            font_size: 20,
            is_active: false,
            cursor_position: 0,
            cursor_blink_timer: 0.0,
        }
    }

    pub fn set_colors(&mut self, background_color: Color, border_color: Color, text_color: Color) {
        self.background_color = background_color;
        self.border_color = border_color;
        self.text_color = text_color;
    }

    pub fn set_font_size(&mut self, font_size: i32) {
        self.font_size = font_size;
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        // Update cursor blink timer
        self.cursor_blink_timer += rl.get_frame_time();
        if self.cursor_blink_timer >= 1.0 {
            self.cursor_blink_timer = 0.0;
        }

        // Handle mouse input for focus
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_pos = rl.get_mouse_position();
            self.is_active = self.bounds.check_collision_point_rec(mouse_pos);
        }

        // Handle keyboard input when active
        if self.is_active {
            // Handle character input
            if let Some(c) = rl.get_char_pressed() {
                if self.text.len() < self.max_length {
                    self.text.insert(self.cursor_position, c);
                    self.cursor_position += 1;
                }
            }

            // Handle special keys
            if let Some(key) = rl.get_key_pressed() {
                match key {
                    KeyboardKey::KEY_BACKSPACE => {
                        if self.cursor_position > 0 {
                            self.text.remove(self.cursor_position - 1);
                            self.cursor_position -= 1;
                        }
                    },
                    KeyboardKey::KEY_LEFT => {
                        if self.cursor_position > 0 {
                            self.cursor_position -= 1;
                        }
                    },
                    KeyboardKey::KEY_RIGHT => {
                        if self.cursor_position < self.text.len() {
                            self.cursor_position += 1;
                        }
                    },
                    KeyboardKey::KEY_HOME => {
                        self.cursor_position = 0;
                    },
                    KeyboardKey::KEY_END => {
                        self.cursor_position = self.text.len();
                    },
                    _ => {}
                }
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        // Draw background
        d.draw_rectangle_rec(self.bounds, self.background_color);
        
        // Draw border (red if active, normal color if not)
        let border_color = if self.is_active { Color::RED } else { self.border_color };
        d.draw_rectangle_lines_ex(self.bounds, 2.0, border_color);

        // Draw text
        let text_y = (self.bounds.y + (self.bounds.height - self.font_size as f32) / 2.0) as i32;
        d.draw_text(
            &self.text,
            (self.bounds.x + 5.0) as i32,
            text_y,
            self.font_size,
            self.text_color,
        );

        // Draw cursor when active
        if self.is_active && self.cursor_blink_timer < 0.5 {
            let text_width = if self.cursor_position > 0 {
                d.measure_text(&self.text[..self.cursor_position], self.font_size) as f32
            } else {
                0.0
            };
            
            d.draw_line(
                (self.bounds.x + 5.0 + text_width) as i32,
                text_y,
                (self.bounds.x + 5.0 + text_width) as i32,
                text_y + self.font_size,
                self.text_color,
            );
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_value(&mut self, value: &str) {
        self.text = value.chars().take(self.max_length).collect();
        self.cursor_position = self.text.len();
    }
    
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.cursor_position = self.text.len();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}

