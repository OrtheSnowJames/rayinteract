# Raylib Interactive
## The DIY GUI Toolkit for Raylib 

This is a library that supports rust and adds many interactive things to your current raylib project, such as:

- Buttons
- Checkboxes
- Dropdowns
- Textfields

Raylib Interactive is a library built on raylib that adds many components to raylib,
allowing you to make better graphical interfaces. The latest version:

- Added Style system with predefined themes and simplified API with public fields. (Not compatible with older versions so much)
- Added update_all and draw_all macros for clarity and less boilerplate.
- Textfield scrolling and cursor for proportional fonts
- Dropdowns support a deselect/clear option and handle selection offset correctly
- Clicking outside a dropdown or textfield will close/deselect it (standard UI behavior)
- Macro-based batch update/draw for all UI elements (no 50 lines of .update and .draw)
- API and UI behavior are cleaner and more intuitive

This project is hosted on github at the [GitHub homepage](https://github.com/OrtheSnowJames/rayinteract).

## Docs

## Overview

### Style System
The library now has a `Style` struct that provides theming across all components:

```rust
pub struct Style {
    // Background colors
    pub background_color: Color,
    pub hover_color: Color,
    pub pressed_color: Color,
    pub active_color: Color,
    pub disabled_color: Color,
    
    // Border colors
    pub border_color: Color,
    pub border_color_hover: Color,
    pub border_color_pressed: Color,
    pub border_color_active: Color,
    
    // Text colors
    pub text_color: Color,
    pub text_color_hover: Color,
    pub text_color_pressed: Color,
    pub text_color_disabled: Color,
    
    // Special colors for specific components
    pub check_color: Color,        // For checkboxes
    pub placeholder_color: Color,  // For text fields
    
    // Typography
    pub font_size: i32,
    
    // Layout
    pub padding: f32,
    pub corner_radius: f32,
    pub border_thickness: f32,
}
```

- `Style::default()` - Default gray theme
- `Style::modern_blue()` - Modern blue accent theme
- `Style::dark_theme()` - Dark theme with blue accents
- `Style::minimal()` - Clean minimal theme
- `Style::new()` - Create custom styles with builder pattern

### Button
```rust
pub struct Button {
    pub bounds: Rectangle,           // Position and size
    pub label: String,               // Button text
    pub style: Style,                // Visual styling
    pub is_hovered: bool,            // Mouse hover state
    pub is_pressed: bool,            // Mouse press state
    pub animation_progress: f32,     // Animation state (0.0-1.0)
    pub enabled: bool,               // Whether button is interactive
}
```

**Methods:**
- `new(x, y, width, height, label) -> Self` // Constructor
- `with_style(style) -> Self` // Apply style
- `set_colors(background, hover, pressed, border, text)` // Quick color setup
- `update(rl)` // Handle input and animations
- `draw(d)` // Render the button
- `is_clicked(rl) -> bool` // Check if clicked this frame

### Checkbox
```rust
pub struct Checkbox {
    pub bounds: Rectangle,           // Position and size
    pub is_checked: bool,            // Checked state
    pub style: Style,                // Visual styling
    pub label: String,               // Label text
    pub is_hovered: bool,            // Mouse hover state
    pub animation_progress: f32,     // Check animation (0.0-1.0)
    pub is_clicked: bool,            // Click state
}
```

**Methods:**
- `new(x, y, size, label) -> Self` // Constructor
- `with_style(style) -> Self` // Apply style
- `set_colors(background, check, border, hover, label)` // Quick color setup
- `update(rl)` // Handle input and animations
- `draw(d)` // Render the checkbox
- `toggle()` // Toggle checked state

### TextField
```rust
pub struct TextField {
    pub bounds: Rectangle,           // Position and size
    pub text: String,                // Current text content
    pub placeholder: String,         // Placeholder text
    pub max_length: usize,           // Maximum characters
    pub style: Style,                // Visual styling
    pub is_active: bool,             // Has focus
    pub cursor_position: usize,      // Text cursor position
    pub cursor_blink_timer: f32,     // Cursor blink animation
    pub backspace_hold_timer: f32,   // Backspace repeat timing
    pub arrow_hold_timer: f32,       // Arrow key repeat timing
    pub scroll_offset: usize,        // Horizontal scroll offset
    pub is_scrolling: bool,          // Scroll bar drag state
    pub allowed_pattern: Option<Regex>, // Regex for allowed characters
    pub character_callback: Option<Box<dyn Fn(char) -> char>>, // Character transform callback
}
```

**Methods:**
- `new(x, y, width, height, max_length) -> Self` // Constructor
- `with_style(style) -> Self` // Apply style
- `set_colors(background, border, text)` // Quick color setup
- `update(rl)` // Handle input and focus
- `draw(d)` // Render the text field
- `clear()` // Clear all text
- `activate()` // Give focus
- `deactivate()` // Remove focus
- `handle_input(rl)` // Process keyboard input
- `only_allow(regex: Regex) -> Self` // Only allow characters matching regex
- `change_character(callback: Fn(char) -> char) -> Self` // Transform each character as typed

**Special:**
- Use `.only_allow()` to restrict allowed characters (e.g., no spaces).
- Use `.change_character()` to mask or transform input (e.g., password fields).

**Password Field Example:**
```rust
use regex::Regex;
let mut password_field = TextField::new(0.0, 0.0, 200.0, 30.0, 20)
    .only_allow(Regex::new("^[^ ]$").unwrap()) // Disallow spaces
    .change_character(|_| '*'); // Mask all input as '*'
```

### Dropdown
```rust
pub struct Dropdown {
    pub bounds: Rectangle,           // Position and size
    pub items: Vec<String>,          // Available options
    pub selected_index: Option<usize>, // Currently selected item
    pub is_open: bool,               // Dropdown expanded state
    pub style: Style,                // Visual styling
    pub hover_index: Option<usize>,  // Hovered item index
    pub max_visible_items: usize,    // Max items shown when open
    pub scroll_offset: usize,        // Scroll position for long lists
}
```

**Methods:**
- `new(x, y, width, height, items) -> Self` // Constructor
- `with_style(style) -> Self` // Apply style
- `with_deselect_option(label) -> Self` // Add a deselect/clear option as the first item
- `set_colors(background, border, text, hover)` // Quick color setup
- `update(rl)` // Handle input and selection
- `draw(d)` // Render the dropdown
- `get_selected_item() -> Option<&String>` // Get selected text
- `add_item(item)` // Add new option
- `remove_item(index)` // Remove option
- `clear_items()` // Remove all options
- `clear_selection()` // Clear selected
- `open()` // Expand dropdown
- `close()` // Collapse dropdown
- `toggle()` // Toggle open/closed

**Special:**
- Selecting the first item (deselect option) or pressing `Esc` while open will clear the selection.

### Style Presets
The library provides convenient style presets for common use cases:

#### Button Presets
- `presets::button_default()` - Default button style
- `presets::button_primary()` - Primary action button (blue)
- `presets::button_secondary()` - Secondary action button (gray)
- `presets::button_success()` - Success action button (green)
- `presets::button_danger()` - Danger action button (red)

#### Component Presets
- `presets::textfield_default()` - Default text field style
- `presets::checkbox_default()` - Default checkbox style
- `presets::dropdown_default()` - Default dropdown style

### Style Builder Methods
The `Style` struct provides builder methods for easy customization:

- `with_background_colors(background, hover, pressed)` // Set background colors
- `with_border_colors(border, hover, pressed)` // Set border colors
- `with_text_colors(text, hover, pressed)` // Set text colors
- `with_typography(font_size)` // Set font size
- `with_layout(padding, corner_radius, border_thickness)` // Set layout properties

### Batch Update/Draw Macros
You can update and draw multiple UI elements at once using the provided macros:

```rust
raylib_interactive::update_all!(
    virtual_mouse, // The mouse position in virtual (UI) coordinates, useful for drawing to off-screen render targets
    &mut rl,       // The Raylib handle
    button,
    checkbox,
    text_field,
    dropdown,
    custom_button,
);

raylib_interactive::draw_all!(
    &mut d,
    button,
    checkbox,
    text_field,
    dropdown,
    custom_button,
);
```

**If you are rendering to a virtual resolution (e.g., with a render texture for scaling/aspect ratio):**

```rust
let mouse = rl.get_mouse_position();
let virtual_mouse = Vector2 {
    x: ((mouse.x - offset_x as f32) / scale).clamp(0.0, VIRTUAL_WIDTH as f32),
    y: ((mouse.y - offset_y as f32) / scale).clamp(0.0, VIRTUAL_HEIGHT as f32),
};
raylib_interactive::update_all!(
    virtual_mouse,
    &mut rl,
    button,
    checkbox,
    // ...
);
```

- The first argument is the virtual mouse position (for correct UI hit testing when using render textures or scaling).
- The second argument is the Raylib handle.
- Then list all your UI elements.

### UI/UX Behavior
- **Clicking outside a dropdown or textfield will close/deselect it.** This matches standard UI expectations and is built-in to the components.
- **Dropdowns with a deselect option:** If you use `.with_deselect_option("None")`, the first item is a clear/deselect option. The selected index and `get_selected_item()` are offset accordingly, so you always get the correct item or `None` if deselected.
- **Theme toggling:** The demo shows how to toggle between dark and light themes, and applies the correct style preset to each button (primary, secondary, success, danger, etc.) so colors always match the intended theme.
- **Enabling/disabling buttons:** The demo shows how a checkbox can enable or disable a button, and how you can change the style and label dynamically.

## Usage Example
```rust
use raylib::prelude::*;
use raylib_interactive::{Button, TextField, Checkbox, Dropdown, Style, presets};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Raylib Interactive Demo")
        .build();

    // Create components with different styles
    let mut button = Button::new(350.0, 200.0, 100.0, 40.0, "Click Me!")
        .with_style(presets::button_primary());
    
    let mut checkbox = Checkbox::new(350.0, 300.0, 20.0, "Enable Feature")
        .with_style(presets::checkbox_default());
    
    let mut text_field = TextField::new(300.0, 400.0, 200.0, 30.0, 50)
        .with_style(presets::textfield_default());
    text_field.placeholder = "Enter text...".to_string();

    let mut dropdown = Dropdown::new(300.0, 450.0, 200.0, 30.0, 
        vec!["Option 1".to_string(), "Option 2".to_string(), "Option 3".to_string()])
        .with_deselect_option("None")
        .with_style(presets::dropdown_default());

    // Create custom style
    let custom_style = Style::new(
        Color::new(255, 200, 100, 255),  // Orange background
        Color::new(255, 220, 120, 255),  // Light orange hover
        Color::new(200, 150, 50, 255),   // Dark orange pressed
        Color::new(150, 100, 0, 255),    // Dark orange border
        Color::new(50, 25, 0, 255),      // Dark brown text
    ).with_typography(18)
     .with_layout(10.0, 8.0, 3.0);

    let mut custom_button = Button::new(350.0, 500.0, 100.0, 40.0, "Custom")
        .with_style(custom_style);

    while !rl.window_should_close() {
        // Get mouse position for UI interaction
        let mouse = rl.get_mouse_position();
        
        // Update all components
        raylib_interactive::update_all!(
            mouse,
            &mut rl,
            button,
            checkbox,
            text_field,
            dropdown,
            custom_button,
        );

        // Handle interactions
        if button.is_clicked(&rl) {
            println!("Button clicked!");
        }

        if checkbox.is_checked {
            println!("Checkbox checked!");
        }

        if let Some(selected) = dropdown.get_selected_item() {
            println!("Selected: {}", selected);
        } else {
            println!("Dropdown selection cleared");
        }

        // Direct field access examples
        button.label = "Updated!".to_string();
        text_field.text = "Hello World".to_string();
        checkbox.is_checked = true;
        dropdown.is_open = true;

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        // Draw all components
        raylib_interactive::draw_all!(
            &mut d,
            button,
            checkbox,
            text_field,
            dropdown,
            custom_button,
        );
    }
}
```