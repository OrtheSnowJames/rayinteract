mod checkbox;
mod dropdown;
mod textfield;
mod button;

use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const PADDING: f32 = 20.0;

fn main() {
    let mut rl = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Interactive UI Elements")
        .msaa_4x()
        .vsync()
        .build();

    // Initialize UI elements with proper spacing
    let mut checkbox = checkbox::Checkbox::new(
        PADDING,
        PADDING,
        24.0,
        "Enable Feature",
    );
    checkbox.set_colors(
        Color::WHITE,
        Color::GREEN,
        Color::BLACK,
        Color::new(245, 245, 245, 255),
        Color::BLACK,
    );

    let mut dropdown = dropdown::Dropdown::new(
        PADDING,
        PADDING * 3.0,
        200.0,
        30.0,
        vec![
            "Option 1".to_string(),
            "Option 2".to_string(),
            "Option 3".to_string(),
            "Option 4".to_string(),
            "Option 5".to_string(),
        ],
    );
    dropdown.set_max_visible_items(4);

    let mut text_field = textfield::TextField::new(
        PADDING,
        PADDING * 6.0,
        250.0,
        35.0,
        32,
    );

    let mut submit_button = button::Button::new(
        PADDING,
        PADDING * 9.0,
        120.0,
        40.0,
        "Submit",
    );
    submit_button.set_corner_radius(5.0);

    let mut reset_button = button::Button::new(
        PADDING * 2.0 + 120.0,
        PADDING * 9.0,
        120.0,
        40.0,
        "Reset",
    );
    reset_button.set_colors(
        Color::new(255, 200, 200, 255),
        Color::new(255, 150, 150, 255),
        Color::new(255, 100, 100, 255),
        Color::BLACK,
        Color::BLACK,
    );
    reset_button.set_corner_radius(5.0);

    while !rl.window_should_close() {
        // Update
        checkbox.update(&rl);
        dropdown.update(&rl);
        text_field.update(&mut rl);
        submit_button.update(&rl);
        reset_button.update(&rl);

        // Handle button clicks
        if submit_button.is_clicked(&rl) {
            println!("Form submitted!");
            println!("Checkbox: {}", checkbox.is_checked());
            println!("Dropdown: {:?}", dropdown.get_selected_item());
            println!("Text: {}", text_field.get_text());
        }

        if reset_button.is_clicked(&rl) {
            checkbox.set_checked(false);
            text_field = textfield::TextField::new(
                PADDING,
                PADDING * 6.0,
                250.0,
                35.0,
                32,
            );
        }

        // Draw
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(245, 245, 245, 255));

        // Draw title
        d.draw_text(
            "Interactive UI Elements",
            PADDING as i32,
            (WINDOW_HEIGHT - 40) as i32,
            24,
            Color::DARKGRAY,
        );

        // Draw UI elements
        checkbox.draw(&mut d);
        dropdown.draw(&mut d);
        text_field.draw(&mut d);
        submit_button.draw(&mut d);
        reset_button.draw(&mut d);

        // Draw help text
        let help_text = "Try interacting with the elements above!";
        d.draw_text(
            help_text,
            (WINDOW_WIDTH - d.measure_text(help_text, 20)) / 2,
            (WINDOW_HEIGHT - 80) as i32,
            20,
            Color::DARKGRAY,
        );
    }
}

