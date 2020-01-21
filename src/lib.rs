pub fn blit(source: &Vec<u32>, destination: &mut Vec<u32>, screen_width: usize, w: usize, h: usize, x: usize, y: usize) {
    for inc_y in 0..h {
        for inc_x in 0..w {
            destination[inc_x + inc_y*screen_width + x + y*screen_width] = source[inc_x];
        }
    }
}

pub fn clear_area(buffer: &mut Vec<u32>, screen_width: usize, w: usize, h: usize, x: usize, y: usize, clear_color: u32) {
    for inc_y in 0..h {
        for inc_x in 0..w {
            buffer[inc_x + inc_y*screen_width + x + y*screen_width] = clear_color;
        }
    }
}

pub fn clear_buffer(buffer: &mut Vec<u32>, screen_width: usize, screen_height: usize, clear_color: u32) {
    for inc_y in 0..screen_height {
        for inc_x in 0..screen_width {
            buffer[inc_x + inc_y*screen_width] = clear_color;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
