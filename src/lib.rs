//! This library performs various blitting and drawing operations on a raw 32 bits framebuffer, whatever the encoding.
//! Early development.
//!
//! Example:
//!```text
//! // Framebuffer initialization
//! let mut pixels: Vec<u32> = vec!(0; WIDTH * HEIGHT);
//! let mut fb = Framebuffer {width: WIDTH, height: HEIGHT, pixels: &mut pixels};
//!
//! // User bitmaps initialization
//! let mut bitmaps = Vec::new();
//! bitmaps.push(Bitmap {w: 10, h: 10, x: 0, y: 0, pixels: &image::PIXELS});
//!
//! while *display loop with some display library* {
//!     blitter_test(&mut fb, &mut bitmaps);
//!     *your display lib display update function with buffer &fb.pixels*
//! }
//!
//! // For testing : moves a 10x10 square and prints a 4x4 pixel at the center of the screen
//! fn blitter_test(mut fb: &mut Framebuffer, bitmaps: &mut Vec<Bitmap>) {
//!     fb.clear_area(640, 10, 0, 0, 0).unwrap();
//!     bitmaps[0].blit(&mut fb).unwrap();   //copies a bitmap to the framebuffer
//!     if bitmaps[0].x < WIDTH - 10 { bitmaps[0].x = bitmaps[0].x+3; } else { fb.clear(0); }
//!     fb.draw_fatpixel(320,240,4,0xffffffff).unwrap();
//! }
//! ```
//!
//! You can also view and run a (very basic) example using the [minifb library](https://crates.io/crates/minifb) in the 'examples' directory:
//! ```text
//! cargo run --example minifb
//! ```

use std::{fmt, result::Result};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BlitError {
    // Index out of bounds
    IndexOutOfBounds,
}

impl fmt::Display for BlitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("tzfile error: ")?;
        f.write_str(match self {
            BlitError::IndexOutOfBounds => "You are blitting outside the framebuffer !",
        })
    }
}

/// This structure stores bitmap's sizes, coordinates, and a pointer to its pixel data
pub struct Bitmap<'a> {
    /// Bitmap width
    pub w: usize,
    /// Bitmap height
    pub h: usize,
    /// Bitmap horizontal position
    pub x: usize,
    /// Bitmap vertical position
    pub y: usize,
    /// 32 bits pixel data
    pub pixels: &'a Vec<u32>,
}

/// The framebuffer struct contains the buffer's width, height, and a pointer to its pixel data
pub struct Framebuffer<'a> {
    /// Framebuffer width
    pub width: usize,
    /// Framebuffer height
    pub height: usize,
    /// 32 bits pixel data
    pub pixels: &'a mut Vec<u32>,
}

impl Bitmap<'_> {
    /// Copies a Bitmap to the framebuffer
    pub fn blit(&self, fb: &mut Framebuffer) -> Result<(), BlitError> {
        if (self.pixels.len()+(self.x+self.w)*(self.y+self.h)) > fb.pixels.len() { return Err(BlitError::IndexOutOfBounds) };
        for inc_y in 0..self.h {
            let x_offset: usize = inc_y * fb.width;
            let y_offset: usize = self.y * fb.width;
            for inc_x in 0..self.w {
                fb.pixels[inc_x + x_offset + self.x + y_offset] = self.pixels[inc_x];
            }
        }
        Ok(())
    }

    /// Copies a Bitmap to the framebuffer, applying a color mask (color acting as transparent in case of non alpha framebuffers)
    pub fn blit_cmask(&self, fb: &mut Framebuffer, mask: u32) -> Result<(), BlitError> {
        if (self.pixels.len()+(self.x+self.w)*(self.y+self.h)) > fb.pixels.len() { return Err(BlitError::IndexOutOfBounds) };
        for inc_y in 0..self.h {
            let x_offset: usize = inc_y * fb.width;
            let y_offset: usize = self.y * fb.width;
            for inc_x in 0..self.w {
                if self.pixels[inc_x] != mask {
                    fb.pixels[inc_x + x_offset + self.x + y_offset] = self.pixels[inc_x]
                };
            }
        }
        Ok(())
    }

    /// Copies a Bitmap to the framebuffer, applying a bits mask (logical AND)
    pub fn blit_lmask(&self, fb: &mut Framebuffer, mask: &Vec<bool>) -> Result<(), BlitError> {
        if (self.pixels.len()+(self.x+self.w)*(self.y+self.h)) > fb.pixels.len() { return Err(BlitError::IndexOutOfBounds) };
        let mut c = 0;
        for inc_y in 0..self.h {
            let x_offset: usize = inc_y * fb.width;
            let y_offset: usize = self.y * fb.width;
            for inc_x in 0..self.w {
                if mask[c] {
                    fb.pixels[inc_x + x_offset + self.x + y_offset] = self.pixels[inc_x]
                };
                c += 1;
            }
        }
        Ok(())
    }
}

impl Framebuffer<'_> {
    /// Partial clear of the framebuffer
    pub fn clear_area(&mut self, w: usize, h: usize, x: usize, y: usize, clear_color: u32) -> Result<(), BlitError> {
        if ((x+w)*(y+h)) > self.pixels.len() { return Err(BlitError::IndexOutOfBounds) };
        for inc_y in 0..h {
            let x_offset: usize = inc_y * self.width;
            let y_offset: usize = y * self.width;
            for inc_x in 0..w {
                self.pixels[inc_x + x_offset + x + y_offset] = clear_color;
            }
        }
        Ok(())
    }

    /// Complete clear of the framebuffer
    pub fn clear(&mut self, clear_color: u32) {
        for inc_x in 0..self.width * self.height {
            self.pixels[inc_x] = clear_color;
        }
    }

    /// Drawing a pixel
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) -> Result<(), BlitError> {
        if x > self.width && y > self.height { return Err(BlitError::IndexOutOfBounds) };
        self.pixels[x + y * self.width] = color;
        Ok(())
    }

    /// Drawing a fat pixel
    pub fn draw_fatpixel(&mut self, x: usize, y: usize, size: usize, color: u32) -> Result<(), BlitError> {
        if x > self.width-size && y > self.height-size { return Err(BlitError::IndexOutOfBounds) };
        self.clear_area(size, size, x, y, color).unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
