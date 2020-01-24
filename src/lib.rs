/// This structure's goal is to persist user bitmaps sizes, coordinates, and store a pointer to pixel data
pub struct Bitmap<'a> {
    /// Bitmap width
    pub w: usize,
    /// Bitmap height
    pub h: usize,
    /// Bitmap horizontal position
    pub x: usize,
    /// Bitmap vertical position
    pub y: usize,
    /// Pixel data
    pub pixels: &'a Vec<u32>
}

/// The framebuffer struct contains the buffer's width, height, and a pointer to its pixel data
pub struct Framebuffer<'a> {
    pub width: usize,
    pub height: usize,
    pub pixels: &'a mut Vec<u32>
}

/// Copies a bitmap to the framebuffer
pub fn blit(bitmap: &Bitmap, fb: &mut Framebuffer) {
    for inc_y in 0..bitmap.h {
        let x_offset: usize = inc_y*fb.width;
        let y_offset: usize = bitmap.y*fb.width;
        for inc_x in 0..bitmap.w {
            fb.pixels[inc_x + x_offset + bitmap.x + y_offset] = bitmap.pixels[inc_x];
        }
    }
}

/// Partial clear a the framebuffer
pub fn clear_area(fb: &mut Framebuffer, w: usize, h: usize, x: usize, y: usize, clear_color: u32) {
    for inc_y in 0..h {
        let x_offset: usize = inc_y*fb.width;
        let y_offset: usize = y*fb.width;
        for inc_x in 0..w {
            fb.pixels[inc_x + x_offset + x + y_offset] = clear_color;
        }
    }
}

/// Complete clear of the framebuffer
pub fn clear_buffer(fb: &mut Framebuffer, clear_color: u32) {
    for inc_x in 0..fb.width*fb.height {
        fb.pixels[inc_x] = clear_color;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
