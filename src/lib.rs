pub struct BmpBlt {
    pub screen_width: usize,
    pub w: usize,
    pub h: usize,
    pub x: usize,
    pub y: usize
}

pub fn blit(source: &Vec<u32>, destination: &mut Vec<u32>, bmp: &BmpBlt) {
    for inc_y in 0..bmp.h {
        let x_offset: usize = inc_y*bmp.screen_width;
        let y_offset: usize = bmp.y*bmp.screen_width;
        for inc_x in 0..bmp.w {
            destination[inc_x + x_offset + bmp.x + y_offset] = source[inc_x];
        }
    }
}

pub fn clear_area(buffer: &mut Vec<u32>, screen_width: usize, w: usize, h: usize, x: usize, y: usize, clear_color: u32) {
    for inc_y in 0..h {
        let x_offset: usize = inc_y*screen_width;
        let y_offset: usize = y*screen_width;
        for inc_x in 0..w {
            buffer[inc_x + x_offset + x + y_offset] = clear_color;
        }
    }
}

pub fn clear_buffer(buffer: &mut Vec<u32>, screen_width: usize, screen_height: usize, clear_color: u32) {
    for inc_x in 0..screen_width*screen_height {
        buffer[inc_x] = clear_color;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
