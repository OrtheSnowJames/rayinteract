# Raylib Interactive
## The library for people who want to do other things but stick to raylib

This is a library that supports c++ and rust and adds many interactive things to your current raylib project, such as:

Buttons,
Checkboxes,
Dropdowns, and
Textfields.

A copy of "rust docs.md" is pasted below:

Raylib Interactive Elements Documentation
=========================================

This document provides an overview of the interactive elements implemented in Rust using the Raylib library. These components provide a rich set of interactive UI functionality for graphical applications or games.

1. **Checkbox**
-----------------
### Description:
An animated checkbox that toggles between checked and unchecked states with visual feedback.

### Public Methods:
- `pub fn new(x: f32, y: f32, size: f32, label: &str) -> Self`
  - Constructor to initialize the checkbox.
  - Parameters:
    - `x`, `y`: Position of the checkbox
    - `size`: Size of the checkbox
    - `label`: Text label displayed next to the checkbox

- `pub fn set_colors(&mut self, background: Color, check: Color, border: Color, hover: Color, label: Color)`
  - Set all colors for the checkbox components.

- `pub fn set_font_size(&mut self, size: i32)`
  - Set the font size for the label.

- `pub fn update(&mut self, rl: &RaylibHandle)`
  - Handles input and animations.

- `pub fn draw(&self, d: &mut RaylibDrawHandle)`
  - Renders the checkbox with animations.

- `pub fn is_checked(&self) -> bool`
  - Returns the current state.

- `pub fn set_checked(&mut self, checked: bool)`
  - Programmatically set the checkbox state.

- `pub fn toggle(&mut self)`
  - Toggle the checkbox state.

2. **Dropdown**
-----------------
### Description:
A scrollable dropdown menu with hover effects and custom styling.

### Public Methods:
- `pub fn new(x: f32, y: f32, width: f32, height: f32, items: Vec<String>) -> Self`
  - Constructor to initialize the dropdown.
  - Parameters:
    - `x`, `y`: Position
    - `width`, `height`: Dimensions
    - `items`: List of options

- `pub fn set_colors(&mut self, background: Color, border: Color, text: Color, hover: Color)`
  - Set the colors for all components.

- `pub fn set_font_size(&mut self, size: i32)`
  - Set the font size.

- `pub fn set_max_visible_items(&mut self, count: usize)`
  - Set maximum number of visible items before scrolling.

- `pub fn update(&mut self, rl: &RaylibHandle)`
  - Handles input, scrolling, and item selection.

- `pub fn draw(&self, d: &mut RaylibDrawHandle)`
  - Renders the dropdown with scroll indicators.

- `pub fn get_selected_index(&self) -> Option<usize>`
  - Returns the selected item's index.

- `pub fn get_selected_item(&self) -> Option<&String>`
  - Returns the selected item.

3. **TextField**
-----------------
### Description:
A text input field with cursor animation and selection support.

### Public Methods:
- `pub fn new(x: f32, y: f32, width: f32, height: f32, max_length: usize) -> Self`
  - Constructor to initialize the text field.
  - Parameters:
    - `x`, `y`: Position
    - `width`, `height`: Dimensions
    - `max_length`: Maximum text length

- `pub fn set_colors(&mut self, background: Color, border: Color, text: Color)`
  - Set the colors.

- `pub fn set_font_size(&mut self, size: i32)`
  - Set the font size.

- `pub fn update(&mut self, rl: &RaylibHandle)`
  - Handles text input and cursor movement.

- `pub fn draw(&self, d: &mut RaylibDrawHandle)`
  - Renders the field with animated cursor.

- `pub fn get_text(&self) -> &str`
  - Returns current text.

- `pub fn is_active(&self) -> bool`
  - Returns focus state.

- `pub fn activate(&mut self)`
  - Set focus to this field.

- `pub fn deactivate(&mut self)`
  - Remove focus.

4. **Button**
-----------------
### Description:
An animated button with hover and press effects, rounded corners, and custom styling.

### Public Methods:
- `pub fn new(x: f32, y: f32, width: f32, height: f32, label: &str) -> Self`
  - Constructor to initialize the button.
  - Parameters:
    - `x`, `y`: Position
    - `width`, `height`: Dimensions
    - `label`: Button text

- `pub fn set_colors(&mut self, background: Color, hover: Color, pressed: Color, border: Color, text: Color)`
  - Set all button colors.

- `pub fn set_font_size(&mut self, size: i32)`
  - Set the font size.

- `pub fn set_corner_radius(&mut self, radius: f32)`
  - Set corner rounding radius.

- `pub fn set_padding(&mut self, padding: f32)`
  - Set internal padding.

- `pub fn set_enabled(&mut self, enabled: bool)`
  - Enable/disable the button.

- `pub fn update(&mut self, rl: &RaylibHandle)`
  - Handles input and animations.

- `pub fn draw(&self, d: &mut RaylibDrawHandle)`
  - Renders the button with effects.

- `pub fn is_clicked(&self, rl: &RaylibHandle) -> bool`
  - Returns true if clicked.

- `pub fn is_pressed(&self) -> bool`
  - Returns true if being pressed.

- `pub fn is_hovered(&self) -> bool`
  - Returns true if mouse is over button.

- `pub fn is_enabled(&self) -> bool`
  - Returns enabled state.

- `pub fn get_label(&self) -> &str`
  - Returns button text.

- `pub fn set_label(&mut self, label: &str)`
  - Set button text.

### Usage Example:
```rust
// Initialize Raylib
let (mut rl, thread) = raylib::init()
    .size(800, 600)
    .title("Raylib Interactive Elements")
    .msaa_4x()
    .vsync()
    .build();

// Create elements
let mut checkbox = Checkbox::new(100.0, 100.0, 24.0, "Enable Option");
let mut dropdown = Dropdown::new(200.0, 100.0, 150.0, 30.0, 
    vec!["Option 1".to_string(), "Option 2".to_string()]);
let mut text_field = TextField::new(100.0, 200.0, 250.0, 35.0, 32);
let mut button = Button::new(300.0, 300.0, 120.0, 40.0, "Submit");

// Customize appearance
button.set_corner_radius(5.0);
dropdown.set_max_visible_items(4);

// Main loop
while !rl.window_should_close() {
    // Update
    checkbox.update(&rl);
    dropdown.update(&rl);
    text_field.update(&rl);
    button.update(&rl);

    // Draw
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::RAYWHITE);

    checkbox.draw(&mut d);
    dropdown.draw(&mut d);
    text_field.draw(&mut d);
    button.draw(&mut d);
}
```

