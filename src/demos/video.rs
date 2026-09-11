use crate::DrawInto;
use embedded_graphics::image::GetPixel;
use embedded_graphics::iterator::raw::RawDataSlice;
use embedded_graphics::pixelcolor::raw::BigEndian;
use embedded_graphics::{image::ImageRaw, prelude::*, primitives::Rectangle};

/// `DrawImage<T>` from embedded_graphics is a valid demo, since it can be drawn into anything
impl<C: PixelColor, D: DrawTarget<Color = C>, T: ImageDrawable<Color = C>> DrawInto<D> for T {
    fn draw_into(&self, target: &mut D, r: Rectangle) -> Result<(), D::Error> {
        self.draw_sub_image(target, &r)
    }
}

/// A scale-able image, preforming "dumb" subsampling scaling
pub struct ImageScaleable<'a, C: PixelColor + From<<C as PixelColor>::Raw>>(ImageRaw<'a, C>);

impl<'a, C: PixelColor + From<<C as PixelColor>::Raw>, D: DrawTarget<Color = C>> DrawInto<D>
    for ImageScaleable<'a, C>
where
    RawDataSlice<'a, C::Raw, BigEndian>: IntoIterator<Item = C::Raw>,
{
    fn draw_into(&self, target: &mut D, r: Rectangle) -> Result<(), <D as DrawTarget>::Error> {
        r.points()
            .map(|p| Pixel(Point::new(0, 0), self.0.pixel(p).expect("Bounds error")))
            .draw(target)
    }
}
