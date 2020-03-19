//! This library performs various blitting and drawing operations on a raw 32 bits framebuffer, whatever the encoding.
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
#[cfg(feature = "png-decode")]
use {png::DecodingError, std::fs::File};

/// Output format of png decoding function
#[cfg(feature = "png-decode")]
pub enum PixelFormat {
    /// 0RGB
    Zrgb,
    /// RGBA, STRIP_ALPHA
    Rgba,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BlitError {
    // Index out of bounds
    BlittingBeyondBoundaries,
}

impl fmt::Display for BlitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("tzfile error: ")?;
        f.write_str(match self {
            BlitError::BlittingBeyondBoundaries => "You are blitting outside the framebuffer !",
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
    pub x: isize,
    /// Bitmap vertical position
    pub y: isize,
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

struct ClippedCoords {
    // Non negative (unsigned) x coordinate
    ux: usize,
    // Non negative (unsigned) y coordinate
    uy: usize,
    // End of blit, x axis
    x_end: usize,
    // End of blit, y axis
    y_end: usize,
    // Pixel counter / start offset
    c: usize,
    // Pixel skip of source bitmap
    src_pixel_skip: usize,
}

impl Bitmap<'_> {
    /// Copies a bitmap to the framebuffer
    pub fn blit(&self, fb: &mut Framebuffer) {
        let mut cr = match self.compute_clipping(fb) {
            Some(c) => c,
            None => return
        };
        for inc_y in 0..cr.y_end {
            let x_offset: usize = inc_y * fb.width;
            let y_offset: usize = cr.uy * fb.width;
            for inc_x in 0..cr.x_end {
                fb.pixels[inc_x + x_offset + cr.ux + y_offset] = self.pixels[cr.c];
                cr.c += 1;
            }
            cr.c += cr.src_pixel_skip;
        }
    }

    /// Copies a portion of a bitmap to the framebuffer
    pub fn blit_part(&self, fb: &mut Framebuffer, start_offset: usize, w: usize, h: usize) {
        let mut c = 0 + start_offset;
        // Temporary pixel buffer
        let mut t_pixels = vec![0; w * h];
        for inc_y in 0..h {
            for inc_x in 0..w {
                t_pixels[inc_x + inc_y * w ] = self.pixels[c];
                c += 1;
            }
            c += self.w - w;
        }
        // Temporary Bitmap; this way we can use the generic blit function
        let tx = self.x;
        let ty = self.y;
        let tw = w;
        let th = h;
        let t = Bitmap { x: tx, y: ty, w: tw, h: th, pixels: &t_pixels};
        t.blit(fb);
    }

    /// Copies a Bitmap to the framebuffer, applying a color mask (color acting as transparent in case of non alpha framebuffers)
    pub fn blit_cmask(&self, fb: &mut Framebuffer, mask: u32) -> Result<(), BlitError> {
        let ux = if self.x > 0 { self.x as usize } else { 0 };
        let uy = if self.y > 0 { self.y as usize } else { 0 };
        if (self.pixels.len() + (ux + self.w) * (uy + self.h) - self.w * self.h) > fb.pixels.len() {
            return Err(BlitError::BlittingBeyondBoundaries);
        };
        let mut c = 0;
        for inc_y in 0..self.h {
            let x_offset: usize = inc_y * fb.width;
            let y_offset: usize = uy * fb.width;
            for inc_x in 0..self.w {
                if self.pixels[inc_x] != mask {
                    fb.pixels[inc_x + x_offset + ux + y_offset] = self.pixels[c];
                    c += 1;
                };
            }
        }
        Ok(())
    }

    /// Copies a Bitmap to the framebuffer, applying a bits mask (logical AND)
    pub fn blit_lmask(&self, fb: &mut Framebuffer, mask: &Vec<bool>) -> Result<(), BlitError> {
        let ux = if self.x > 0 { self.x as usize } else { 0 };
        let uy = if self.y > 0 { self.y as usize } else { 0 };
        if (self.pixels.len() + (ux + self.w) * (uy + self.h) - self.w * self.h) > fb.pixels.len() {
            return Err(BlitError::BlittingBeyondBoundaries);
        };
        let mut c = 0;
        for inc_y in 0..self.h {
            let x_offset: usize = inc_y * fb.width;
            let y_offset: usize = uy * fb.width;
            for inc_x in 0..self.w {
                if mask[c] {
                    fb.pixels[inc_x + x_offset + ux + y_offset] = self.pixels[c];
                    c += 1;
                };
            }
        }
        Ok(())
    }

    fn compute_clipping(&self, fb: &Framebuffer) -> Option<ClippedCoords> {
        // Are x or y negative values ? compute cropped pixels size and convert x and y to unsigned values
        let ux = if self.x > 0 { self.x as usize } else { 0 };
        let uy = if self.y > 0 { self.y as usize } else { 0 };
        let cropped_x = self.x.abs() as usize;
        let cropped_y = self.y.abs() as usize;
        // Need to crop the top of the bitmap
        let r = if ux + self.w <= fb.width && uy + self.h < fb.height && self.x >= 0 && self.y < 0 && (self.y + self.h as isize) > 0 {
            //println!("Cropping top");
            Some(ClippedCoords {
                x_end: self.w,
                y_end: self.h - cropped_y,
                src_pixel_skip: 0,
                c: cropped_y * self.w,
                ux: ux,
                uy: uy,
            })
        }
        // Need to crop the top left of the bitmap
        else if self.x < 0 && self.y < 0 && self.x + self.w as isize > 0 && self.y + self.h as isize > 0 {
            //println!("Cropping top left");
            Some(ClippedCoords {
                x_end: self.w - cropped_x,
                y_end: self.h - cropped_y,
                src_pixel_skip: cropped_x,
                c: cropped_y * self.w + cropped_x,
                ux: ux,
                uy: uy,
            })
        }
        // Need to crop the top right of the bitmap
        else if ux + self.w > fb.width && ux < fb.width && self.y < 0 && self.y + self.h as isize > 0 {
            //println!("Cropping top right");
            Some(ClippedCoords {
                x_end: fb.width - ux,
                y_end: self.h - cropped_y,
                src_pixel_skip: self.w - (fb.width - ux),
                c: cropped_y * self.w,
                ux: ux,
                uy: uy,
            })
        }
        // Need to crop the bottom left of the bitmap
        else if uy + self.h > fb.height && self.x < 0 && self.x + self.w as isize > 0 && uy < fb.height {
            //println!("Cropping bottom left");
            Some(ClippedCoords {
                x_end: self.w - cropped_x,
                y_end: fb.height - uy,
                src_pixel_skip: cropped_x,
                c: cropped_x,
                ux: ux,
                uy: uy,
            })
        }
        // Need to crop the bottom right of the bitmap
        else if ux + self.w > fb.width && uy + self.h > fb.height && ux < fb.width && uy < fb.height {
            //println!("Cropping bottom right");
            Some(ClippedCoords {
                x_end: fb.width - ux,
                y_end: fb.height - uy,
                src_pixel_skip: self.w - (fb.width - ux),
                c: 0,
                ux: ux,
                uy: uy,
            })
        }
        // Need to crop the bottom of the bitmap
        else if ux + self.w < fb.width && self.x + self.w as isize > 0 && uy + self.h > fb.height && uy < fb.height {
            //println!("Cropping bottom");
            Some(ClippedCoords {
                x_end: self.w,
                y_end: fb.height - uy,
                src_pixel_skip: 0,
                c: 0,
                ux: ux,
                uy: uy,
            })
        }
        // Need to crop the left of the bitmap
        else if self.x < 0 && self.x + self.w as isize > 0 && self.y > 0 && uy < fb.height {
            //println!("Cropping left");
            Some(ClippedCoords {
                x_end: self.w - cropped_x,
                y_end: self.h,
                src_pixel_skip: cropped_x,
                c: cropped_x,
                ux: ux,
                uy: uy,
            })
        }
        // Need to crop the right of the bitmap
        else if ux + self.w > fb.width && self.y >= 0 && ux <= fb.width && uy + self.h < fb.height {
            //println!("Cropping right");
            Some(ClippedCoords {
                x_end: fb.width - ux,
                y_end: self.h,
                src_pixel_skip: self.w - (fb.width - ux),
                c: 0,
                ux: ux,
                uy: uy,
            })
        }
        // Blitting outside the screen -> no need to blit anything
        else if ux > fb.width || uy > fb.height || (self.x + self.w as isize) < 0 || (self.y + self.h as isize) < 0 {
            //println!("Outside framebuffer");
            None
        }
        // No need to crop      self.x + self.w <= fb.width && self.y + self.h <= fb.height
        else {
            //println!("No cropping");
            Some(ClippedCoords {
                x_end: self.w,
                y_end: self.h,
                src_pixel_skip: 0,
                c: 0,
                ux: ux,
                uy: uy,
            })
        };
        r
    }
}

impl Framebuffer<'_> {
    /// Partial clear of the framebuffer
    pub fn clear_area(
        &mut self,
        w: usize,
        h: usize,
        x: usize,
        y: usize,
        clear_color: u32,
    ) -> Result<(), BlitError> {
        if ((x + w) * (y + h)) > self.pixels.len() {
            return Err(BlitError::BlittingBeyondBoundaries);
        };
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
        if x > self.width || y > self.height {
            return Err(BlitError::BlittingBeyondBoundaries);
        };
        self.pixels[x + y * self.width] = color;
        Ok(())
    }

    /// Drawing a fat pixel
    pub fn draw_fatpixel(
        &mut self,
        x: usize,
        y: usize,
        size: usize,
        color: u32,
    ) -> Result<(), BlitError> {
        if x > self.width - size || y > self.height - size {
            return Err(BlitError::BlittingBeyondBoundaries);
        };
        self.clear_area(size, size, x, y, color)?;
        Ok(())
    }
}

#[cfg(feature = "png-decode")]
/// Creates a tuple containing width, height, and pixel data from a PNG file
pub fn from_png_file(
    pngfile: &str,
    pxfmt: PixelFormat,
) -> Result<(usize, usize, Vec<u32>), DecodingError> {
    let shift: u32 = match pxfmt {
        PixelFormat::Zrgb => 0,
        PixelFormat::Rgba => 8,
    };
    // The default output transformation is `Transformations::EXPAND | Transformations::STRIP_ALPHA`.
    let decoder = png::Decoder::new(File::open(&pngfile)?);
    let (info, mut reader) = decoder.read_info()?;
    // Allocate the output buffer.
    let mut buf = vec![0; info.buffer_size()];
    // Read the next frame. Currently this function should only called once.
    reader.next_frame(&mut buf)?;
    // convert buffer to u32
    let u32_buffer: Vec<u32> = buf
        .chunks(3)
        .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
        .map(|x| x << shift)
        .collect();

    Ok((info.width as usize, info.height as usize, u32_buffer))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
