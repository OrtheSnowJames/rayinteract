use raylib::prelude::*;
use raylib_interactive::{Button, TextField, Checkbox, Dropdown, Style, presets};

#[test]
fn test_button_creation() {
    let button = Button::new(100.0, 100.0, 150.0, 40.0, "Test Button");
    assert_eq!(button.label, "Test Button");
    assert_eq!(button.bounds.width, 150.0);
    assert_eq!(button.bounds.height, 40.0);
    assert_eq!(button.bounds.x, 100.0);
    assert_eq!(button.bounds.y, 100.0);
    assert!(button.enabled);
}

#[test]
fn test_button_styling() {
    let style = presets::button_primary();
    let button = Button::new(0.0, 0.0, 100.0, 30.0, "Test").with_style(style.clone());
    assert_eq!(button.style.background_color, style.background_color);
    assert_eq!(button.style.text_color, style.text_color);
}

#[test]
fn test_button_state_changes() {
    let mut button = Button::new(0.0, 0.0, 100.0, 30.0, "Test");
    
    button.enabled = false;
    assert!(!button.enabled);
    
    button.label = "Updated".to_string();
    assert_eq!(button.label, "Updated");
    
    button.bounds.x = 200.0;
    button.bounds.y = 200.0;
    assert_eq!(button.bounds.x, 200.0);
    assert_eq!(button.bounds.y, 200.0);
}

#[test]
fn test_textfield_creation() {
    let textfield = TextField::new(100.0, 100.0, 200.0, 30.0, 50);
    assert_eq!(textfield.max_length, 50);
    assert_eq!(textfield.bounds.width, 200.0);
    assert_eq!(textfield.bounds.height, 30.0);
    assert!(textfield.text.is_empty());
    assert!(!textfield.is_active);
}

#[test]
fn test_textfield_content() {
    let mut textfield = TextField::new(0.0, 0.0, 100.0, 30.0, 20);
    
    textfield.text = "Hello World".to_string();
    assert_eq!(textfield.text, "Hello World");
    
    textfield.placeholder = "Enter text...".to_string();
    assert_eq!(textfield.placeholder, "Enter text...");
    
    textfield.is_active = true;
    assert!(textfield.is_active);
    
    textfield.cursor_position = 5;
    assert_eq!(textfield.cursor_position, 5);
}

#[test]
fn test_checkbox_creation() {
    let checkbox = Checkbox::new(100.0, 100.0, 20.0, "Test Checkbox");
    assert_eq!(checkbox.label, "Test Checkbox");
    assert_eq!(checkbox.bounds.width, 20.0);
    assert_eq!(checkbox.bounds.height, 20.0);
    assert!(!checkbox.is_checked);
}

#[test]
fn test_checkbox_state() {
    let mut checkbox = Checkbox::new(0.0, 0.0, 20.0, "Test");
    
    checkbox.is_checked = true;
    assert!(checkbox.is_checked);
    
    checkbox.label = "Updated".to_string();
    assert_eq!(checkbox.label, "Updated");
    
    checkbox.animation_progress = 0.5;
    assert_eq!(checkbox.animation_progress, 0.5);
    
    checkbox.toggle();
    assert!(!checkbox.is_checked);
}

#[test]
fn test_dropdown_creation() {
    let items = vec!["Item 1".to_string(), "Item 2".to_string(), "Item 3".to_string()];
    let dropdown = Dropdown::new(100.0, 100.0, 200.0, 30.0, items.clone());
    assert_eq!(dropdown.items.len(), 3);
    assert_eq!(dropdown.bounds.width, 200.0);
    assert!(!dropdown.is_open);
    assert_eq!(dropdown.selected_index, None);
}

#[test]
fn test_dropdown_operations() {
    let items = vec!["Item 1".to_string(), "Item 2".to_string()];
    let mut dropdown = Dropdown::new(0.0, 0.0, 100.0, 30.0, items);
    
    dropdown.is_open = true;
    assert!(dropdown.is_open);
    
    dropdown.selected_index = Some(1);
    assert_eq!(dropdown.selected_index, Some(1));
    
    dropdown.add_item("Item 3".to_string());
    assert_eq!(dropdown.items.len(), 3);
    
    dropdown.remove_item(0);
    assert_eq!(dropdown.items.len(), 2);
    
    dropdown.clear_items();
    assert!(dropdown.items.is_empty());
    
    dropdown.toggle();
    assert!(!dropdown.is_open);
}

#[test]
fn test_style_default() {
    let style = Style::default();
    assert_eq!(style.font_size, 20);
    assert_eq!(style.padding, 5.0);
    assert_eq!(style.corner_radius, 5.0);
    assert_eq!(style.border_thickness, 2.0);
}

#[test]
fn test_style_custom() {
    let style = Style::new(
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::WHITE,
        Color::BLACK,
    );
    assert_eq!(style.background_color, Color::RED);
    assert_eq!(style.hover_color, Color::GREEN);
    assert_eq!(style.pressed_color, Color::BLUE);
    assert_eq!(style.border_color, Color::WHITE);
    assert_eq!(style.text_color, Color::BLACK);
}

#[test]
fn test_style_presets() {
    let primary = presets::button_primary();
    assert_eq!(primary.text_color, Color::WHITE);
    
    let secondary = presets::button_secondary();
    assert_eq!(secondary.background_color, Color::new(240, 240, 240, 255));
    
    let success = presets::button_success();
    assert_eq!(success.background_color, Color::new(40, 167, 69, 255));
    
    let danger = presets::button_danger();
    assert_eq!(danger.background_color, Color::new(220, 53, 69, 255));
}

#[test]
fn test_style_builder() {
    let style = Style::default()
        .with_typography(24)
        .with_layout(10.0, 8.0, 3.0);
    
    assert_eq!(style.font_size, 24);
    assert_eq!(style.padding, 10.0);
    assert_eq!(style.corner_radius, 8.0);
    assert_eq!(style.border_thickness, 3.0);
}

#[test]
fn test_style_themes() {
    let modern = Style::modern_blue();
    assert_eq!(modern.background_color, Color::WHITE);
    assert_eq!(modern.font_size, 16);
    
    let dark = Style::dark_theme();
    assert_eq!(dark.background_color, Color::new(40, 40, 40, 255));
    assert_eq!(dark.text_color, Color::WHITE);
    
    let minimal = Style::minimal();
    assert_eq!(minimal.font_size, 14);
    assert_eq!(minimal.border_thickness, 1.0);
}

#[test]
fn test_all_components_with_styles() {
    // Test that all components can use the same style
    let style = Style::modern_blue();
    
    let button = Button::new(0.0, 0.0, 100.0, 30.0, "Test").with_style(style.clone());
    let textfield = TextField::new(0.0, 0.0, 100.0, 30.0, 20).with_style(style.clone());
    let checkbox = Checkbox::new(0.0, 0.0, 20.0, "Test").with_style(style.clone());
    let dropdown = Dropdown::new(0.0, 0.0, 100.0, 30.0, vec![]).with_style(style.clone());
    
    // All should have the same style
    assert_eq!(button.style.font_size, textfield.style.font_size);
    assert_eq!(textfield.style.font_size, checkbox.style.font_size);
    assert_eq!(checkbox.style.font_size, dropdown.style.font_size);
    assert_eq!(dropdown.style.font_size, 16); // modern_blue font size
} 