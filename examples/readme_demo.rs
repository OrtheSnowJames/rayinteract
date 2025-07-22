use raylib::prelude::*;
use raylib_interactive::{presets, Button, Checkbox, Dropdown, Style, TextField};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Raylib Interactive Demo")
        .build();

    // Create components with different styles
    let mut button =
        Button::new(350.0, 200.0, 100.0, 40.0, "Click Me!").with_style(presets::button_primary());

    let mut checkbox =
        Checkbox::new(350.0, 300.0, 20.0, "Enable Spamming To Stdout").with_style(presets::checkbox_default());

    let mut text_field =
        TextField::new(300.0, 400.0, 200.0, 30.0, 50).with_style(presets::textfield_default());
    text_field.placeholder = "Enter text...".to_string();

    let mut dropdown = Dropdown::new(
        300.0,
        450.0,
        200.0,
        30.0,
        vec![
            "Option 1".to_string(),
            "Option 2".to_string(),
            "Option 3".to_string(),
        ],
    )
    .with_deselect_option("None")
    .with_style(presets::dropdown_default());

    // Create custom style
    let custom_style = Style::new(
        Color::new(255, 200, 100, 255), // Orange background
        Color::new(255, 220, 120, 255), // Light orange hover
        Color::new(200, 150, 50, 255),  // Dark orange pressed
        Color::new(150, 100, 0, 255),   // Dark orange border
        Color::new(50, 25, 0, 255),     // Dark brown text
    )
    .with_typography(18)
    .with_layout(10.0, 8.0, 3.0);

    let mut custom_button =
        Button::new(350.0, 10.0, 100.0, 40.0, "Custom").with_style(custom_style);

    while !rl.window_should_close() {
        // Get mouse position for UI interaction
        let mouse = rl.get_mouse_position();

        // Update all components
        raylib_interactive::update_all!(
            &mut rl,
            mouse,
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

        if custom_button.is_clicked(&rl) {
            println!("Custom button clicked!");
        }

        if checkbox.is_checked {
            println!("Checkbox checked!");
        }

        if let Some(selected) = dropdown.get_selected_item() {
            println!("Selected: {}", selected);
        } 

        // Direct field access examples

        button.label = "Updated!".to_string();
        //text_field.text = "Hello World".to_string();
        //checkbox.is_checked = true;
        //dropdown.is_open = true;

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
