/// This structure's goal is to persist user bitmaps sizes and coordinates
pub struct BmpBlt {
    /// Bitmap width
    pub w: usize,
    /// Bitmap height
    pub h: usize,
    /// Bitmap horizontal position
    pub x: usize,
    /// Bitmap vertical position
    pub y: usize
}

/// The framebuffer struct contains the buffer's width, height, and a pointer to its pixel data
pub struct Framebuffer<'a> {
    pub width: usize,
    pub height: usize,
    pub pixels: &'a mut Vec<u32>
}

/// Copies a bitmap to the framebuffer
pub fn blit(source: &Vec<u32>, destination: &mut Framebuffer, bmp: &BmpBlt) {
    for inc_y in 0..bmp.h {
        let x_offset: usize = inc_y*destination.width;
        let y_offset: usize = bmp.y*destination.width;
        for inc_x in 0..bmp.w {
            destination.pixels[inc_x + x_offset + bmp.x + y_offset] = source[inc_x];
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
