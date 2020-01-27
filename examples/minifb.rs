#![windows_subsystem = "windows"]
use minifb::{Key, Window, WindowOptions};
use blitter::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    // Framebuffer initialization
    let mut pixels: Vec<u32> = vec!(0; WIDTH * HEIGHT);
    let mut fb = Framebuffer {width: WIDTH, height: HEIGHT, pixels: &mut pixels};
    
    // User bitmaps initialization
    let image:  Vec<u32> =  { vec![0xffffffff; 100] };
    let mut bitmaps = Vec::new();
    bitmaps.push(Bitmap {w: 10, h: 10, x: 0, y: 0, pixels: &image});

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        blitter_test(&mut fb, &mut bitmaps);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&fb.pixels, WIDTH, HEIGHT)
            .unwrap();
    }
}

// For testing : moves a 10x10 square and prints a 4x4 pixel at the center of the screen
fn blitter_test(mut fb: &mut Framebuffer, bitmaps: &mut Vec<Bitmap>) {
    fb.clear_area(640, 10, 0, 0, 0);
    bitmaps[0].blit(&mut fb).unwrap();
    if bitmaps[0].x < WIDTH - 10 { bitmaps[0].x = bitmaps[0].x+3; } else { fb.clear(0); }
    fb.draw_fatpixel(320,240,4,0xffffffff);
}
