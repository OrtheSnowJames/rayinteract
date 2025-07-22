use raylib::prelude::*;
use crate::style::Style;

pub struct Dropdown {
    pub bounds: Rectangle,
    pub items: Vec<String>,
    pub selected_index: Option<usize>,
    pub is_open: bool,
    pub style: Style,
    pub hover_index: Option<usize>,
    pub max_visible_items: usize,
    pub scroll_offset: usize,
}

impl Dropdown {
    pub fn new(x: f32, y: f32, width: f32, height: f32, items: Vec<String>) -> Self {
        Self {
            bounds: Rectangle::new(x, y, width, height),
            items,
            selected_index: None,
            is_open: false,
            style: Style::default(),
            hover_index: None,
            max_visible_items: 5,
            scroll_offset: 0,
        }
    }

    /// Create a dropdown from an enum that implements ToString
    pub fn from_enum<T: ToString>(x: f32, y: f32, width: f32, height: f32, options: &[T]) -> Self {
        let items: Vec<String> = options.iter().map(|opt| opt.to_string()).collect();
        Self::new(x, y, width, height, items)
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn set_colors(&mut self, background: Color, border: Color, text: Color, hover: Color) {
        self.style.background_color = background;
        self.style.border_color = border;
        self.style.text_color = text;
        self.style.hover_color = hover;
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        let mouse_pos = rl.get_mouse_position();
        self.hover_index = None;

        // Handle main dropdown box click
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if self.bounds.check_collision_point_rec(mouse_pos) {
                self.is_open = !self.is_open;
            } else if self.is_open {
                // Check clicks on dropdown items
                let visible_items = self.items.len().min(self.max_visible_items);
                for i in 0..visible_items {
                    let item_bounds = self.get_item_bounds(i);
                    if item_bounds.check_collision_point_rec(mouse_pos) {
                        self.selected_index = Some(i + self.scroll_offset);
                        self.is_open = false;
                        break;
                    }
                }
            }
        }

        // Handle scrolling when dropdown is open
        if self.is_open && self.items.len() > self.max_visible_items {
            let wheel_move = rl.get_mouse_wheel_move() as i32;
            if wheel_move != 0 {
                self.scroll_offset = (self.scroll_offset as i32 - wheel_move)
                    .max(0)
                    .min((self.items.len() - self.max_visible_items) as i32) as usize;
            }
        }

        // Update hover state
        if self.is_open {
            let visible_items = self.items.len().min(self.max_visible_items);
            for i in 0..visible_items {
                let item_bounds = self.get_item_bounds(i);
                if item_bounds.check_collision_point_rec(mouse_pos) {
                    self.hover_index = Some(i + self.scroll_offset);
                    break;
                }
            }
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw) {
        // Draw main dropdown box
        d.draw_rectangle_rec(self.bounds, self.style.background_color);
        d.draw_rectangle_lines_ex(self.bounds, self.style.border_thickness, self.style.border_color);

        // Draw selected item or placeholder
        if let Some(selected) = self.selected_index {
            if selected < self.items.len() {
                d.draw_text(
                    &self.items[selected],
                    self.bounds.x as i32 + self.style.padding as i32,
                    (self.bounds.y + (self.bounds.height - self.style.font_size as f32) / 2.0) as i32,
                    self.style.font_size,
                    self.style.text_color,
                );
            }
        }

        // Draw dropdown arrow
        let arrow_size = self.style.font_size as f32 * 0.5;
        let arrow_x = self.bounds.x + self.bounds.width - arrow_size - self.style.padding;
        let arrow_y = self.bounds.y + (self.bounds.height - arrow_size) / 2.0;
        
        d.draw_triangle(
            Vector2::new(arrow_x, arrow_y),
            Vector2::new(arrow_x + arrow_size, arrow_y),
            Vector2::new(arrow_x + arrow_size / 2.0, arrow_y + arrow_size),
            self.style.text_color,
        );

        // Draw dropdown items when open
        if self.is_open {
            let visible_items = self.items.len().min(self.max_visible_items);
            for i in 0..visible_items {
                let item_index = i + self.scroll_offset;
                if item_index >= self.items.len() {
                    break;
                }

                let item_bounds = self.get_item_bounds(i);
                let background_color = if Some(item_index) == self.hover_index {
                    self.style.hover_color
                } else {
                    self.style.background_color
                };

                d.draw_rectangle_rec(item_bounds, background_color);
                d.draw_rectangle_lines_ex(item_bounds, self.style.border_thickness, self.style.border_color);
                d.draw_text(
                    &self.items[item_index],
                    item_bounds.x as i32 + self.style.padding as i32,
                    (item_bounds.y + (item_bounds.height - self.style.font_size as f32) / 2.0) as i32,
                    self.style.font_size,
                    self.style.text_color,
                );
            }

            // Draw scroll indicators if needed
            if self.items.len() > self.max_visible_items {
                if self.scroll_offset > 0 {
                    self.draw_scroll_indicator(d, true);
                }
                if self.scroll_offset < self.items.len() - self.max_visible_items {
                    self.draw_scroll_indicator(d, false);
                }
            }
        }
    }

    fn get_item_bounds(&self, index: usize) -> Rectangle {
        Rectangle::new(
            self.bounds.x,
            self.bounds.y + self.bounds.height * (index as f32 + 1.0),
            self.bounds.width,
            self.bounds.height,
        )
    }

    fn draw_scroll_indicator(&self, d: &mut impl RaylibDraw, is_up: bool) {
        let x = self.bounds.x + self.bounds.width - 15.0;
        let y = if is_up {
            self.bounds.y + self.bounds.height
        } else {
            self.bounds.y + self.bounds.height * (self.max_visible_items as f32 + 1.0)
        };

        d.draw_triangle(
            Vector2::new(x, y + (if is_up { 10.0 } else { 0.0 })),
            Vector2::new(x + 10.0, y + (if is_up { 10.0 } else { 0.0 })),
            Vector2::new(x + 5.0, y + (if is_up { 0.0 } else { 10.0 })),
            self.style.text_color,
        );
    }

    pub fn get_selected_item(&self) -> Option<&String> {
        self.selected_index.map(|i| &self.items[i])
    }

    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.remove(index);
            if let Some(selected) = self.selected_index {
                if selected == index {
                    self.selected_index = None;
                } else if selected > index {
                    self.selected_index = Some(selected - 1);
                }
            }
        }
    }

    pub fn clear_items(&mut self) {
        self.items.clear();
        self.selected_index = None;
        self.scroll_offset = 0;
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }
}
