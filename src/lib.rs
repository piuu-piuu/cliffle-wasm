// src/lib.rs
#![no_std]

use core::sync::atomic::{AtomicU32, Ordering};

static FRAME: AtomicU32 = AtomicU32::new(0);

#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern fn go() {
    // This is called from JavaScript, and should *only* be
    // called from JavaScript. If you maintain that condition,
    // then we know that the &mut we're about to produce is
    // unique, and therefore safe.
    render_frame_safe(&mut BUFFER)
}


// We split this out so that we can escape 'unsafe' as quickly as possible.
// fn render_frame_safe(buffer: &mut [u32; WIDTH * HEIGHT]) {
//     for pixel in buffer.iter_mut() {
//         *pixel = 0xFF_FF_00_FF;
//     }
// }
// Animated:
fn render_frame_safe(buffer: &mut [u32; WIDTH * HEIGHT]) {
    // animation counter
    let f = FRAME.fetch_add(1, Ordering::Relaxed);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
    //         buffer[y * WIDTH + x] = (x ^ y) as u32 | 0xFF_00_00_00;
    buffer[y * WIDTH + x] = f.wrapping_add((x ^ y) as u32) | 0xFF_00_00_00;
        }
    }
}

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

#[no_mangle]
static mut BUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];