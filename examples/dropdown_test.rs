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

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Dropdown Test with Enum")
        .build();

    // Test variable that starts at -1
    let mut test_variable: i32 = -1;
    
    // Convert enum options to strings for the dropdown
    let enum_options = TestOption::all_options();
    let dropdown_items: Vec<String> = enum_options.iter().map(|opt| opt.to_string()).collect();
    
    // Create dropdown with enum-based items
    let mut dropdown = Dropdown::new(50.0, 100.0, 300.0, 30.0, dropdown_items)
        .with_style(presets::dropdown_default());
    
    // Set max visible items to show scrolling
    dropdown.max_visible_items = 4;

    // Control buttons
    let mut yes_button = Button::new(50.0, 200.0, 100.0, 40.0, "YES")
        .with_style(presets::button_success());

    let mut no_button = Button::new(170.0, 200.0, 100.0, 40.0, "NO")
        .with_style(presets::button_danger());

    let mut reset_button = Button::new(290.0, 200.0, 100.0, 40.0, "Reset")
        .with_style(presets::button_secondary());

    let mut run_test_button = Button::new(410.0, 200.0, 120.0, 40.0, "Run Test")
        .with_style(presets::button_primary());

    // Status and info
    let mut status_text = "Select an option from dropdown, then click YES/NO".to_string();
    let mut test_result = "Test not run yet".to_string();
    let mut selected_option_text = "No option selected".to_string();

    while !rl.window_should_close() {
        // Update all components
        dropdown.update(&rl);
        yes_button.update(&rl);
        no_button.update(&rl);
        reset_button.update(&rl);
        run_test_button.update(&rl);

        // Handle button clicks
        if yes_button.is_clicked(&rl) {
            test_variable = 1;
            status_text = "Variable set to 1 (YES clicked)".to_string();
        }

        if no_button.is_clicked(&rl) {
            test_variable = 0;
            status_text = "Variable set to 0 (NO clicked)".to_string();
        }

        if reset_button.is_clicked(&rl) {
            test_variable = -1;
            dropdown.selected_index = None;
            status_text = "Reset: Variable = -1, no selection".to_string();
            test_result = "Test not run yet".to_string();
        }

        if run_test_button.is_clicked(&rl) {
            // Run the test assertion
            if test_variable == 1 {
                test_result = "Test PASSED: Variable is 1".to_string();
            } else {
                test_result = "Test FAILED: Variable is not 1".to_string();
            }
        }

        // Update selected option display
        if let Some(selected_index) = dropdown.selected_index {
            if let Some(selected_enum) = TestOption::from_index(selected_index) {
                selected_option_text = format!("Selected: {:?} ({})", selected_enum, selected_enum.to_string());
            }
        } else {
            selected_option_text = "No option selected".to_string();
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        // Draw title
        d.draw_text("Dropdown Test with Enum", 50, 20, 24, Color::BLACK);
        d.draw_text("Test Variable Value:", 50, 50, 16, Color::BLACK);
        d.draw_text(&format!("{}", test_variable), 250, 50, 16, 
            if test_variable == 1 { Color::GREEN } else if test_variable == 0 { Color::RED } else { Color::GRAY });

        // Draw dropdown
        d.draw_text("Enum-based Dropdown (8 options, shows 4):", 50, 80, 14, Color::BLACK);
        dropdown.draw(&mut d);

        // Draw selected option info
        d.draw_text(&selected_option_text, 50, 150, 14, Color::BLACK);

        // Draw control buttons
        d.draw_text("Controls:", 50, 180, 16, Color::BLACK);
        yes_button.draw(&mut d);
        no_button.draw(&mut d);
        reset_button.draw(&mut d);
        run_test_button.draw(&mut d);

        // Draw status
        d.draw_text("Status:", 50, 260, 16, Color::BLACK);
        d.draw_text(&status_text, 50, 280, 14, Color::BLACK);

        // Draw test result
        d.draw_text("Test Result:", 50, 310, 16, Color::BLACK);
        d.draw_text(&test_result, 50, 330, 14, 
            if test_result.contains("PASSED") { Color::GREEN } else if test_result.contains("FAILED") { Color::RED } else { Color::GRAY });

        // Draw instructions
        let instructions_y = 380;
        d.draw_text("Instructions:", 50, instructions_y, 16, Color::BLACK);
        d.draw_text("1. Select an option from the dropdown", 50, instructions_y + 25, 12, Color::GRAY);
        d.draw_text("2. Click YES to set variable to 1 (test will pass)", 50, instructions_y + 40, 12, Color::GRAY);
        d.draw_text("3. Click NO to set variable to 0 (test will fail)", 50, instructions_y + 55, 12, Color::GRAY);
        d.draw_text("4. Click 'Run Test' to execute assertion", 50, instructions_y + 70, 12, Color::GRAY);
        d.draw_text("5. Click Reset to start over", 50, instructions_y + 85, 12, Color::GRAY);

        // Draw enum info
        let enum_y = 480;
        d.draw_text("Enum Options:", 50, enum_y, 16, Color::BLACK);
        d.draw_text("TestOption::Option1 -> 'First Option'", 50, enum_y + 20, 12, Color::GRAY);
        d.draw_text("TestOption::Option2 -> 'Second Option'", 50, enum_y + 35, 12, Color::GRAY);
        d.draw_text("TestOption::Option3 -> 'Third Option'", 50, enum_y + 50, 12, Color::GRAY);
        d.draw_text("TestOption::Option4 -> 'Fourth Option'", 50, enum_y + 65, 12, Color::GRAY);
        d.draw_text("TestOption::Option5 -> 'Fifth Option'", 50, enum_y + 80, 12, Color::GRAY);
        d.draw_text("TestOption::Option6 -> 'Sixth Option'", 50, enum_y + 95, 12, Color::GRAY);
        d.draw_text("TestOption::Option7 -> 'Seventh Option'", 50, enum_y + 110, 12, Color::GRAY);
        d.draw_text("TestOption::Option8 -> 'Eighth Option'", 50, enum_y + 125, 12, Color::GRAY);

        // Draw assertion info
        let assertion_y = 620;
        d.draw_text("Assertion Logic:", 50, assertion_y, 16, Color::BLACK);
        d.draw_text("assert_eq!(test_variable, 1);", 50, assertion_y + 20, 12, Color::BLUE);
        d.draw_text("• YES button sets variable to 1 -> assertion passes", 50, assertion_y + 35, 12, Color::GREEN);
        d.draw_text("• NO button sets variable to 0 -> assertion fails", 50, assertion_y + 50, 12, Color::RED);
    }
} 