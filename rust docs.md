Raylib Interactive Elements Documentation
=========================================

This document provides an overview of the interactive elements implemented in Rust using the Raylib library (via the raylib-rs crate). These components can be integrated into graphical applications or games for interactive UI functionality.

Example addition:

Cargo.toml:
[dependencies]
raylib_interactive = { path = "/usr/lib/libraylib_interactive.rlib" }

main.rs:
use raylib_interactive::{checkbox::Checkbox, button::Button};

1. **Checkbox**
-----------------
### Description:
A simple checkbox that toggles between checked and unchecked states when clicked.

### Public Methods:
- `pub fn new(x: f32, y: f32, size: f32, label: &str) -> Self`
  - Constructor to initialize the checkbox.
  - Parameters:
    - `x`, `y`: Position of the checkbox.
    - `size`: Size of the checkbox.
    - `label`: Text label displayed next to the checkbox.

- `pub fn set_colors(&mut self, box_color: Color, check_color: Color, border_color: Color, label_color: Color)`
  - Set the colors for the checkbox and label.

- `pub fn set_font_size(&mut self, size: i32)`
  - Set the font size for the label.

- `pub fn update(&mut self, rl: &RaylibHandle)`
  - Handles input and toggles the state when clicked.

- `pub fn draw(&self, d: &mut RaylibDrawHandle)`
  - Renders the checkbox and its label.

- `pub fn is_checked(&self) -> bool`
  - Returns the current state of the checkbox.

2. **Dropdown**
-----------------
### Description:
A dropdown menu that allows users to select one item from a list of predefined options.

### Public Methods:
- `pub fn new(x: f32, y: f32, width: f32, height: f32, items: Vec<String>) -> Self`
  - Constructor to initialize the dropdown menu.
  - Parameters:
    - `x`, `y`: Position of the dropdown.
    - `width`, `height`: Dimensions of the dropdown.
    - `items`: List of options available in the dropdown.

- `pub fn set_colors(&mut self, box_color: Color, border_color: Color, text_color: Color, hover_color: Color)`
  - Set the colors for the dropdown box, text, and hover state.

- `pub fn set_font_size(&mut self, font_size: i32)`
  - Set the font size for the dropdown items.

- `pub fn update(&mut self, rl: &RaylibHandle)`
  - Handles input, toggles visibility, and selects items when clicked.

- `pub fn draw(&self, d: &mut RaylibDrawHandle)`
  - Renders the dropdown and its items (if open).

- `pub fn get_selected_index(&self) -> Option<usize>`
  - Returns the index of the currently selected item.

- `pub fn get_selected_item(&self) -> Option<&String>`
  - Returns the currently selected item as a string.

3. **TextField**
-----------------
### Description:
A text field for user input with a customizable maximum character limit.

### Public Methods:
- `pub fn new(x: f32, y: f32, width: f32, height: f32, max_length: usize) -> Self`
  - Constructor to initialize the text field.
  - Parameters:
    - `x`, `y`: Position of the text field.
    - `width`, `height`: Dimensions of the text field.
    - `max_length`: Maximum number of characters allowed.

- `pub fn set_colors(&mut self, background_color: Color, border_color: Color, text_color: Color)`
  - Set the colors for the text field.

- `pub fn set_font_size(&mut self, font_size: i32)`
  - Set the font size for the text.

- `pub fn update(&mut self, rl: &RaylibHandle)`
  - Handles user input and updates the text.

- `pub fn draw(&self, d: &mut RaylibDrawHandle)`
  - Renders the text field and its current contents.

- `pub fn get_text(&self) -> &str`
  - Returns the current text in the field.

- `pub fn get_last_key(&self) -> Option<i32>`
  - Returns the last key pressed by the user.

- `pub fn is_active(&self) -> bool`
  - Returns whether the text field is active.

- `pub fn activate(&mut self)`
  - Activates the text field for input.

- `pub fn deactivate(&mut self)`
  - Deactivates the text field.

4. **Button**
-----------------
### Description:
A clickable button with a customizable label, size, and colors.

### Public Methods:
- `pub fn new(x: f32, y: f32, width: f32, height: f32, label: &str) -> Self`
  - Constructor to initialize the button.
  - Parameters:
    - `x`, `y`: Position of the button.
    - `width`, `height`: Dimensions of the button.
    - `label`: Text displayed on the button.

- `pub fn set_colors(&mut self, background_color: Color, border_color: Color, text_color: Color)`
  - Set the colors for the button and its label.

- `pub fn set_font_size(&mut self, font_size: i32)`
  - Set the font size for the button label.

- `pub fn update(&mut self, rl: &RaylibHandle)`
  - Detects if the button is clicked.

- `pub fn draw(&self, d: &mut RaylibDrawHandle)`
  - Renders the button and its label.

- `pub fn is_clicked(&self, rl: &RaylibHandle) -> bool`
  - Returns `true` if the button was clicked.

### Usage Example:
1. Initialize Raylib:
   ```rust
   let (mut rl, thread) = raylib::init().size(800, 600).title("Raylib Interactive Elements").build();
   ```
2. Create interactive elements:
   ```rust
   let mut checkbox = Checkbox::new(100.0, 100.0, 20.0, "Enable Option");
   let mut dropdown = Dropdown::new(200.0, 100.0, 150.0, 30.0, vec!["Option 1".to_string(), "Option 2".to_string(), "Option 3".to_string()]);
   let mut text_field = TextField::new(100.0, 200.0, 200.0, 30.0, 32);
   let mut button = Button::new(300.0, 300.0, 100.0, 40.0, "Click Me");
   ```
3. Update and draw in the game loop:
   ```rust
   while !rl.window_should_close() {
       let mut d = rl.begin_drawing(&thread);
       d.clear_background(Color::RAYWHITE);

       checkbox.update(&rl);
       dropdown.update(&rl);
       text_field.update(&rl);
       button.update(&rl);

       checkbox.draw(&mut d);
       dropdown.draw(&mut d);
       text_field.draw(&mut d);
       button.draw(&mut d);
   }
   ```

