use raylib::prelude::*;
use raylib_interactive::{Button, TextField, Checkbox, Dropdown, Style, presets};
use regex::Regex;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1000, 700)
        .title("Raylib Interactive Style Demo")
        .resizable()
        .build();

    // Fixed virtual resolution for UI
    const VIRTUAL_WIDTH: i32 = 1000;
    const VIRTUAL_HEIGHT: i32 = 700;
    let mut render_texture: RenderTexture2D = rl.load_render_texture(&thread, VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32).unwrap();

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
        .with_style(Style::minimal())
        .only_allow(Regex::new("^[^ ]$").unwrap())
        .change_character(|_| '*');
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
        .with_style(presets::dropdown_default())
        .with_deselect_option("Deselect");

    let mut dropdown2 = Dropdown::new(270.0, 280.0, 200.0, 30.0, 
        vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()])
        .with_style(Style::modern_blue())
        .with_deselect_option("Deselect");

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
        .with_style(custom_style.clone());

    // Theme toggle button
    let mut theme_button = Button::new(270.0, 340.0, 200.0, 40.0, "Toggle Theme")
        .with_style(presets::button_secondary());

    let mut is_dark_theme = false;

    // Animation demo
    let mut animated_button = Button::new(50.0, 420.0, 150.0, 40.0, "Animated")
        .with_style(Style::modern_blue());

    while !rl.window_should_close() {
        // Calculate scale and offset for virtual rendering
        let screen_width = rl.get_screen_width();
        let screen_height = rl.get_screen_height();
        let scale_x = screen_width as f32 / VIRTUAL_WIDTH as f32;
        let scale_y = screen_height as f32 / VIRTUAL_HEIGHT as f32;
        let scale = scale_x.min(scale_y);
        let draw_width = (VIRTUAL_WIDTH as f32 * scale).round() as i32;
        let draw_height = (VIRTUAL_HEIGHT as f32 * scale).round() as i32;
        let offset_x = (screen_width - draw_width) / 2;
        let offset_y = (screen_height - draw_height) / 2;

        // Transform mouse position to virtual coordinates
        let mouse = rl.get_mouse_position();
        let virtual_mouse = Vector2 {
            x: ((mouse.x - offset_x as f32) / scale).clamp(0.0, VIRTUAL_WIDTH as f32),
            y: ((mouse.y - offset_y as f32) / scale).clamp(0.0, VIRTUAL_HEIGHT as f32),
        };

        // Update all components with virtual mouse position
        raylib_interactive::update_all!(
            &mut rl,
            virtual_mouse,
            primary_button,
            secondary_button,
            success_button,
            danger_button,
            disabled_button,
            text_field,
            password_field,
            checkbox1,
            checkbox2,
            dropdown1,
            dropdown2,
            custom_button,
            theme_button,
            animated_button,
        );

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

        if disabled_button.is_clicked(&rl) {
            println!("Disabled (Enabled) button clicked!");
        }

        if theme_button.is_clicked(&rl) {
            is_dark_theme = !is_dark_theme;
            if is_dark_theme {
                let new_style = Style::dark_theme();
                primary_button.style = new_style.clone();
                secondary_button.style = new_style.clone();
                success_button.style = new_style.clone();
                danger_button.style = new_style.clone();
                text_field.style = new_style.clone();
                checkbox1.style = new_style.clone();
                dropdown1.style = new_style.clone();
                // Keep custom_button and animated_button as is
                theme_button.style = new_style.clone();
            } else {
                primary_button.style = presets::button_primary();
                secondary_button.style = presets::button_secondary();
                success_button.style = presets::button_success();
                danger_button.style = presets::button_danger();
                text_field.style = presets::textfield_default();
                checkbox1.style = presets::checkbox_default();
                dropdown1.style = presets::dropdown_default();
                custom_button.style = custom_style.clone();
                animated_button.style = Style::modern_blue();
                theme_button.style = presets::button_secondary();
            }
        }

        if animated_button.is_clicked(&rl) {
            println!("Animated button clicked!");
        }

        // Handle checkbox toggles
        if checkbox1.is_checked {
            println!("Feature 1 enabled: {}", checkbox1.is_checked);
        }

        // Feature 2 controls the disabled button
        if checkbox2.is_checked {
            disabled_button.enabled = true;
            disabled_button.style = Style::modern_blue();
            disabled_button.label = "Enabled".to_string();
        } else {
            disabled_button.enabled = false;
            disabled_button.style = Style::default();
            disabled_button.label = "Disabled".to_string();
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

        // Draw everything to the render texture
        {
            let mut d = rl.begin_texture_mode(&thread, &mut render_texture);
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
            raylib_interactive::draw_all!(
                &mut d,
                primary_button,
                secondary_button,
                success_button,
                danger_button,
                disabled_button,
                text_field,
                password_field,
                checkbox1,
                checkbox2,
                dropdown1,
                dropdown2,
                custom_button,
                theme_button,
                animated_button,
            );

            // Draw style presets info
            let preset_y = 650;
            d.draw_text("Style Presets: default, modern_blue, dark_theme, minimal", 50, preset_y, 14, title_color);
            d.draw_text("Button Presets: primary, secondary, success, danger", 50, preset_y + 20, 14, title_color);
        }

        // Now draw the render texture to the window, centered and scaled
        let screen_width = rl.get_screen_width();
        let screen_height = rl.get_screen_height();
        let scale_x = screen_width as f32 / VIRTUAL_WIDTH as f32;
        let scale_y = screen_height as f32 / VIRTUAL_HEIGHT as f32;
        let scale = scale_x.min(scale_y);
        let draw_width = (VIRTUAL_WIDTH as f32 * scale).round() as i32;
        let draw_height = (VIRTUAL_HEIGHT as f32 * scale).round() as i32;
        let offset_x = (screen_width - draw_width) / 2;
        let offset_y = (screen_height - draw_height) / 2;

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture_pro(
            &render_texture,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: VIRTUAL_WIDTH as f32,
                height: -VIRTUAL_HEIGHT as f32, // Flip vertically
            },
            Rectangle {
                x: offset_x as f32,
                y: offset_y as f32,
                width: draw_width as f32,
                height: draw_height as f32,
            },
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );
    }
} 