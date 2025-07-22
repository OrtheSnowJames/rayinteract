use raylib::prelude::*;
use raylib_interactive::{Button, TextField, Checkbox, Dropdown, Style, presets, TestResults, run_all_tests, update_all, draw_all};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1000, 800)
        .title("Raylib Interactive Test Runner")
        .build();

    // Test results
    let mut test_results = run_all_tests();
    let mut tests_run = false;

    // UI Components
    let mut run_tests_button = Button::new(50.0, 50.0, 150.0, 40.0, "Run Tests")
        .with_style(presets::button_primary());

    let mut yes_button = Button::new(50.0, 700.0, 100.0, 40.0, "YES")
        .with_style(presets::button_success());
    yes_button.enabled = false;

    let mut no_button = Button::new(170.0, 700.0, 100.0, 40.0, "NO")
        .with_style(presets::button_danger());
    no_button.enabled = false;

    let mut reset_button = Button::new(290.0, 700.0, 100.0, 40.0, "Reset")
        .with_style(presets::button_secondary());

    // Demo components to show they work
    let mut demo_button = Button::new(250.0, 50.0, 120.0, 40.0, "Demo Button")
        .with_style(presets::button_secondary());

    let mut demo_textfield = TextField::new(400.0, 50.0, 150.0, 30.0, 20)
        .with_style(presets::textfield_default());
    demo_textfield.placeholder = "Type here...".to_string();

    let mut demo_checkbox = Checkbox::new(580.0, 50.0, 20.0, "Demo Checkbox")
        .with_style(presets::checkbox_default());

    let mut demo_dropdown = Dropdown::new(750.0, 50.0, 150.0, 30.0, 
        vec!["Option A".to_string(), "Option B".to_string(), "Option C".to_string()])
        .with_style(presets::dropdown_default());

    // Status text
    let mut status_text = "Click 'Run Tests' to start testing".to_string();

    while !rl.window_should_close() {
        // Update all components
        update_all!(&mut rl, Vector2::new(0.0, 0.0), run_tests_button, yes_button, no_button, reset_button, demo_button, demo_textfield, demo_checkbox, demo_dropdown);

        // Handle button clicks
        if run_tests_button.is_clicked(&rl) && !tests_run {
            test_results = run_all_tests();
            tests_run = true;
            status_text = "Tests completed! Check results below.".to_string();
            yes_button.enabled = true;
            no_button.enabled = true;
        }

        if yes_button.is_clicked(&rl) {
            if test_results.all_passed {
                status_text = "Correct! All tests passed!".to_string();
            } else {
                status_text = "Incorrect! Some tests failed!".to_string();
            }
        }

        if no_button.is_clicked(&rl) {
            if !test_results.all_passed {
                status_text = "Correct! Some tests failed!".to_string();
            } else {
                status_text = "Incorrect! All tests actually passed!".to_string();
            }
        }

        if reset_button.is_clicked(&rl) {
            tests_run = false;
            status_text = "Click 'Run Tests' to start testing".to_string();
            yes_button.enabled = false;
            no_button.enabled = false;
        }

        // Demo interactions
        if demo_button.is_clicked(&rl) {
            println!("Demo button clicked!");
        }

        if demo_checkbox.is_checked {
            println!("Demo checkbox is checked!");
        }

        if let Some(selected) = demo_dropdown.get_selected_item() {
            println!("Demo dropdown selected: {}", selected);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        // Draw title
        d.draw_text("Raylib Interactive Test Runner", 50, 10, 24, Color::BLACK);
        d.draw_text(&status_text, 50, 30, 16, Color::BLACK);

        // Draw demo components
        d.draw_text("Demo Components (test interaction):", 250, 20, 14, Color::GRAY);
        draw_all!(&mut d, demo_button, demo_textfield, demo_checkbox, demo_dropdown);

        // Draw test results
        if tests_run {
            draw_test_results(&mut d, &test_results);
        }

        // Draw control buttons
        d.draw_text("Controls:", 50, 670, 16, Color::BLACK);
        run_tests_button.draw(&mut d);
        yes_button.draw(&mut d);
        no_button.draw(&mut d);
        reset_button.draw(&mut d);

        // Draw instructions
        let instructions_y = 750;
        d.draw_text("Instructions:", 50, instructions_y, 14, Color::GRAY);
        d.draw_text("1. Click 'Run Tests' to execute all tests", 50, instructions_y + 20, 12, Color::GRAY);
        d.draw_text("2. Review the test results below", 50, instructions_y + 35, 12, Color::GRAY);
        d.draw_text("3. Click YES if all tests passed, NO if any failed", 50, instructions_y + 50, 12, Color::GRAY);
        d.draw_text("4. Click Reset to run tests again", 50, instructions_y + 65, 12, Color::GRAY);
    }
}

fn draw_test_results(d: &mut RaylibDrawHandle, results: &TestResults) {
    let start_y = 120;
    let line_height = 25;
    let mut y = start_y;

    // Title
    d.draw_text("Test Results:", 50, y, 18, Color::BLACK);
    y += line_height + 10;

    // Individual test results
    let test_color = if results.button_tests { Color::GREEN } else { Color::RED };
    let test_status = if results.button_tests { "PASS" } else { "FAIL" };
    d.draw_text(&format!("Button Tests: {}", test_status), 50, y, 16, test_color);
    y += line_height;

    let test_color = if results.textfield_tests { Color::GREEN } else { Color::RED };
    let test_status = if results.textfield_tests { "PASS" } else { "FAIL" };
    d.draw_text(&format!("TextField Tests: {}", test_status), 50, y, 16, test_color);
    y += line_height;

    let test_color = if results.checkbox_tests { Color::GREEN } else { Color::RED };
    let test_status = if results.checkbox_tests { "PASS" } else { "FAIL" };
    d.draw_text(&format!("Checkbox Tests: {}", test_status), 50, y, 16, test_color);
    y += line_height;

    let test_color = if results.dropdown_tests { Color::GREEN } else { Color::RED };
    let test_status = if results.dropdown_tests { "PASS" } else { "FAIL" };
    d.draw_text(&format!("Dropdown Tests: {}", test_status), 50, y, 16, test_color);
    y += line_height;

    let test_color = if results.style_tests { Color::GREEN } else { Color::RED };
    let test_status = if results.style_tests { "PASS" } else { "FAIL" };
    d.draw_text(&format!("Style Tests: {}", test_status), 50, y, 16, test_color);
    y += line_height + 10;

    // Overall result
    let overall_color = if results.all_passed { Color::GREEN } else { Color::RED };
    let overall_status = if results.all_passed { "ALL TESTS PASSED" } else { "SOME TESTS FAILED" };
    d.draw_text(&format!("Overall: {}", overall_status), 50, y, 18, overall_color);
    y += line_height + 20;

    // Test details
    d.draw_text("Test Details:", 50, y, 16, Color::BLACK);
    y += line_height;

    let details = vec![
        "• Button: Creation, styling, state changes, bounds",
        "• TextField: Creation, styling, text content, placeholder, cursor",
        "• Checkbox: Creation, styling, state, label, animation, toggle",
        "• Dropdown: Creation, styling, state, selection, items, clear",
        "• Style: Default, custom, presets, builder methods, themes"
    ];

    for detail in details {
        d.draw_text(detail, 50, y, 12, Color::GRAY);
        y += 20;
    }

    // Verification prompt
    y += 20;
    d.draw_text("Verification:", 50, y, 16, Color::BLACK);
    y += line_height;
    d.draw_text("Based on the results above, click YES or NO to verify:", 50, y, 14, Color::GRAY);
    y += 20;
    d.draw_text("YES = All tests passed correctly", 50, y, 12, Color::GREEN);
    y += 15;
    d.draw_text("NO = Some tests failed", 50, y, 12, Color::RED);
} 