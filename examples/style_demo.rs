use raylib::prelude::*;
use raylib_interactive::{Button, TextField, Checkbox, Dropdown, Style, presets};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1000, 700)
        .title("Raylib Interactive Style Demo")
        .build();

    // Create buttons with different styles
    let mut primary_button = Button::new(50.0, 50.0, 150.0, 40.0, "Primary")
        .with_style(presets::button_primary());
    
    let mut secondary_button = Button::new(220.0, 50.0, 150.0, 40.0, "Secondary")
        .with_style(presets::button_secondary());
    
    let mut success_button = Button::new(390.0, 50.0, 150.0, 40.0, "Success")
        .with_style(presets::button_success());
    
    let mut danger_button = Button::new(560.0, 50.0, 150.0, 40.0, "Danger")
        .with_style(presets::button_danger());

    let mut disabled_button = Button::new(730.0, 50.0, 150.0, 40.0, "Disabled")
        .with_style(Style::default());
    disabled_button.enabled = false;

    // Create text fields with different styles
    let mut text_field = TextField::new(50.0, 120.0, 200.0, 30.0, 50)
        .with_style(presets::textfield_default());
    text_field.placeholder = "Enter text here...".to_string();

    let mut password_field = TextField::new(270.0, 120.0, 200.0, 30.0, 20)
        .with_style(Style::minimal());
    password_field.placeholder = "Password".to_string();

    // Create checkboxes
    let mut checkbox1 = Checkbox::new(50.0, 180.0, 20.0, "Enable Feature 1")
        .with_style(presets::checkbox_default());
    
    let mut checkbox2 = Checkbox::new(50.0, 220.0, 20.0, "Enable Feature 2")
        .with_style(Style::dark_theme().with_typography(16));

    // Create dropdowns
    let mut dropdown1 = Dropdown::new(50.0, 280.0, 200.0, 30.0, 
        vec!["Option 1".to_string(), "Option 2".to_string(), "Option 3".to_string(), 
             "Option 4".to_string(), "Option 5".to_string(), "Option 6".to_string()])
        .with_style(presets::dropdown_default());

    let mut dropdown2 = Dropdown::new(270.0, 280.0, 200.0, 30.0, 
        vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()])
        .with_style(Style::modern_blue());

    // Create a custom style
    let custom_style = Style::new(
        Color::new(255, 200, 100, 255),  // Orange background
        Color::new(255, 220, 120, 255),  // Light orange hover
        Color::new(200, 150, 50, 255),   // Dark orange pressed
        Color::new(150, 100, 0, 255),    // Dark orange border
        Color::new(50, 25, 0, 255),      // Dark brown text
    ).with_typography(18)
     .with_layout(10.0, 8.0, 3.0);

    let mut custom_button = Button::new(50.0, 340.0, 200.0, 50.0, "Custom Style")
        .with_style(custom_style);

    // Theme toggle button
    let mut theme_button = Button::new(270.0, 340.0, 150.0, 40.0, "Toggle Theme")
        .with_style(presets::button_secondary());

    let mut is_dark_theme = false;

    // Animation demo
    let mut animated_button = Button::new(50.0, 420.0, 150.0, 40.0, "Animated")
        .with_style(Style::modern_blue());

    while !rl.window_should_close() {
        // Update all components
        primary_button.update(&rl);
        secondary_button.update(&rl);
        success_button.update(&rl);
        danger_button.update(&rl);
        disabled_button.update(&rl);
        text_field.update(&mut rl);
        password_field.update(&mut rl);
        checkbox1.update(&rl);
        checkbox2.update(&rl);
        dropdown1.update(&rl);
        dropdown2.update(&rl);
        custom_button.update(&rl);
        theme_button.update(&rl);
        animated_button.update(&rl);

        // Handle button clicks
        if primary_button.is_clicked(&rl) {
            println!("Primary button clicked!");
        }

        if secondary_button.is_clicked(&rl) {
            println!("Secondary button clicked!");
        }

        if success_button.is_clicked(&rl) {
            println!("Success button clicked!");
        }

        if danger_button.is_clicked(&rl) {
            println!("Danger button clicked!");
        }

        if custom_button.is_clicked(&rl) {
            println!("Custom button clicked!");
        }

        if theme_button.is_clicked(&rl) {
            is_dark_theme = !is_dark_theme;
            let new_style = if is_dark_theme {
                Style::dark_theme()
            } else {
                Style::modern_blue()
            };
            
            // Apply theme to all components
            primary_button.style = new_style.clone();
            secondary_button.style = new_style.clone();
            success_button.style = new_style.clone();
            danger_button.style = new_style.clone();
            text_field.style = new_style.clone();
            checkbox1.style = new_style.clone();
            dropdown1.style = new_style.clone();
        }

        if animated_button.is_clicked(&rl) {
            println!("Animated button clicked!");
        }

        // Handle checkbox toggles
        if checkbox1.is_checked {
            println!("Feature 1 enabled: {}", checkbox1.is_checked);
        }

        if checkbox2.is_checked {
            println!("Feature 2 enabled: {}", checkbox2.is_checked);
        }

        // Handle dropdown selections
        if let Some(selected) = dropdown1.get_selected_item() {
            println!("Dropdown 1 selected: {}", selected);
        }

        if let Some(selected) = dropdown2.get_selected_item() {
            println!("Dropdown 2 selected: {}", selected);
        }

        // Handle text field input
        if !text_field.text.is_empty() {
            println!("Text field content: {}", text_field.text);
        }

        let mut d = rl.begin_drawing(&thread);
        
        // Clear background based on theme
        if is_dark_theme {
            d.clear_background(Color::new(30, 30, 30, 255));
        } else {
            d.clear_background(Color::WHITE);
        }

        // Draw title
        let title_color = if is_dark_theme { Color::WHITE } else { Color::BLACK };
        d.draw_text("Raylib Interactive Style Demo", 50, 10, 24, title_color);
        d.draw_text("Click buttons to see console output", 50, 30, 16, title_color);

        // Draw all components
        primary_button.draw(&mut d);
        secondary_button.draw(&mut d);
        success_button.draw(&mut d);
        danger_button.draw(&mut d);
        disabled_button.draw(&mut d);
        text_field.draw(&mut d);
        password_field.draw(&mut d);
        checkbox1.draw(&mut d);
        checkbox2.draw(&mut d);
        dropdown1.draw(&mut d);
        dropdown2.draw(&mut d);
        custom_button.draw(&mut d);
        theme_button.draw(&mut d);
        animated_button.draw(&mut d);

        // Draw info text
        let info_y = 500;
        d.draw_text("API Methods Demonstrated:", 50, info_y, 18, title_color);
        d.draw_text("• with_style() - Apply custom styles", 50, info_y + 25, 14, title_color);
        d.draw_text("• Direct field access - Modify properties directly", 50, info_y + 45, 14, title_color);
        d.draw_text("• Public fields - No getters/setters needed", 50, info_y + 65, 14, title_color);
        d.draw_text("• Simple API - Direct access to bounds, text, etc.", 50, info_y + 85, 14, title_color);
        d.draw_text("• is_clicked() / is_checked - State queries", 50, info_y + 105, 14, title_color);
        d.draw_text("• get_selected_item() - Dropdown selection", 50, info_y + 125, 14, title_color);

        // Draw style presets info
        let preset_y = 650;
        d.draw_text("Style Presets: default, modern_blue, dark_theme, minimal", 50, preset_y, 14, title_color);
        d.draw_text("Button Presets: primary, secondary, success, danger", 50, preset_y + 20, 14, title_color);
    }
} 