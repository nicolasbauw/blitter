pub fn blit(source: &Vec<u32>, destination: &mut Vec<u32>, screen_width: usize, w: usize, h: usize, x: usize, y: usize) {
    for inc_y in 0..h {
        for inc_x in 0..w {
            destination[inc_x + inc_y*screen_width + x + y*screen_width] = source[inc_x];
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
