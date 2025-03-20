use raylib::prelude::*;

pub struct Dropdown {
    pub bounds: Rectangle,
    items: Vec<String>,
    selected_index: Option<usize>,
    is_open: bool,
    background_color: Color,
    border_color: Color,
    text_color: Color,
    hover_color: Color,
    font_size: i32,
    hover_index: Option<usize>,
    max_visible_items: usize,
    scroll_offset: usize,
}

impl Dropdown {
    pub fn new(x: f32, y: f32, width: f32, height: f32, items: Vec<String>) -> Self {
        Self {
            bounds: Rectangle::new(x, y, width, height),
            items,
            selected_index: None,
            is_open: false,
            background_color: Color::WHITE,
            border_color: Color::BLACK,
            text_color: Color::BLACK,
            hover_color: Color::LIGHTGRAY,
            font_size: 20,
            hover_index: None,
            max_visible_items: 5,
            scroll_offset: 0,
        }
    }

    pub fn set_colors(&mut self, background: Color, border: Color, text: Color, hover: Color) {
        self.background_color = background;
        self.border_color = border;
        self.text_color = text;
        self.hover_color = hover;
    }

    pub fn set_font_size(&mut self, size: i32) {
        self.font_size = size;
    }

    pub fn set_max_visible_items(&mut self, count: usize) {
        self.max_visible_items = count;
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

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        // Draw main dropdown box
        d.draw_rectangle_rec(self.bounds, self.background_color);
        d.draw_rectangle_lines_ex(self.bounds, 2.0, self.border_color);

        // Draw selected item or placeholder
        if let Some(selected) = self.selected_index {
            if selected < self.items.len() {
                d.draw_text(
                    &self.items[selected],
                    self.bounds.x as i32 + 5,
                    (self.bounds.y + (self.bounds.height - self.font_size as f32) / 2.0) as i32,
                    self.font_size,
                    self.text_color,
                );
            }
        }

        // Draw dropdown arrow
        let arrow_size = self.font_size as f32 * 0.5;
        let arrow_x = self.bounds.x + self.bounds.width - arrow_size - 5.0;
        let arrow_y = self.bounds.y + (self.bounds.height - arrow_size) / 2.0;
        
        d.draw_triangle(
            Vector2::new(arrow_x, arrow_y),
            Vector2::new(arrow_x + arrow_size, arrow_y),
            Vector2::new(arrow_x + arrow_size / 2.0, arrow_y + arrow_size),
            self.text_color,
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
                    self.hover_color
                } else {
                    self.background_color
                };

                d.draw_rectangle_rec(item_bounds, background_color);
                d.draw_rectangle_lines_ex(item_bounds, 2.0, self.border_color);
                d.draw_text(
                    &self.items[item_index],
                    item_bounds.x as i32 + 5,
                    (item_bounds.y + (item_bounds.height - self.font_size as f32) / 2.0) as i32,
                    self.font_size,
                    self.text_color,
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

    fn draw_scroll_indicator(&self, d: &mut RaylibDrawHandle, is_up: bool) {
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
            self.text_color,
        );
    }

    pub fn get_selected_index(&self) -> Option<usize> {
        self.selected_index
    }

    pub fn get_selected_item(&self) -> Option<&String> {
        self.selected_index.map(|i| &self.items[i])
    }

    pub fn set_selected_index(&mut self, index: Option<usize>) {
        self.selected_index = index;
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn get_hover_index(&self) -> Option<usize> {
        self.hover_index
    }

    pub fn get_hover_item(&self) -> Option<&String> {
        self.hover_index.map(|i| &self.items[i])
    }

    pub fn set_items(&mut self, items: Vec<String>) {
        self.items = items;
        self.selected_index = None;
        self.scroll_offset = 0;
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

    pub fn get_items(&self) -> &Vec<String> {
        &self.items
    }

    pub fn get_item_count(&self) -> usize {
        self.items.len()
    }

    pub fn get_max_visible_items(&self) -> usize {
        self.max_visible_items
    }

    pub fn get_scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    pub fn set_scroll_offset(&mut self, offset: usize) {
        self.scroll_offset = offset.min(self.items.len() - self.max_visible_items);
    }

    pub fn get_bounds(&self) -> Rectangle {
        self.bounds
    }

    pub fn set_bounds(&mut self, bounds: Rectangle) {
        self.bounds = bounds;
    }

    pub fn get_background_color(&self) -> Color {
        self.background_color
    }

    pub fn get_border_color(&self) -> Color {
        self.border_color
    }

    pub fn get_text_color(&self) -> Color {
        self.text_color
    }

    pub fn get_hover_color(&self) -> Color {
        self.hover_color
    }

    pub fn get_font_size(&self) -> i32 {
        self.font_size
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_border_color(&mut self, color: Color) {
        self.border_color = color;
    }

    pub fn set_text_color(&mut self, color: Color) {
        self.text_color = color;
    }

    pub fn set_hover_color(&mut self, color: Color) {
        self.hover_color = color;
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.items.len() >= self.max_visible_items
    }

    pub fn is_enabled(&self) -> bool {
        !self.items.is_empty()
    }

    pub fn is_disabled(&self) -> bool {
        self.items.is_empty()
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.selected_index = None;
        self.scroll_offset = 0;
    }

    pub fn reset(&mut self) {
        self.selected_index = None;
        self.scroll_offset = 0;
        self.is_open = false;
    }

    pub fn is_opened(&self) -> bool {
        self.is_open
    }

    pub fn is_closed(&self) -> bool {
        !self.is_open
    }

    pub fn is_hovered(&self) -> bool {
        self.hover_index.is_some()
    }

    pub fn is_hovered_item(&self) -> bool {
        self.hover_index.is_some() && self.is_open
    }

    pub fn is_selected(&self) -> bool {
        self.selected_index.is_some()
    }

    pub fn is_selected_item(&self) -> bool {
        self.selected_index.is_some() && !self.is_open
    }

    pub fn is_scrollable(&self) -> bool {
        self.items.len() > self.max_visible_items
    }

    pub fn is_scrolled(&self) -> bool {
        self.scroll_offset > 0
    }
}
