# blitter

[![Current Crates.io Version](https://img.shields.io/crates/v/blitter.svg)](https://crates.io/crates/blitter)
[![Downloads badge](https://img.shields.io/crates/d/blitter.svg)](https://crates.io/crates/blitter)

This library performs various blitting and drawing operations on a raw 32 bits framebuffer, whatever the encoding.
Early development.

New in 0.3.0 :
- emergency patch : BlittingBeyondBoundaries checks fixed
- PNG feature added (WIP)

Example:
```
// Framebuffer initialization
let mut pixels: Vec<u32> = vec!(0; WIDTH * HEIGHT);
let mut fb = Framebuffer {width: WIDTH, height: HEIGHT, pixels: &mut pixels};

// User bitmaps initialization
let mut bitmaps = Vec::new();
bitmaps.push(Bitmap {w: 10, h: 10, x: 0, y: 0, pixels: &image::PIXELS});

while *display loop with some display library* {
    blitter_test(&mut fb, &mut bitmaps);
    *your display lib display update function with buffer &fb.pixels*
}

// For testing : moves a 10x10 square and prints a 4x4 pixel at the center of the screen
fn blitter_test(mut fb: &mut Framebuffer, bitmaps: &mut Vec<Bitmap>) {
    fb.clear_area(640, 10, 0, 0, 0).unwrap();
    bitmaps[0].blit(&mut fb).unwrap();   //copies a bitmap to the framebuffer
    if bitmaps[0].x < WIDTH - 10 { bitmaps[0].x = bitmaps[0].x+3; } else { fb.clear(0); }
    fb.draw_fatpixel(320,240,4,0xffffffff).unwrap();
}
```

You can also view and run a (very basic) example using the [minifb library](https://crates.io/crates/minifb) in the 'examples' directory:
```
cargo run --example minifb --features="png-decode"
```
![Screenshot](resources/screenshot.png)

License: GPL-3.0
