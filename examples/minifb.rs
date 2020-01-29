//#![windows_subsystem = "windows"]
use minifb::{Key, Window, WindowOptions};
use blitter::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 520;

fn main() {
    // Framebuffer initialization
    let mut pixels: Vec<u32> = vec!(0; WIDTH * HEIGHT);
    let mut fb = Framebuffer {width: WIDTH, height: HEIGHT, pixels: &mut pixels};
    
    // User bitmaps generating / loading
    let image:  Vec<u32> =  { vec![0xffffffff; 100] };
    let image2:  Vec<u32> =  { vec![0x0000ff00; 2500] };
    let path = "resources/test-image.png";
    let png = info_from_png(&path);

    // Bitmaps structs stored in a Vec, could be a hashmap, to give more easily ownership to other functions
    let mut bitmaps = Vec::new();
    bitmaps.push(Bitmap {w: 10, h: 10, x: 0, y: 0, pixels: &image});
    bitmaps.push(Bitmap{w: png.0, h: png.1, x: 0, y: 320, pixels: &png.2});
    bitmaps.push(Bitmap {w: 50, h: 50, x: 590, y: 470, pixels: &image2});

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
        
        // We can easily give bitmaps and framebuffer ownership; of course you can do the way you want
        blitter_test(&mut fb, &mut bitmaps);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&fb.pixels, WIDTH, HEIGHT)
            .unwrap();
    }
}

// For testing : moves a 10x10 square and prints a 4x4 pixel at the center of the screen
fn blitter_test(mut fb: &mut Framebuffer, bitmaps: &mut Vec<Bitmap>) {
    // We clear just the animated part of the screen
    fb.clear_area(640, 10, 0, 0, 0).unwrap();
    bitmaps[0].blit(&mut fb).unwrap();
    // For illustration. It's of course not necessary to copy non-moving or non-changing bitmaps on the framebuffer at each frame.
    bitmaps[1].blit(&mut fb).unwrap();
    bitmaps[2].blit(&mut fb).unwrap();
    if bitmaps[0].x < WIDTH - 10 { bitmaps[0].x = bitmaps[0].x+3; }
    fb.draw_fatpixel(WIDTH/2, HEIGHT/2, 4, 0xffffffff).unwrap();
}
