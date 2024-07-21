//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;
pub use self::name::color_name;

pub mod kinds {
    /// --snip--
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    /// --snip--
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to crate
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}

pub mod name {
    /// --snip--
    use crate::kinds::*;

    /// Module for names of colors
    pub fn color_name() {
        // --snip--
        unimplemented!();
    }
}
