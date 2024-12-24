# Raylib Interactive
## The library for people who want to do other things but stick to raylib

This is a library that supports c++ and rust and adds many interactive things to your current raylib project, such as:

Buttons,
Checkboxes,
Dropdowns, and
Textfields.

A copy of "rust docs.md" is pasted below:

# Raylib Interactive Documentation

Raylib Interactive is a library built on raylib that adds many components to raylib-rs,
allowing you to make better graphical interfaces. The latest version:
Added backspace hold functionality to TextField.
For more information, visit my [GitHub homepage](https://github.com/OrtheSnowJames/rayinteract).

This library adds Buttons, Checkboxes, Textfields, and Dropdowns.
Hope you enjoy! -James

## Overview
### Button Methods
- `new(x: f32, y: f32, width: f32, height: f32, text: &str) -> Self` // Creates a new button with specified position, dimensions and text
- `with_style(mut self, style: ButtonStyle) -> Self` // Applies a custom style to the button
- `set_enabled(&mut self, enabled: bool)` // Enables or disables the button interactivity
- `is_enabled(&self) -> bool` // Returns whether the button is currently enabled
- `set_position(&mut self, x: f32, y: f32)` // Updates the button's position
- `set_size(&mut self, width: f32, height: f32)` // Updates the button's dimensions
- `set_text(&mut self, text: &str)` // Changes the button's text
- `get_text(&self) -> &str` // Returns the current button text
- `get_position(&self) -> Vector2` // Returns the current position
- `get_size(&self) -> Vector2` // Returns the current dimensions
- `is_clicked(&self, rl: &RaylibHandle) -> bool` // Checks if button was clicked this frame
- `is_hovered(&self) -> bool` // Checks if mouse is hovering over button
- `update(&mut self, rl: &RaylibHandle)` // Updates button state
- `draw(&self, d: &mut RaylibDrawHandle)` // Renders the button

### Checkbox Methods
- `new(x: f32, y: f32, size: f32, label: &str) -> Self` // Creates a new checkbox with position, size and label
- `set_checked(&mut self, checked: bool)` // Sets the checked state
- `is_checked(&self) -> bool` // Returns current checked state
- `toggle(&mut self)` // Toggles between checked/unchecked
- `set_position(&mut self, x: f32, y: f32)` // Updates checkbox position
- `set_size(&mut self, size: f32)` // Updates checkbox size
- `set_label(&mut self, label: &str)` // Changes the label text
- `get_label(&self) -> &str` // Returns current label text
- `with_animation(mut self, animation: CheckboxAnimation) -> Self` // Applies custom animation
- `update(&mut self, rl: &RaylibHandle)` // Updates checkbox state
- `draw(&self, d: &mut RaylibDrawHandle)` // Renders the checkbox

### TextField Methods
- `new(x: f32, y: f32, width: f32, height: f32, max_length: usize) -> Self` // Creates new text field with position, size and max length
- `set_text(&mut self, text: &str)` // Sets the field's text content
- `get_text(&self) -> &str` // Returns current text content
- `set_placeholder(&mut self, placeholder: &str)` // Sets placeholder text shown when empty
- `get_placeholder(&self) -> &str` // Returns current placeholder text
- `set_mask_char(&mut self, mask: Option<char>)` // Sets masking character for password fields
- `set_max_length(&mut self, max_length: usize)` // Updates maximum text length
- `is_focused(&self) -> bool` // Returns whether field has input focus
- `set_position(&mut self, x: f32, y: f32)` // Updates field position
- `set_size(&mut self, width: f32, height: f32)` // Updates field dimensions
- `clear(&mut self)` // Clears all text content
- `handle_input(&mut self, rl: &RaylibHandle)` // Processes keyboard input
- `update(&mut self, rl: &RaylibHandle)` // Updates field state
- `draw(&self, d: &mut RaylibDrawHandle)` // Renders the text field

### Dropdown Methods
- `new(x: f32, y: f32, width: f32, height: f32, items: Vec<String>) -> Self` // Creates dropdown with position, size and items
- `set_items(&mut self, items: Vec<String>)` // Updates the list of items
- `get_items(&self) -> &[String]` // Returns current item list
- `set_selected_index(&mut self, index: Option<usize>)` // Sets currently selected item
- `get_selected_index(&self) -> Option<usize>` // Returns index of selected item
- `get_selected_item(&self) -> Option<&String>` // Returns currently selected item
- `enable_search(&mut self, enabled: bool)` // Enables/disables search functionality
- `is_search_enabled(&self) -> bool` // Returns whether search is enabled
- `set_max_height(&mut self, height: f32)` // Sets maximum height when opened
- `set_position(&mut self, x: f32, y: f32)` // Updates dropdown position
- `set_size(&mut self, width: f32, height: f32)` // Updates dropdown dimensions
- `is_opened(&self) -> bool` // Returns whether dropdown is expanded
- `update(&mut self, rl: &RaylibHandle)` // Updates dropdown state
- `draw(&self, d: &mut RaylibDrawHandle)` // Renders the dropdown

### Common Traits
- `Drawable::draw(&self, d: &mut RaylibDrawHandle)` // Renders a UI component
- `Interactive::update(&mut self, rl: &RaylibHandle)` // Updates component state
- `Interactive::handle_input(&mut self, rl: &RaylibHandle)` // Processes user input
- `Styleable::with_theme(self, theme: Theme) -> Self` // Applies a theme to component
- `Styleable::override_style(self, style: Style) -> Self` // Overrides default styling

### Event System
- `EventEmitter::on(&mut self, event: Event, callback: Box<dyn Fn(&T)>)` // Registers event callback
- `EventEmitter::emit(&self, event: Event, data: &T)` // Triggers registered event callbacks

### Layout System
- `Layout::horizontal()` // Creates horizontal layout container
- `Layout::vertical()` // Creates vertical layout container
- `Layout::grid(rows: u32, cols: u32)` // Creates grid layout container
- `Layout::add<T: Widget>(&mut self, widget: T)` // Adds widget to layout
- `Layout::remove(&mut self, index: usize)` // Removes widget at index
- `Layout::clear(&mut self)` // Removes all widgets
## Introduction
Raylib Interactive is a high-level UI library built on top of Raylib, providing an intuitive and flexible interface for creating interactive graphical user interfaces in Rust. It offers a comprehensive set of widgets, event handling, and layout management tools while maintaining Raylib's simplicity and performance.

### Key Features
- Ready-to-use UI components (buttons, checkboxes, text fields, dropdowns)
- Event-driven architecture with custom callbacks
- Flexible layout system for complex UI arrangements
- Customizable themes and styling
- Automatic input handling and state management
- Seamless integration with existing Raylib applications

## Usage Example
```rust
use raylib::prelude::*;
use raylib_interactive::*;

fn main() {
  let (mut rl, thread) = raylib::init()
    .size(800, 600)
    .title("Raylib Interactive Demo")
    .build();

  let mut button = Button::new(350.0, 200.0, 100.0, 40.0, "Click Me!")
    .with_style(ButtonStyle::default());
  
  let mut checkbox = Checkbox::new(350.0, 300.0, 20.0, "Enable Feature");
  
  let mut text_field = TextField::new(300.0, 400.0, 200.0, 30.0, 50);

  while !rl.window_should_close() {
    button.update(&rl);
    checkbox.update(&rl);
    text_field.update(&rl);

    if button.is_clicked(&rl) {
      println!("Button clicked!");
    }

    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::WHITE);

    button.draw(&mut d);
    checkbox.draw(&mut d);
    text_field.draw(&mut d);
  }
}
```