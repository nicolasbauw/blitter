//#![windows_subsystem = "windows"]
use blitter::*;
use minifb::{Key, Scale, Window, WindowOptions};

const WIDTH: usize = 320;
const HEIGHT: usize = 256;
const TEXT: &'static str = "Blit,Blit with color mask,Automatic cropping,Blit with bits mask,Blit of a partial bitmap";

fn main() {
    // Framebuffer initialization
    let mut pixels: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut fb = Framebuffer {
        width: WIDTH,
        height: HEIGHT,
        pixels: &mut pixels,
    };

    // Font generation
    let f = include_bytes!("../resources/FONT2_8X8.BIN");
    // To store the 96 * 8 * 8 pixels as 32 bits pixel data
    let mut font = vec![0; 6144];
    // Pixel counter for destination bitmap
    let mut c = 0;
    for byte in 0..8 {
        // index of first char byte
        for char_index in 0..96 {
            // for each bit
            for bit in 0..8 {
                if f[char_index * 8 + byte] << bit & 0x80u8 == 0x80u8 {
                    font[c] = 0xffffffffu32;
                } else {
                    font[c] = 0u32;
                }
                c += 1;
            }
        }
    }

    // Bitmap generation
    let path = "resources/littledragonG.png";
    let png = from_png_file(&path, PixelFormat::Zrgb).unwrap();
    // Bitmaps structs stored in a Vec, could be a hashmap, to give more easily ownership to other functions
    let mut bitmaps = Vec::new();
    bitmaps.push(Bitmap {
        w: 768,
        h: 8,
        x: 0,
        y: 0,
        pixels: &font,
    }); //bitmaps[0] is the font
    bitmaps.push(Bitmap{w: png.0, h: png.1, x: 0, y: 0, pixels: &png.2});

    let mut window = Window::new(
        "Blitter demo",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let text:Vec<&str> = TEXT.split(",").collect();
    let mut bitmask = Vec::new();
    for _i in 0..6400 {
        bitmask.push(true);
        bitmask.push(false);
    }
    let cm = Mask::Color(0x00ff00);
    let bm = Mask::Bits(&bitmask);

    bitmaps[0].x = 0;
    bitmaps[0].y = 85;
    bitmaps[1].x = 0;
    bitmaps[1].y = 0;
    bitmaps[1].blit(&mut fb);
    draw_text(text[0], &mut fb, &mut bitmaps[0]);

    bitmaps[0].x = 160;
    bitmaps[0].y = 85;
    bitmaps[1].x = 240;
    bitmaps[1].y = 0;
    bitmaps[1].blit_mask(&mut fb, cm);
    draw_text(text[1], &mut fb, &mut bitmaps[0]);

    bitmaps[0].x = 160;
    bitmaps[0].y = 166;
    bitmaps[1].x = 240;
    bitmaps[1].y = 176;
    bitmaps[1].blit_mask(&mut fb, bm);
    draw_text(text[3], &mut fb, &mut bitmaps[0]);

    bitmaps[0].x = 85;
    bitmaps[0].y = 125;
    bitmaps[1].x = 35;
    bitmaps[1].y = 125;
    bitmaps[1].blit_part(&mut fb, 0, 40, 40);
    draw_text(text[4], &mut fb, &mut bitmaps[0]);

    bitmaps[0].x = 0;
    bitmaps[0].y = 196;
    bitmaps[1].x = -20;
    bitmaps[1].y = 206;
    bitmaps[1].blit(&mut fb);
    draw_text(text[2], &mut fb, &mut bitmaps[0]);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&fb.pixels, WIDTH, HEIGHT)
            .unwrap();
    }
}

// Text drawing
fn draw_text(text: &str, mut fb: &mut Framebuffer, font: &mut Bitmap) {
    let text = text.as_bytes();
    for i in 0..text.len() {
        font.blit_part(&mut fb, 8 * (text[i] as usize - 32), 8, 8);
        font.x += 8;
    }
}