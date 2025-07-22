use raylib::prelude::*;
use crate::style::Style;

pub struct TextField {
    pub bounds: Rectangle,
    pub text: String,
    pub placeholder: String,
    pub max_length: usize,
    pub style: Style,
    pub is_active: bool,
    pub cursor_position: usize,
    pub cursor_blink_timer: f32,
    pub backspace_hold_timer: f32,
    pub arrow_hold_timer: f32, // New timer for arrow keys
    pub scroll_offset: usize, // New field for text scrolling
    pub is_scrolling: bool, // Track if we're currently scrolling
}

impl TextField {
    pub fn new(x: f32, y: f32, width: f32, height: f32, max_length: usize) -> Self {
        Self {
            bounds: Rectangle::new(x, y, width, height),
            text: String::new(),
            placeholder: String::new(),
            max_length,
            style: Style::default(),
            is_active: false,
            cursor_position: 0,
            cursor_blink_timer: 0.0,
            backspace_hold_timer: 0.0,
            arrow_hold_timer: 0.0,
            scroll_offset: 0,
            is_scrolling: false,
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn set_colors(&mut self, background_color: Color, border_color: Color, text_color: Color) {
        self.style.background_color = background_color;
        self.style.border_color = border_color;
        self.style.text_color = text_color;
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
            
            // Handle scroll bar interaction
            if self.bounds.check_collision_point_rec(mouse_pos) {
                let scroll_bar_height = 3.0;
                let scroll_bar_y = self.bounds.y + self.bounds.height - scroll_bar_height;
                let scroll_bar_bounds = Rectangle::new(self.bounds.x, scroll_bar_y, self.bounds.width, scroll_bar_height);
                
                if scroll_bar_bounds.check_collision_point_rec(mouse_pos) {
                    self.is_scrolling = true;
                    // Calculate scroll position based on mouse click
                    let relative_x = mouse_pos.x - self.bounds.x;
                    let scroll_ratio = (relative_x / self.bounds.width).max(0.0).min(1.0);
                    let max_visible_chars = self.get_max_visible_chars();
                    let max_scroll = if self.text.len() > max_visible_chars {
                        self.text.len() - max_visible_chars
                    } else {
                        0
                    };
                    self.scroll_offset = (scroll_ratio * max_scroll as f32) as usize;
                    self.scroll_offset = self.scroll_offset.min(max_scroll);
                }
            }
        }

        // Handle scroll bar dragging - continue even if mouse moves outside scroll bar
        if self.is_scrolling && rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_pos = rl.get_mouse_position();
            let relative_x = mouse_pos.x - self.bounds.x;
            let scroll_ratio = (relative_x / self.bounds.width).max(0.0).min(1.0);
            let max_visible_chars = self.get_max_visible_chars();
            let max_scroll = if self.text.len() > max_visible_chars {
                self.text.len() - max_visible_chars
            } else {
                0
            };
            self.scroll_offset = (scroll_ratio * max_scroll as f32) as usize;
            self.scroll_offset = self.scroll_offset.min(max_scroll);
        }

        // Stop scrolling when mouse button is released
        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            self.is_scrolling = false;
        }

        // Handle keyboard input when active
        if self.is_active {
            // Handle character input
            if let Some(c) = rl.get_char_pressed() {
                if self.text.len() < self.max_length {
                    self.text.insert(self.cursor_position, c);
                    self.cursor_position += 1;
                    self.update_scroll_offset();
                }
            }

            // Handle special keys (single press)
            if let Some(key) = rl.get_key_pressed() {
                match key {
                    KeyboardKey::KEY_BACKSPACE => {
                        if self.cursor_position > 0 {
                            self.text.remove(self.cursor_position - 1);
                            self.cursor_position -= 1;
                            self.update_scroll_offset();
                        }
                    },
                    KeyboardKey::KEY_LEFT => {
                        if self.cursor_position > 0 {
                            self.cursor_position -= 1;
                            self.update_scroll_offset();
                        }
                    },
                    KeyboardKey::KEY_RIGHT => {
                        if self.cursor_position < self.text.len() {
                            self.cursor_position += 1;
                            self.update_scroll_offset();
                        }
                    },
                    KeyboardKey::KEY_HOME => {
                        self.cursor_position = 0;
                        self.update_scroll_offset();
                    },
                    KeyboardKey::KEY_END => {
                        self.cursor_position = self.text.len();
                        self.update_scroll_offset();
                    },
                    _ => {}
                }
            }

            // Handle held keys (continuous input)
            let frame_time = rl.get_frame_time();
            
            // Handle held arrow keys with faster movement
            if rl.is_key_down(KeyboardKey::KEY_LEFT) {
                self.arrow_hold_timer += frame_time;
                if self.arrow_hold_timer > 0.1 { // Faster repeat rate for arrows
                    if self.cursor_position > 0 {
                        self.cursor_position -= 1;
                        self.update_scroll_offset();
                    }
                    self.arrow_hold_timer = 0.05; // Small delay between movements
                }
            } else if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
                self.arrow_hold_timer += frame_time;
                if self.arrow_hold_timer > 0.1 { // Faster repeat rate for arrows
                    if self.cursor_position < self.text.len() {
                        self.cursor_position += 1;
                        self.update_scroll_offset();
                    }
                    self.arrow_hold_timer = 0.05; // Small delay between movements
                }
            } else {
                // Reset timer when no arrow keys are held
                self.arrow_hold_timer = 0.0;
            }

            // Handle holding backspace with gradual deletion
            if rl.is_key_down(KeyboardKey::KEY_BACKSPACE) {
                self.backspace_hold_timer += frame_time;
                if self.backspace_hold_timer > 0.5 { // Initial delay
                    if self.backspace_hold_timer > 0.6 { // Start deleting after delay
                        if self.cursor_position > 0 {
                            self.text.remove(self.cursor_position - 1);
                            self.cursor_position -= 1;
                            self.update_scroll_offset();
                        }
                        self.backspace_hold_timer = 0.55; // Slower repeat rate for backspace
                    }
                }
            } else {
                // Reset backspace timer when not held
                self.backspace_hold_timer = 0.0;
            }
        }
    }

    fn get_max_visible_chars(&self) -> usize {
        let visible_width = self.bounds.width - self.style.padding * 2.0;
        let char_width = self.style.font_size as f32 * 0.6; // Approximate character width
        (visible_width / char_width) as usize
    }

    fn update_scroll_offset(&mut self) {
        // Calculate how much text can fit in the visible area
        let max_visible_chars = self.get_max_visible_chars();
        
        // If cursor is beyond visible area, adjust scroll offset
        if self.cursor_position > self.scroll_offset + max_visible_chars {
            self.scroll_offset = self.cursor_position - max_visible_chars;
        } else if self.cursor_position < self.scroll_offset {
            self.scroll_offset = self.cursor_position;
        }
        
        // Ensure scroll offset doesn't go negative
        self.scroll_offset = self.scroll_offset.max(0);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        // Draw background
        d.draw_rectangle_rec(self.bounds, self.style.background_color);
        
        // Draw border (active color if active, normal color if not)
        let border_color = if self.is_active { 
            self.style.border_color_active 
        } else { 
            self.style.border_color 
        };
        d.draw_rectangle_lines_ex(self.bounds, self.style.border_thickness, border_color);

        // Draw text or placeholder
        let text_to_draw = if self.text.is_empty() && !self.placeholder.is_empty() {
            &self.placeholder
        } else {
            &self.text
        };

        let text_color = if self.text.is_empty() && !self.placeholder.is_empty() {
            self.style.placeholder_color
        } else {
            self.style.text_color
        };

        let text_y = (self.bounds.y + (self.bounds.height - self.style.font_size as f32) / 2.0) as i32;
        
        // Draw visible portion of text with scrolling
        if !self.text.is_empty() {
            let visible_text = if self.scroll_offset < text_to_draw.len() {
                &text_to_draw[self.scroll_offset..]
            } else {
                ""
            };
            
            d.draw_text(
                visible_text,
                (self.bounds.x + self.style.padding) as i32,
                text_y,
                self.style.font_size,
                text_color,
            );
        } else {
            d.draw_text(
                text_to_draw,
                (self.bounds.x + self.style.padding) as i32,
                text_y,
                self.style.font_size,
                text_color,
            );
        }

        // Draw cursor when active
        if self.is_active && self.cursor_blink_timer < 0.5 {
            let cursor_visible_pos = if self.cursor_position > self.scroll_offset {
                self.cursor_position - self.scroll_offset
            } else {
                0
            };
            
            let text_width = if cursor_visible_pos > 0 {
                let visible_text = &self.text[self.scroll_offset..self.scroll_offset + cursor_visible_pos];
                d.measure_text(visible_text, self.style.font_size) as f32
            } else {
                0.0
            };
            
            d.draw_line(
                (self.bounds.x + self.style.padding + text_width) as i32,
                text_y,
                (self.bounds.x + self.style.padding + text_width) as i32,
                text_y + self.style.font_size,
                self.style.text_color,
            );
        }

        // Draw scroll indicator if text is scrolled
        let max_visible_chars = self.get_max_visible_chars();
        if self.scroll_offset > 0 || self.text.len() > max_visible_chars {
            let scroll_bar_height = 3.0;
            let scroll_bar_y = self.bounds.y + self.bounds.height - scroll_bar_height;
            let scroll_bar_width = self.bounds.width;
            
            // Background of scroll bar
            d.draw_rectangle_rec(
                Rectangle::new(self.bounds.x, scroll_bar_y, scroll_bar_width, scroll_bar_height),
                Color::new(200, 200, 200, 100)
            );
            
            // Scroll indicator
            let max_scroll = if self.text.len() > max_visible_chars {
                self.text.len() - max_visible_chars
            } else {
                0
            };
            let scroll_ratio = if max_scroll > 0 {
                (self.scroll_offset as f32 / max_scroll as f32).min(1.0)
            } else {
                0.0
            };
            let indicator_width = (scroll_bar_width * (max_visible_chars as f32 / self.text.len().max(1) as f32)).min(scroll_bar_width);
            let indicator_x = self.bounds.x + scroll_ratio * (scroll_bar_width - indicator_width);
            
            d.draw_rectangle_rec(
                Rectangle::new(indicator_x, scroll_bar_y, indicator_width, scroll_bar_height),
                Color::new(100, 100, 100, 200)
            );
        }
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor_position = 0;
        self.scroll_offset = 0;
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.cursor_position = self.text.len();
        self.update_scroll_offset();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn handle_input(&mut self, rl: &mut RaylibHandle) {
        self.update(rl);
    }
}

