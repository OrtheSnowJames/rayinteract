use raylib::prelude::*;
use crate::style::Style;
use std::ffi::CString;
use regex::Regex;
use arboard::Clipboard;

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
    pub allowed_pattern: Option<Regex>,
    pub character_callback: Option<Box<dyn Fn(char) -> char>>,
    pub selection_anchor: Option<usize>, // Selection anchor for text selection
    pub backspace_repeat_timer: f32,
    pub backspace_repeat_active: bool,
    pub delete_repeat_timer: f32,
    pub delete_repeat_active: bool,
    pub arrow_repeat_timer: f32,
    pub arrow_repeat_active: bool,
    pub clipboard: Option<Clipboard>,
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
            allowed_pattern: None,
            character_callback: None,
            selection_anchor: None,
            backspace_repeat_timer: 0.0,
            backspace_repeat_active: false,
            delete_repeat_timer: 0.0,
            delete_repeat_active: false,
            arrow_repeat_timer: 0.0,
            arrow_repeat_active: false,
            clipboard: Clipboard::new().ok(),
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn with_placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    pub fn set_colors(&mut self, background_color: Color, border_color: Color, text_color: Color) {
        self.style.background_color = background_color;
        self.style.border_color = border_color;
        self.style.text_color = text_color;
    }

    pub fn only_allow(mut self, regex: Regex) -> Self {
        self.allowed_pattern = Some(regex);
        self
    }

    pub fn change_character<F: Fn(char) -> char + 'static>(mut self, callback: F) -> Self {
        self.character_callback = Some(Box::new(callback));
        self
    }

    pub fn update(&mut self, mouse: Vector2, rl: &mut RaylibHandle) {
        // Update cursor blink timer
        self.cursor_blink_timer += rl.get_frame_time();
        if self.cursor_blink_timer >= 1.0 {
            self.cursor_blink_timer = 0.0;
        }

        // Handle mouse input for focus
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_pos = mouse;
            let was_active = self.is_active;
            self.is_active = self.bounds.check_collision_point_rec(mouse_pos);
            if !self.is_active {
                // Clicked outside, deactivate
                self.deactivate();
            }
            // Click to move cursor
            if self.is_active {
                let available_width = self.bounds.width - self.style.padding * 2.0;
                let text_slice = &self.text[self.scroll_offset..];
                let mut end = text_slice.len();
                let mut last_good = 0;
                for (i, _) in text_slice.char_indices() {
                    let candidate = &text_slice[..i];
                    let cstr = std::ffi::CString::new(candidate).unwrap_or_default();
                    let w = unsafe { raylib::ffi::MeasureText(cstr.as_ptr(), self.style.font_size) } as f32;
                    if w > available_width {
                        end = last_good;
                        break;
                    }
                    last_good = i;
                }
                let visible_text = &text_slice[..end];
                let click_x = mouse_pos.x - self.bounds.x - self.style.padding;
                let mut closest = 0;
                let mut min_dist = f32::MAX;
                let mut acc_width = 0.0;
                let mut char_idx = 0;
                for (i, ch) in visible_text.char_indices() {
                    let cstr = std::ffi::CString::new(&visible_text[..i]).unwrap_or_default();
                    let w = unsafe { raylib::ffi::MeasureText(cstr.as_ptr(), self.style.font_size) } as f32;
                    let cstr_next = std::ffi::CString::new(&visible_text[..i + ch.len_utf8()]).unwrap_or_default();
                    let w_next = unsafe { raylib::ffi::MeasureText(cstr_next.as_ptr(), self.style.font_size) } as f32;
                    let char_center = (w + w_next) / 2.0;
                    let dist = (click_x - char_center).abs();
                    if dist < min_dist {
                        min_dist = dist;
                        closest = char_idx;
                    }
                    char_idx += 1;
                }
                // If click is past the end, put cursor at end
                let cstr = std::ffi::CString::new(visible_text).unwrap_or_default();
                let w = unsafe { raylib::ffi::MeasureText(cstr.as_ptr(), self.style.font_size) } as f32;
                if click_x > w {
                    closest = visible_text.chars().count();
                }
                self.cursor_position = self.scroll_offset + closest;
                self.update_scroll_offset();
            }

            // Handle scroll bar interaction
            if self.bounds.check_collision_point_rec(mouse_pos) {
                let scroll_bar_height = 12.0; // Increased height for easier interaction
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
            let mouse_pos = mouse;
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
            // Clamp cursor to visible text range after scrolling
            let start = self.scroll_offset;
            let end = (self.scroll_offset + max_visible_chars).min(self.text.len());
            self.cursor_position = self.cursor_position.clamp(start, end);
        }

        // Stop scrolling when mouse button is released
        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            self.is_scrolling = false;
        }

        // Handle keyboard input when active
        if self.is_active {
            // Use persistent clipboard
            let ctrl = rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL);
            let shift = rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT);

            // Ctrl+A: select all
            if ctrl && rl.is_key_pressed(KeyboardKey::KEY_A) {
                self.selection_anchor = Some(0);
                self.cursor_position = self.text.len();
                self.update_scroll_offset();
            }
            // Ctrl+C: copy
            if ctrl && rl.is_key_pressed(KeyboardKey::KEY_C) {
                if let Some(anchor) = self.selection_anchor {
                    if anchor != self.cursor_position {
                        let (start, end) = if anchor < self.cursor_position {
                            (anchor, self.cursor_position)
                        } else {
                            (self.cursor_position, anchor)
                        };
                        if let Some(cb) = self.clipboard.as_mut() {
                            let _ = cb.set_text(self.text[start..end].to_string());
                        }
                    }
                }
            }
            // Ctrl+X: cut
            if ctrl && rl.is_key_pressed(KeyboardKey::KEY_X) {
                if let Some(anchor) = self.selection_anchor {
                    if anchor != self.cursor_position {
                        let (start, end) = if anchor < self.cursor_position {
                            (anchor, self.cursor_position)
                        } else {
                            (self.cursor_position, anchor)
                        };
                        if let Some(cb) = self.clipboard.as_mut() {
                            let _ = cb.set_text(self.text[start..end].to_string());
                        }
                        self.text.replace_range(start..end, "");
                        self.cursor_position = start;
                        self.selection_anchor = None;
                        self.update_scroll_offset();
                    }
                }
            }
            // Ctrl+V: paste
            if ctrl && rl.is_key_pressed(KeyboardKey::KEY_V) {
                let mut paste_text = None;
                if let Some(cb) = self.clipboard.as_mut() {
                    if let Ok(paste) = cb.get_text() {
                        paste_text = Some(paste);
                    }
                }
                if let Some(paste) = paste_text {
                    // Remove selection if any
                    if let Some(anchor) = self.selection_anchor {
                        if anchor != self.cursor_position {
                            let (start, end) = if anchor < self.cursor_position {
                                (anchor, self.cursor_position)
                            } else {
                                (self.cursor_position, anchor)
                            };
                            self.text.replace_range(start..end, "");
                            self.cursor_position = start;
                        }
                    }
                    let insert = paste.chars().take(self.max_length - self.text.len()).collect::<String>();
                    self.text.insert_str(self.cursor_position, &insert);
                    self.cursor_position += insert.len();
                    self.selection_anchor = None;
                    self.update_scroll_offset();
                }
            }

            // Helper: clear selection if not holding shift
            let mut clear_selection = || {
                if !shift {
                    self.selection_anchor = None;
                }
            };

            // Helper: get selection range
            let get_selection = |anchor: usize, cursor: usize| -> (usize, usize) {
                if anchor < cursor { (anchor, cursor) } else { (cursor, anchor) }
            };

            // --- Standard key repeat logic for selection/navigation (arrows, home/end) ---
            let arrow_repeat_delay = 0.45;
            let arrow_repeat_interval = 0.05;
            let mut arrow_action = None;
            // Detect which navigation key is pressed/held
            let left = rl.is_key_down(KeyboardKey::KEY_LEFT);
            let right = rl.is_key_down(KeyboardKey::KEY_RIGHT);
            let home = rl.is_key_down(KeyboardKey::KEY_HOME);
            let end = rl.is_key_down(KeyboardKey::KEY_END);
            let nav_any = left || right || home || end;
            // On first press
            if rl.is_key_pressed(KeyboardKey::KEY_LEFT) {
                arrow_action = Some("left");
                self.arrow_repeat_timer = 0.0;
                self.arrow_repeat_active = true;
            } else if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
                arrow_action = Some("right");
                self.arrow_repeat_timer = 0.0;
                self.arrow_repeat_active = true;
            } else if rl.is_key_pressed(KeyboardKey::KEY_HOME) {
                arrow_action = Some("home");
                self.arrow_repeat_timer = 0.0;
                self.arrow_repeat_active = true;
            } else if rl.is_key_pressed(KeyboardKey::KEY_END) {
                arrow_action = Some("end");
                self.arrow_repeat_timer = 0.0;
                self.arrow_repeat_active = true;
            } else if nav_any && self.arrow_repeat_active {
                self.arrow_repeat_timer += rl.get_frame_time();
                if self.arrow_repeat_timer > arrow_repeat_delay {
                    if left { arrow_action = Some("left"); }
                    if right { arrow_action = Some("right"); }
                    if home { arrow_action = Some("home"); }
                    if end { arrow_action = Some("end"); }
                    self.arrow_repeat_timer -= arrow_repeat_interval;
                }
            } else if !nav_any {
                self.arrow_repeat_active = false;
                self.arrow_repeat_timer = 0.0;
            }
            // Perform navigation action
            if let Some(action) = arrow_action {
                match action {
                    "left" => {
                        if shift {
                            if self.selection_anchor.is_none() {
                                self.selection_anchor = Some(self.cursor_position);
                            }
                        } else {
                            self.selection_anchor = None;
                        }
                        if ctrl {
                            // Move to previous word
                            if self.cursor_position > 0 {
                                let mut idx = self.cursor_position;
                                let chars = self.text[..idx].char_indices().collect::<Vec<_>>();
                                let mut word_start = 0;
                                for (i, (pos, ch)) in chars.iter().enumerate().rev() {
                                    if ch.is_whitespace() && i != chars.len() - 1 {
                                        word_start = chars[i + 1].0;
                                        break;
                                    }
                                }
                                self.cursor_position = word_start;
                                self.update_scroll_offset();
                            }
                        } else {
                            if self.cursor_position > 0 {
                                self.cursor_position -= 1;
                                self.update_scroll_offset();
                            }
                        }
                    },
                    "right" => {
                        if shift {
                            if self.selection_anchor.is_none() {
                                self.selection_anchor = Some(self.cursor_position);
                            }
                        } else {
                            self.selection_anchor = None;
                        }
                        if ctrl {
                            // Move to next word
                            if self.cursor_position < self.text.len() {
                                let chars = self.text[self.cursor_position..].char_indices().collect::<Vec<_>>();
                                let mut word_end = self.text.len();
                                for (i, (pos, ch)) in chars.iter().enumerate() {
                                    if ch.is_whitespace() && i != 0 {
                                        word_end = self.cursor_position + chars[i].0;
                                        break;
                                    }
                                }
                                self.cursor_position = word_end;
                                self.update_scroll_offset();
                            }
                        } else {
                            if self.cursor_position < self.text.len() {
                                self.cursor_position += 1;
                                self.update_scroll_offset();
                            }
                        }
                    },
                    "home" => {
                        if shift {
                            if self.selection_anchor.is_none() {
                                self.selection_anchor = Some(self.cursor_position);
                            }
                        } else {
                            self.selection_anchor = None;
                        }
                        self.cursor_position = 0;
                        self.update_scroll_offset();
                    },
                    "end" => {
                        if shift {
                            if self.selection_anchor.is_none() {
                                self.selection_anchor = Some(self.cursor_position);
                            }
                        } else {
                            self.selection_anchor = None;
                        }
                        self.cursor_position = self.text.len();
                        self.update_scroll_offset();
                    },
                    _ => {}
                }
            }

            let repeat_delay = 0.5;
            let repeat_interval = 0.05;
            
            let mut selection_deleted = false;
            if let Some(anchor) = self.selection_anchor {
                if anchor != self.cursor_position {
                    let mut should_delete = false;
                    // Backspace
                    if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
                        should_delete = true;
                        self.backspace_repeat_timer = 0.0;
                        self.backspace_repeat_active = true;
                    } else if rl.is_key_down(KeyboardKey::KEY_BACKSPACE) && self.backspace_repeat_active {
                        self.backspace_repeat_timer += rl.get_frame_time();
                        if self.backspace_repeat_timer > repeat_delay {
                            should_delete = true;
                            self.backspace_repeat_timer -= repeat_interval;
                        }
                    } else if !rl.is_key_down(KeyboardKey::KEY_BACKSPACE) {
                        self.backspace_repeat_active = false;
                        self.backspace_repeat_timer = 0.0;
                    }
                    // Delete
                    if rl.is_key_pressed(KeyboardKey::KEY_DELETE) {
                        should_delete = true;
                        self.delete_repeat_timer = 0.0;
                        self.delete_repeat_active = true;
                    } else if rl.is_key_down(KeyboardKey::KEY_DELETE) && self.delete_repeat_active {
                        self.delete_repeat_timer += rl.get_frame_time();
                        if self.delete_repeat_timer > repeat_delay {
                            should_delete = true;
                            self.delete_repeat_timer -= repeat_interval;
                        }
                    } else if !rl.is_key_down(KeyboardKey::KEY_DELETE) {
                        self.delete_repeat_active = false;
                        self.delete_repeat_timer = 0.0;
                    }
                    if should_delete {
                        let (start, end) = get_selection(anchor, self.cursor_position);
                        if start < end && end <= self.text.len() {
                            self.text.replace_range(start..end, "");
                            self.cursor_position = start;
                            self.selection_anchor = None;
                            self.update_scroll_offset();
                            selection_deleted = true;
                        }
                    }
                }
            }
            // Normal backspace/delete repeat logic (if no selection)
            if !selection_deleted {
                let mut should_backspace = false;
                let mut should_delete = false;
                // Backspace
                if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
                    should_backspace = true;
                    self.backspace_repeat_timer = 0.0;
                    self.backspace_repeat_active = true;
                } else if rl.is_key_down(KeyboardKey::KEY_BACKSPACE) && self.backspace_repeat_active {
                    self.backspace_repeat_timer += rl.get_frame_time();
                    if self.backspace_repeat_timer > repeat_delay {
                        should_backspace = true;
                        self.backspace_repeat_timer -= repeat_interval;
                    }
                } else if !rl.is_key_down(KeyboardKey::KEY_BACKSPACE) {
                    self.backspace_repeat_active = false;
                    self.backspace_repeat_timer = 0.0;
                }
                // Delete
                if rl.is_key_pressed(KeyboardKey::KEY_DELETE) {
                    should_delete = true;
                    self.delete_repeat_timer = 0.0;
                    self.delete_repeat_active = true;
                } else if rl.is_key_down(KeyboardKey::KEY_DELETE) && self.delete_repeat_active {
                    self.delete_repeat_timer += rl.get_frame_time();
                    if self.delete_repeat_timer > repeat_delay {
                        should_delete = true;
                        self.delete_repeat_timer -= repeat_interval;
                    }
                } else if !rl.is_key_down(KeyboardKey::KEY_DELETE) {
                    self.delete_repeat_active = false;
                    self.delete_repeat_timer = 0.0;
                }
                if should_backspace {
                    if self.cursor_position > 0 {
                        self.text.remove(self.cursor_position - 1);
                        self.cursor_position -= 1;
                        self.update_scroll_offset();
                    }
                }
                if should_delete {
                    if self.cursor_position < self.text.len() {
                        self.text.remove(self.cursor_position);
                        self.update_scroll_offset();
                    }
                }
            }

            // Handle character input
            if let Some(c) = rl.get_char_pressed() {
                if self.text.len() < self.max_length {
                    let processed_char = self.process_character(c);
                    self.text.insert(self.cursor_position, processed_char);
                    self.cursor_position += 1;
                    self.update_scroll_offset();
                }
            }
        }
    }

    fn get_max_visible_chars(&self) -> usize {
        // Instead of returning a character count, return the number of characters that fit in the visible width using pixel-accurate measurement
        let visible_width = self.bounds.width - self.style.padding * 2.0;
        let mut width = 0.0;
        let mut count = 0;
        let text_slice = &self.text[self.scroll_offset..];
        for (i, ch) in text_slice.char_indices() {
            let candidate = &text_slice[..i + ch.len_utf8()];
            let cstr = std::ffi::CString::new(candidate).unwrap_or_default();
            let w = unsafe { raylib::ffi::MeasureText(cstr.as_ptr(), self.style.font_size) } as f32;
            if w > visible_width {
                break;
            }
            count += 1;
        }
        count
    }

    fn update_scroll_offset(&mut self) {
        // Use pixel-accurate measurement to ensure the cursor is always visible
        let visible_width = self.bounds.width - self.style.padding * 2.0;
        let text_len = self.text.len();
        let mut start = self.scroll_offset;
        let mut end = start;
        let mut found_cursor = false;
        let mut cursor_x = 0.0;
        // Find the visible window such that the cursor is always visible
        while end < text_len {
            let slice = &self.text[start..end];
            let cstr = std::ffi::CString::new(slice).unwrap_or_default();
            let w = unsafe { raylib::ffi::MeasureText(cstr.as_ptr(), self.style.font_size) } as f32;
            if w > visible_width {
                break;
            }
            if end == self.cursor_position {
                found_cursor = true;
                cursor_x = w;
            }
            // Move to next char
            if let Some((next_idx, _)) = self.text[end..].char_indices().next() {
                end += next_idx + 1;
            } else {
                end = text_len;
                break;
            }
        }
        // If cursor is not visible, adjust scroll_offset
        if self.cursor_position < start {
            self.scroll_offset = self.cursor_position;
        } else if self.cursor_position > end {
            // Move scroll_offset forward so cursor is at the end
            self.scroll_offset = self.cursor_position;
            // Recompute window
            let mut temp_start = self.scroll_offset;
            let mut temp_end = temp_start;
            while temp_start > 0 {
                let slice = &self.text[temp_start..self.cursor_position];
                let cstr = std::ffi::CString::new(slice).unwrap_or_default();
                let w = unsafe { raylib::ffi::MeasureText(cstr.as_ptr(), self.style.font_size) } as f32;
                if w > visible_width {
                    break;
                }
                temp_start -= 1;
            }
            self.scroll_offset = temp_start + 1;
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw) {
        // Draw background
        d.draw_rectangle_rec(self.bounds, self.style.background_color);
        
        // Draw border (active color if active, normal color if not)
        let border_color = if self.is_active { 
            self.style.border_color_active 
        } else { 
            self.style.border_color 
        };
        d.draw_rectangle_lines_ex(self.bounds, self.style.border_thickness, border_color);

        // Draw selection highlight if any
        if let Some(anchor) = self.selection_anchor {
            if anchor != self.cursor_position {
                let (start, end) = if anchor < self.cursor_position {
                    (anchor, self.cursor_position)
                } else {
                    (self.cursor_position, anchor)
                };
                // Only draw highlight for visible part
                let visible_start = start.max(self.scroll_offset);
                let visible_end = end.min(self.scroll_offset + self.text.len() - self.scroll_offset);
                let text_slice = &self.text[self.scroll_offset..];
                let mut x_start = 0.0;
                let mut x_end = 0.0;
                let mut char_count = 0;
                for (i, ch) in text_slice.char_indices() {
                    let cstr = std::ffi::CString::new(&text_slice[..i]).unwrap_or_default();
                    let w = unsafe { raylib::ffi::MeasureText(cstr.as_ptr(), self.style.font_size) } as f32;
                    if self.scroll_offset + char_count == visible_start {
                        x_start = w;
                    }
                    if self.scroll_offset + char_count == visible_end {
                        x_end = w;
                        break;
                    }
                    char_count += 1;
                }
                // If selection goes to end
                if visible_end == self.scroll_offset + text_slice.chars().count() {
                    let cstr = std::ffi::CString::new(text_slice).unwrap_or_default();
                    x_end = unsafe { raylib::ffi::MeasureText(cstr.as_ptr(), self.style.font_size) } as f32;
                }
                let highlight_x = self.bounds.x + self.style.padding + x_start;
                let highlight_y = self.bounds.y + (self.bounds.height - self.style.font_size as f32) / 2.0;
                let highlight_w = (x_end - x_start).max(2.0);
                let highlight_h = self.style.font_size as f32;
                d.draw_rectangle_rec(
                    Rectangle::new(highlight_x, highlight_y, highlight_w, highlight_h),
                    Color::new(80, 160, 255, 120), // blue highlight
                );
            }
        }

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
            let available_width = self.bounds.width - self.style.padding * 2.0;
            let text_slice = &self.text[self.scroll_offset..];
            let mut end = text_slice.len();
            let mut last_good = 0;
            for (i, _) in text_slice.char_indices() {
                let candidate = &text_slice[..i];
                let cstr = CString::new(candidate).unwrap_or_default();
                let w = unsafe { raylib::ffi::MeasureText(cstr.as_ptr(), self.style.font_size) } as f32;
                if w > available_width {
                    end = last_good;
                    break;
                }
                last_good = i;
            }
            let visible_text = &text_slice[..end];
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
            let text_slice = &self.text[self.scroll_offset..];
            let available_width = self.bounds.width - self.style.padding * 2.0;
            let mut end = text_slice.len();
            let mut last_good = 0;
            let mut cursor_x = 0.0;
            let mut count = 0;
            for (i, _) in text_slice.char_indices() {
                let candidate = &text_slice[..i];
                let cstr = CString::new(candidate).unwrap_or_default();
                let w = unsafe { raylib::ffi::MeasureText(cstr.as_ptr(), self.style.font_size) } as f32;
                if w > available_width {
                    end = last_good;
                    break;
                }
                last_good = i;
                if self.cursor_position == self.scroll_offset + count {
                    cursor_x = w;
                }
                count += 1;
            }
            // If cursor is at the end
            if self.cursor_position == self.scroll_offset + count {
                let cstr = CString::new(&text_slice[..end]).unwrap_or_default();
                cursor_x = unsafe { raylib::ffi::MeasureText(cstr.as_ptr(), self.style.font_size) } as f32;
            }
            d.draw_line(
                (self.bounds.x + self.style.padding + cursor_x) as i32,
                text_y,
                (self.bounds.x + self.style.padding + cursor_x) as i32,
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
        self.update(rl.get_mouse_position(), rl);
    }

    fn process_character(&self, c: char) -> char {
        if let Some(ref pattern) = self.allowed_pattern {
            if !pattern.is_match(&c.to_string()) {
                return c; // Character not allowed, return original
            }
        }
        if let Some(ref callback) = self.character_callback {
            return callback(c);
        }
        c // Return original character if no processing is needed
    }
}