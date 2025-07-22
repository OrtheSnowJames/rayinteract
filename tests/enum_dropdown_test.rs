use raylib::prelude::*;
use raylib_interactive::{Button, Dropdown, Style, presets};

#[derive(Debug, Clone, PartialEq)]
enum TestOption {
    Option1,
    Option2,
    Option3,
    Option4,
    Option5,
    Option6,
    Option7,
    Option8,
}

impl TestOption {
    fn to_string(&self) -> String {
        match self {
            TestOption::Option1 => "First Option".to_string(),
            TestOption::Option2 => "Second Option".to_string(),
            TestOption::Option3 => "Third Option".to_string(),
            TestOption::Option4 => "Fourth Option".to_string(),
            TestOption::Option5 => "Fifth Option".to_string(),
            TestOption::Option6 => "Sixth Option".to_string(),
            TestOption::Option7 => "Seventh Option".to_string(),
            TestOption::Option8 => "Eighth Option".to_string(),
        }
    }

    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(TestOption::Option1),
            1 => Some(TestOption::Option2),
            2 => Some(TestOption::Option3),
            3 => Some(TestOption::Option4),
            4 => Some(TestOption::Option5),
            5 => Some(TestOption::Option6),
            6 => Some(TestOption::Option7),
            7 => Some(TestOption::Option8),
            _ => None,
        }
    }

    fn all_options() -> Vec<TestOption> {
        vec![
            TestOption::Option1,
            TestOption::Option2,
            TestOption::Option3,
            TestOption::Option4,
            TestOption::Option5,
            TestOption::Option6,
            TestOption::Option7,
            TestOption::Option8,
        ]
    }
}

#[test]
fn test_enum_dropdown_creation() {
    let enum_options = TestOption::all_options();
    let dropdown_items: Vec<String> = enum_options.iter().map(|opt| opt.to_string()).collect();
    
    let dropdown = Dropdown::new(0.0, 0.0, 300.0, 30.0, dropdown_items);
    
    assert_eq!(dropdown.items.len(), 8);
    assert_eq!(dropdown.items[0], "First Option");
    assert_eq!(dropdown.items[1], "Second Option");
    assert_eq!(dropdown.items[7], "Eighth Option");
}

#[test]
fn test_enum_dropdown_selection() {
    let enum_options = TestOption::all_options();
    let dropdown_items: Vec<String> = enum_options.iter().map(|opt| opt.to_string()).collect();
    
    let mut dropdown = Dropdown::new(0.0, 0.0, 300.0, 30.0, dropdown_items);
    
    // Test selection by index
    dropdown.selected_index = Some(2);
    assert_eq!(dropdown.selected_index, Some(2));
    
    // Test getting selected enum
    if let Some(selected_index) = dropdown.selected_index {
        if let Some(selected_enum) = TestOption::from_index(selected_index) {
            assert_eq!(selected_enum, TestOption::Option3);
        }
    }
}

#[test]
fn test_enum_dropdown_all_selections() {
    let enum_options = TestOption::all_options();
    let dropdown_items: Vec<String> = enum_options.iter().map(|opt| opt.to_string()).collect();
    
    let mut dropdown = Dropdown::new(0.0, 0.0, 300.0, 30.0, dropdown_items);
    
    // Test all possible selections
    for i in 0..8 {
        dropdown.selected_index = Some(i);
        if let Some(selected_enum) = TestOption::from_index(i) {
            assert_eq!(dropdown.selected_index, Some(i));
            assert_eq!(dropdown.items[i], selected_enum.to_string());
        }
    }
}

#[test]
fn test_enum_dropdown_with_yes_no_logic() {
    let enum_options = TestOption::all_options();
    let dropdown_items: Vec<String> = enum_options.iter().map(|opt| opt.to_string()).collect();
    
    let mut dropdown = Dropdown::new(0.0, 0.0, 300.0, 30.0, dropdown_items);
    let mut test_variable: i32 = -1;
    
    // Simulate YES button click
    test_variable = 1;
    assert_eq!(test_variable, 1);
    
    // Test that assertion would pass
    assert_eq!(test_variable, 1);
    
    // Simulate NO button click
    test_variable = 0;
    assert_eq!(test_variable, 0);
    
    // Test that assertion would fail
    assert_ne!(test_variable, 1);
}

#[test]
fn test_enum_dropdown_with_selection_and_yes_no() {
    let enum_options = TestOption::all_options();
    let dropdown_items: Vec<String> = enum_options.iter().map(|opt| opt.to_string()).collect();
    
    let mut dropdown = Dropdown::new(0.0, 0.0, 300.0, 30.0, dropdown_items);
    let mut test_variable: i32 = -1;
    
    // Select an option
    dropdown.selected_index = Some(3);
    let selected_enum = TestOption::from_index(3).unwrap();
    assert_eq!(selected_enum, TestOption::Option4);
    
    // Simulate YES button (test passes)
    test_variable = 1;
    assert_eq!(test_variable, 1);
    
    // Simulate NO button (test fails)
    test_variable = 0;
    assert_ne!(test_variable, 1);
}

#[test]
fn test_enum_dropdown_reset_functionality() {
    let enum_options = TestOption::all_options();
    let dropdown_items: Vec<String> = enum_options.iter().map(|opt| opt.to_string()).collect();
    
    let mut dropdown = Dropdown::new(0.0, 0.0, 300.0, 30.0, dropdown_items);
    let mut test_variable: i32 = 1;
    
    // Set a selection
    dropdown.selected_index = Some(5);
    assert_eq!(dropdown.selected_index, Some(5));
    
    // Reset
    dropdown.selected_index = None;
    test_variable = -1;
    
    assert_eq!(dropdown.selected_index, None);
    assert_eq!(test_variable, -1);
}

#[test]
fn test_enum_dropdown_max_visible_items() {
    let enum_options = TestOption::all_options();
    let dropdown_items: Vec<String> = enum_options.iter().map(|opt| opt.to_string()).collect();
    
    let mut dropdown = Dropdown::new(0.0, 0.0, 300.0, 30.0, dropdown_items);
    
    // Set max visible items to show scrolling
    dropdown.max_visible_items = 4;
    assert_eq!(dropdown.max_visible_items, 4);
    
    // Test that we have more items than visible
    assert!(dropdown.items.len() > dropdown.max_visible_items);
    assert_eq!(dropdown.items.len(), 8);
}

#[test]
fn test_enum_dropdown_comprehensive() {
    let enum_options = TestOption::all_options();
    let dropdown_items: Vec<String> = enum_options.iter().map(|opt| opt.to_string()).collect();
    
    let mut dropdown = Dropdown::new(0.0, 0.0, 300.0, 30.0, dropdown_items);
    let mut test_variable: i32 = -1;
    
    // Test initial state
    assert_eq!(dropdown.selected_index, None);
    assert_eq!(test_variable, -1);
    
    // Select an option
    dropdown.selected_index = Some(1);
    let selected_enum = TestOption::from_index(1).unwrap();
    assert_eq!(selected_enum, TestOption::Option2);
    assert_eq!(selected_enum.to_string(), "Second Option");
    
    // Simulate YES button
    test_variable = 1;
    assert_eq!(test_variable, 1);
    
    // Test assertion passes
    assert_eq!(test_variable, 1);
    
    // Simulate NO button
    test_variable = 0;
    assert_eq!(test_variable, 0);
    
    // Test assertion fails
    assert_ne!(test_variable, 1);
    
    // Reset everything
    dropdown.selected_index = None;
    test_variable = -1;
    
    assert_eq!(dropdown.selected_index, None);
    assert_eq!(test_variable, -1);
} 