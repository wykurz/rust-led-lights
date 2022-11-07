use std::ops::Add;
use std::time::{Duration, Instant};

/// My example code uses CPU intensive spin locks in order to work properly all the time.
/// If you experience a flickering of the LEDs when using `thread::sleep()` instead, then
/// please have a look at: <https://github.com/phip1611/ws2818-rgb-led-spi-driver/issues/9>
#[inline(always)]
pub fn sleep_busy_waiting_ms(ms: u64) {
    let target_time = Instant::now().add(Duration::from_millis(ms));
    loop {
        if Instant::now() >= target_time {
            break;
        }
    }
}

/// Returns a pixel with a random color and a minimal
/// brightness. Tries to get real colors instead of white.
pub fn get_random_pixel_val() -> (u8, u8, u8) {
    // RED and GREEN are inverted
    const _HALLOWEEN_COLORS: [(u8, u8, u8); 9] = [
        (255, 255, 255), // white
        (113, 255, 0),   // orange
        (113, 255, 0),   // orange
        (151, 253, 2),   // yellow
        (2, 225, 255),   // light purple
        (2, 225, 255),   // light purple
        (3, 174, 255),   // dark purple
        (3, 174, 255),   // dark purple
        (3, 174, 255),   // dark purple
                         // (255, 0, 0), // green
                         // (0, 255, 0), // red
                         // (0, 0, 255), // blue
    ];

    const THANKSGIVING_COLORS: [(u8, u8, u8); 7] = [
        (47, 156, 47),  // pastel red
        (175, 252, 44), // orange
        (86, 182, 42),  // orange
        (186, 219, 51), // mustard green
        (31, 81, 22),   // brown
        (137, 78, 38),  // dark green
        (157, 130, 54), // light green
    ];

    let i = rand::random::<u8>();
    let i = i % THANKSGIVING_COLORS.len() as u8;

    THANKSGIVING_COLORS[i as usize]
}

pub fn darken_rgb(r: u8, g: u8, b: u8, factor: f32) -> (u8, u8, u8) {
    (
        ((r as f32) * factor) as u8,
        ((g as f32) * factor) as u8,
        ((b as f32) * factor) as u8,
    )
}
