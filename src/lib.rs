#![no_std]
#![warn(missing_docs)]

/*! A set of size and color agnostic demos for embedded_graphics, centered around the [`DrawInto`]
 * trait, which can "draw into" any arbitrary rectangle. Since this trait is dyn-compatible, more things
 * like storing demos in a tree or vector are possible
 */

use embedded_graphics::{prelude::*, primitives::Rectangle};

/// The demos module, containing things like content splitting, and bad apple
pub mod demos;

/// A dyn-compatible drawing trait for screen objects that can be resized to draw "into"
/// any rectangle
pub trait DrawInto<D: DrawTarget> {
    /// Draw into the rectangle specified
    fn draw_into(&self, target: &mut D, r: Rectangle) -> Result<(), D::Error>;
}
