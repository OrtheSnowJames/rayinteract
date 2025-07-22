use raylib::prelude::*;

/// Universal style struct for all UI components
#[derive(Clone, Debug)]
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

impl Default for Style {
    fn default() -> Self {
        Self {
            // Background colors
            background_color: Color::LIGHTGRAY,
            hover_color: Color::new(200, 200, 200, 255),
            pressed_color: Color::DARKGRAY,
            active_color: Color::new(100, 150, 255, 255),
            disabled_color: Color::new(200, 200, 200, 128),
            
            // Border colors
            border_color: Color::BLACK,
            border_color_hover: Color::new(100, 100, 100, 255),
            border_color_pressed: Color::new(50, 50, 50, 255),
            border_color_active: Color::new(50, 100, 200, 255),
            
            // Text colors
            text_color: Color::BLACK,
            text_color_hover: Color::BLACK,
            text_color_pressed: Color::WHITE,
            text_color_disabled: Color::new(128, 128, 128, 255),
            
            // Special colors
            check_color: Color::GREEN,
            placeholder_color: Color::new(128, 128, 128, 255),
            
            // Typography
            font_size: 20,
            
            // Layout
            padding: 5.0,
            corner_radius: 5.0,
            border_thickness: 2.0,
        }
    }
}

impl Style {
    /// Create a new style with custom colors
    pub fn new(
        background_color: Color,
        hover_color: Color,
        pressed_color: Color,
        border_color: Color,
        text_color: Color,
    ) -> Self {
        Self {
            background_color,
            hover_color,
            pressed_color,
            active_color: Color::new(100, 150, 255, 255),
            disabled_color: Color::new(200, 200, 200, 128),
            border_color,
            border_color_hover: Color::new(100, 100, 100, 255),
            border_color_pressed: Color::new(50, 50, 50, 255),
            border_color_active: Color::new(50, 100, 200, 255),
            text_color,
            text_color_hover: text_color,
            text_color_pressed: Color::WHITE,
            text_color_disabled: Color::new(128, 128, 128, 255),
            check_color: Color::GREEN,
            placeholder_color: Color::new(128, 128, 128, 255),
            font_size: 20,
            padding: 5.0,
            corner_radius: 5.0,
            border_thickness: 2.0,
        }
    }

    /// Create a modern style with blue accent
    pub fn modern_blue() -> Self {
        Self {
            background_color: Color::WHITE,
            hover_color: Color::new(240, 248, 255, 255), // AliceBlue
            pressed_color: Color::new(70, 130, 180, 255), // SteelBlue
            active_color: Color::new(100, 150, 255, 255),
            disabled_color: Color::new(245, 245, 245, 255),
            border_color: Color::new(200, 200, 200, 255),
            border_color_hover: Color::new(100, 150, 255, 255),
            border_color_pressed: Color::new(70, 130, 180, 255),
            border_color_active: Color::new(100, 150, 255, 255),
            text_color: Color::new(50, 50, 50, 255),
            text_color_hover: Color::new(50, 50, 50, 255),
            text_color_pressed: Color::WHITE,
            text_color_disabled: Color::new(150, 150, 150, 255),
            check_color: Color::new(100, 150, 255, 255),
            placeholder_color: Color::new(150, 150, 150, 255),
            font_size: 16,
            padding: 8.0,
            corner_radius: 6.0,
            border_thickness: 1.5,
        }
    }

    /// Create a dark theme style
    pub fn dark_theme() -> Self {
        Self {
            background_color: Color::new(40, 40, 40, 255),
            hover_color: Color::new(60, 60, 60, 255),
            pressed_color: Color::new(80, 80, 80, 255),
            active_color: Color::new(100, 150, 255, 255),
            disabled_color: Color::new(30, 30, 30, 128),
            border_color: Color::new(80, 80, 80, 255),
            border_color_hover: Color::new(100, 150, 255, 255),
            border_color_pressed: Color::new(120, 170, 255, 255),
            border_color_active: Color::new(100, 150, 255, 255),
            text_color: Color::WHITE,
            text_color_hover: Color::WHITE,
            text_color_pressed: Color::WHITE,
            text_color_disabled: Color::new(100, 100, 100, 255),
            check_color: Color::new(100, 150, 255, 255),
            placeholder_color: Color::new(100, 100, 100, 255),
            font_size: 16,
            padding: 8.0,
            corner_radius: 6.0,
            border_thickness: 1.5,
        }
    }

    /// Create a minimal style
    pub fn minimal() -> Self {
        Self {
            background_color: Color::WHITE,
            hover_color: Color::new(250, 250, 250, 255),
            pressed_color: Color::new(240, 240, 240, 255),
            active_color: Color::new(245, 245, 245, 255),
            disabled_color: Color::new(248, 248, 248, 255),
            border_color: Color::new(220, 220, 220, 255),
            border_color_hover: Color::new(200, 200, 200, 255),
            border_color_pressed: Color::new(180, 180, 180, 255),
            border_color_active: Color::new(100, 150, 255, 255),
            text_color: Color::new(50, 50, 50, 255),
            text_color_hover: Color::new(50, 50, 50, 255),
            text_color_pressed: Color::new(50, 50, 50, 255),
            text_color_disabled: Color::new(150, 150, 150, 255),
            check_color: Color::new(100, 150, 255, 255),
            placeholder_color: Color::new(150, 150, 150, 255),
            font_size: 14,
            padding: 6.0,
            corner_radius: 4.0,
            border_thickness: 1.0,
        }
    }

    /// Builder method to set background colors
    pub fn with_background_colors(mut self, background: Color, hover: Color, pressed: Color) -> Self {
        self.background_color = background;
        self.hover_color = hover;
        self.pressed_color = pressed;
        self
    }

    /// Builder method to set border colors
    pub fn with_border_colors(mut self, border: Color, hover: Color, pressed: Color) -> Self {
        self.border_color = border;
        self.border_color_hover = hover;
        self.border_color_pressed = pressed;
        self
    }

    /// Builder method to set text colors
    pub fn with_text_colors(mut self, text: Color, hover: Color, pressed: Color) -> Self {
        self.text_color = text;
        self.text_color_hover = hover;
        self.text_color_pressed = pressed;
        self
    }

    /// Builder method to set typography
    pub fn with_typography(mut self, font_size: i32) -> Self {
        self.font_size = font_size;
        self
    }

    /// Builder method to set layout properties
    pub fn with_layout(mut self, padding: f32, corner_radius: f32, border_thickness: f32) -> Self {
        self.padding = padding;
        self.corner_radius = corner_radius;
        self.border_thickness = border_thickness;
        self
    }
}

/// Predefined style presets for quick use
pub mod presets {
    use super::*;

    /// Default button style
    pub fn button_default() -> Style {
        Style::default()
    }

    /// Primary button style
    pub fn button_primary() -> Style {
        Style::new(
            Color::new(100, 150, 255, 255),
            Color::new(120, 170, 255, 255),
            Color::new(80, 130, 235, 255),
            Color::new(80, 130, 235, 255),
            Color::WHITE,
        )
    }

    /// Secondary button style
    pub fn button_secondary() -> Style {
        Style::new(
            Color::new(240, 240, 240, 255),
            Color::new(220, 220, 220, 255),
            Color::new(200, 200, 200, 255),
            Color::new(180, 180, 180, 255),
            Color::new(50, 50, 50, 255),
        )
    }

    /// Success button style
    pub fn button_success() -> Style {
        Style::new(
            Color::new(40, 167, 69, 255),
            Color::new(60, 187, 89, 255),
            Color::new(20, 147, 49, 255),
            Color::new(20, 147, 49, 255),
            Color::WHITE,
        )
    }

    /// Danger button style
    pub fn button_danger() -> Style {
        Style::new(
            Color::new(220, 53, 69, 255),
            Color::new(240, 73, 89, 255),
            Color::new(200, 33, 49, 255),
            Color::new(200, 33, 49, 255),
            Color::WHITE,
        )
    }

    /// Default text field style
    pub fn textfield_default() -> Style {
        Style::minimal().with_border_colors(
            Color::new(200, 200, 200, 255),
            Color::new(100, 150, 255, 255),
            Color::new(100, 150, 255, 255),
        )
    }

    /// Default checkbox style
    pub fn checkbox_default() -> Style {
        Style::default().with_layout(4.0, 3.0, 2.0)
    }

    /// Default dropdown style
    pub fn dropdown_default() -> Style {
        Style::default().with_layout(5.0, 4.0, 1.5)
    }
} 