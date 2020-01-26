//! This library performs various blitting and drawing operations on a 32 bits framebuffer.
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
//!     *your display lib display update function with &fb.pixels*
//! }
//! 
//! // For testing : moves a 10x10 square and prints a 4x4 pixel at the center of the screen
//! fn blitter_test(mut fb: &mut Framebuffer, bitmaps: &mut Vec<Bitmap>) {
//!     fb.clear_area(640, 10, 0, 0, 0);
//!     bitmaps[0].blit(&mut fb);   //copies a bitmap to the framebuffer
//!     if bitmaps[0].x < WIDTH - 10 { bitmaps[0].x = bitmaps[0].x+3; } else { fb.clear(0); }
//!     fb.draw_fatpixel(320,240,4,0xffffffff);
//! }

/// This structure contains bitmaps sizes, coordinates, and a pointer to its pixel data
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
    pub fn blit(&self, fb: &mut Framebuffer) {
        for inc_y in 0..self.h {
            let x_offset: usize = inc_y * fb.width;
            let y_offset: usize = self.y * fb.width;
            for inc_x in 0..self.w {
                fb.pixels[inc_x + x_offset + self.x + y_offset] = self.pixels[inc_x];
            }
        }
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
    ) {
        for inc_y in 0..h {
            let x_offset: usize = inc_y * self.width;
            let y_offset: usize = y * self.width;
            for inc_x in 0..w {
                self.pixels[inc_x + x_offset + x + y_offset] = clear_color;
            }
        }
    }

    /// Complete clear of the framebuffer
    pub fn clear(&mut self, clear_color: u32) {
        for inc_x in 0..self.width * self.height {
            self.pixels[inc_x] = clear_color;
        }
    }

    /// Drawing a pixel
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        self.pixels[x + y*self.width] = color;
    }

    /// Drawing a fat pixel
    pub fn draw_fatpixel(
        &mut self,
        x: usize,
        y: usize,
        size: usize,
        color: u32,
    ) {
        self.clear_area(size, size, x, y, color)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
