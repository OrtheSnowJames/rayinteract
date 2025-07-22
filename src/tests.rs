use raylib::prelude::*;
use crate::{Button, TextField, Checkbox, Dropdown, Style, presets};

pub struct TestResults {
    pub button_tests: bool,
    pub textfield_tests: bool,
    pub checkbox_tests: bool,
    pub dropdown_tests: bool,
    pub style_tests: bool,
    pub all_passed: bool,
}

impl TestResults {
    pub fn new() -> Self {
        Self {
            button_tests: false,
            textfield_tests: false,
            checkbox_tests: false,
            dropdown_tests: false,
            style_tests: false,
            all_passed: false,
        }
    }

    pub fn update_all_passed(&mut self) {
        self.all_passed = self.button_tests && 
                          self.textfield_tests && 
                          self.checkbox_tests && 
                          self.dropdown_tests && 
                          self.style_tests;
    }
}

pub fn run_button_tests() -> bool {
    // Test 1: Button creation
    let mut button = Button::new(100.0, 100.0, 150.0, 40.0, "Test Button");
    if button.label != "Test Button" || button.bounds.width != 150.0 || button.bounds.height != 40.0 {
        return false;
    }

    // Test 2: Button styling
    let style = presets::button_primary();
    button = button.with_style(style.clone());
    if button.style.background_color != style.background_color {
        return false;
    }

    // Test 3: Button state changes
    button.enabled = false;
    if button.enabled {
        return false;
    }

    button.label = "Updated Label".to_string();
    if button.label != "Updated Label" {
        return false;
    }

    // Test 4: Button bounds
    button.bounds.x = 200.0;
    button.bounds.y = 200.0;
    if button.bounds.x != 200.0 || button.bounds.y != 200.0 {
        return false;
    }

    true
}

pub fn run_textfield_tests() -> bool {
    // Test 1: TextField creation
    let mut textfield = TextField::new(100.0, 100.0, 200.0, 30.0, 50);
    if textfield.max_length != 50 || textfield.bounds.width != 200.0 {
        return false;
    }

    // Test 2: TextField styling
    let style = presets::textfield_default();
    textfield = textfield.with_style(style.clone());
    if textfield.style.background_color != style.background_color {
        return false;
    }

    // Test 3: TextField content
    textfield.text = "Hello World".to_string();
    if textfield.text != "Hello World" {
        return false;
    }

    // Test 4: TextField placeholder
    textfield.placeholder = "Enter text...".to_string();
    if textfield.placeholder != "Enter text..." {
        return false;
    }

    // Test 5: TextField state
    textfield.is_active = true;
    if !textfield.is_active {
        return false;
    }

    // Test 6: TextField cursor
    textfield.cursor_position = 5;
    if textfield.cursor_position != 5 {
        return false;
    }

    true
}

pub fn run_checkbox_tests() -> bool {
    // Test 1: Checkbox creation
    let mut checkbox = Checkbox::new(100.0, 100.0, 20.0, "Test Checkbox");
    if checkbox.label != "Test Checkbox" || checkbox.bounds.width != 20.0 {
        return false;
    }

    // Test 2: Checkbox styling
    let style = presets::checkbox_default();
    checkbox = checkbox.with_style(style.clone());
    if checkbox.style.check_color != style.check_color {
        return false;
    }

    // Test 3: Checkbox state
    checkbox.is_checked = true;
    if !checkbox.is_checked {
        return false;
    }

    // Test 4: Checkbox label
    checkbox.label = "Updated Checkbox".to_string();
    if checkbox.label != "Updated Checkbox" {
        return false;
    }

    // Test 5: Checkbox animation
    checkbox.animation_progress = 0.5;
    if checkbox.animation_progress != 0.5 {
        return false;
    }

    // Test 6: Checkbox toggle
    checkbox.toggle();
    if checkbox.is_checked {
        return false;
    }

    true
}

pub fn run_dropdown_tests() -> bool {
    // Test 1: Dropdown creation
    let items = vec!["Item 1".to_string(), "Item 2".to_string(), "Item 3".to_string()];
    let mut dropdown = Dropdown::new(100.0, 100.0, 200.0, 30.0, items.clone());
    if dropdown.items.len() != 3 || dropdown.bounds.width != 200.0 {
        return false;
    }

    // Test 2: Dropdown styling
    let style = presets::dropdown_default();
    dropdown = dropdown.with_style(style.clone());
    if dropdown.style.background_color != style.background_color {
        return false;
    }

    // Test 3: Dropdown state
    dropdown.is_open = true;
    if !dropdown.is_open {
        return false;
    }

    // Test 4: Dropdown selection
    dropdown.selected_index = Some(1);
    if dropdown.selected_index != Some(1) {
        return false;
    }

    // Test 5: Dropdown items
    dropdown.add_item("Item 4".to_string());
    if dropdown.items.len() != 4 {
        return false;
    }

    // Test 6: Dropdown item removal
    dropdown.remove_item(0);
    if dropdown.items.len() != 3 {
        return false;
    }

    // Test 7: Dropdown clear
    dropdown.clear_items();
    if !dropdown.items.is_empty() {
        return false;
    }

    // Test 8: Dropdown toggle
    dropdown.toggle();
    if dropdown.is_open {
        return false;
    }


    // Test 9: Dropdown from enum
    #[derive(Debug, Clone, PartialEq)]
    enum TestOption {
        Option1,
        Option2,
        Option3,
    }

    impl ToString for TestOption {
        fn to_string(&self) -> String {
            format!("{:?}", self)
        }
    }

    let items = vec![TestOption::Option1, TestOption::Option2, TestOption::Option3];
    let mut dropdown = Dropdown::from_enum(100.0, 100.0, 200.0, 30.0, &items);
    if dropdown.items.len() != 3 || dropdown.bounds.width != 200.0 {
        return false;
    }

    // Test 10: Show the dropdown to user and make them confirm with yes or no button
    let mut yes_button = Button::new(100.0, 100.0, 100.0, 30.0, "Yes").with_style(presets::button_success());
    let mut no_button = Button::new(200.0, 100.0, 100.0, 30.0, "No").with_style(presets::button_danger());

    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Dropdown Test")
        .build();

    while !rl.window_should_close() {
        yes_button.update(&rl);
        no_button.update(&rl);
        dropdown.update(&rl);

        if yes_button.is_clicked(&rl) {
            return true;
        }

        if no_button.is_clicked(&rl) {
            return false;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        dropdown.draw(&mut d);
        yes_button.draw(&mut d);
        no_button.draw(&mut d);
    }
     
    false
}

pub fn run_style_tests() -> bool {
    // Test 1: Default style
    let style = Style::default();
    if style.font_size != 20 || style.padding != 5.0 {
        return false;
    }

    // Test 2: Custom style
    let custom_style = Style::new(
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::WHITE,
        Color::BLACK,
    );
    if custom_style.background_color != Color::RED {
        return false;
    }

    // Test 3: Style presets
    let primary_style = presets::button_primary();
    if primary_style.text_color != Color::WHITE {
        return false;
    }

    let secondary_style = presets::button_secondary();
    if secondary_style.background_color != Color::new(240, 240, 240, 255) {
        return false;
    }

    // Test 4: Style builder methods
    let modified_style = style
        .with_typography(24)
        .with_layout(10.0, 8.0, 3.0);
    
    if modified_style.font_size != 24 || modified_style.padding != 10.0 {
        return false;
    }

    // Test 5: Theme styles
    let modern_style = Style::modern_blue();
    if modern_style.background_color != Color::WHITE {
        return false;
    }

    let dark_style = Style::dark_theme();
    if dark_style.background_color != Color::new(40, 40, 40, 255) {
        return false;
    }

    let minimal_style = Style::minimal();
    if minimal_style.font_size != 14 {
        return false;
    }

    true
}

pub fn run_all_tests() -> TestResults {
    let mut results = TestResults::new();
    
    results.button_tests = run_button_tests();
    results.textfield_tests = run_textfield_tests();
    results.checkbox_tests = run_checkbox_tests();
    results.dropdown_tests = run_dropdown_tests();
    results.style_tests = run_style_tests();
    
    results.update_all_passed();
    results
} 