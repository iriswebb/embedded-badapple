extern crate alloc;

use crate::DrawInto;
use alloc::{boxed::Box, vec::Vec};
use core::num::NonZeroU16;
use embedded_graphics::{prelude::*, primitives::Rectangle};

/// A split between different demos at a certain ratio
pub struct Split<D: DrawTarget> {
    /// Is the split horizontal?
    pub is_horiz: bool,
    /// A Vector of the demos to split
    pub widgets: Vec<(NonZeroU16, Box<dyn DrawInto<D>>)>,
}

impl<D: DrawTarget> DrawInto<D> for Split<D> {
    fn draw_into(&self, target: &mut D, r: Rectangle) -> Result<(), D::Error> {
        let Rectangle {
            top_left: Point { x, y },
            size: Size {
                width: w,
                height: h,
            },
        } = r;

        let sum: u32 = self.widgets.iter().map(|&(x, _)| x.get() as u32).sum();

        let mut offset = if self.is_horiz { x } else { y } as f32;

        for (r, wid) in self.widgets.iter() {
            let proportional = (r.get() as f32) / (sum as f32);
            if self.is_horiz {
                let total: f32 = proportional * w as f32;
                wid.draw_into(
                    target,
                    Rectangle::new(Point::new(offset as i32, y), Size::new(total as u32, h)),
                )?;
                offset += total;
            } else {
                let total: f32 = proportional * h as f32;
                wid.draw_into(
                    target,
                    Rectangle::new(Point::new(x, offset as i32), Size::new(w, total as u32)),
                )?;
                offset += total;
            }
        }

        Ok(())
    }
}
