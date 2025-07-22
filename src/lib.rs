use raylib::prelude::*;

pub mod checkbox;
pub mod dropdown;
pub mod textfield;
pub mod button;
pub mod style;
pub mod tests;

pub const PADDING: f32 = 20.0;

#[macro_export]
macro_rules! update_all {
    ($rl:expr, $mouse:expr, $($element:expr),+ $(,)?) => {
        $( $element.update($mouse, $rl); )+
    };
}

#[macro_export]
macro_rules! draw_all {
    ($rl:expr, $($element:expr),+ $(,)?) => {
        $( $element.draw($rl); )+
    };
}

// Re-export commonly used items
pub use style::{Style, presets};
pub use button::Button;
pub use textfield::TextField;
pub use checkbox::Checkbox;
pub use dropdown::Dropdown;
pub use tests::{TestResults, run_all_tests};