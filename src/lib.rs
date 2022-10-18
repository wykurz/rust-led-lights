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
    const COLORS: [(u8, u8, u8); 11] = [
        // 28
        // some colors are multiple times listed to increase
        // their probability.
        (255, 255, 255), // white
        (255, 0, 0),     // red
        (255, 0, 0),     // red
        (255, 0, 0),     // red
        // (0, 255, 0),     // green
        // (0, 0, 255),     // blue
        // (13, 255, 248),  // turquoise
        // (13, 255, 248),  // turquoise
        // (13, 255, 248),  // turquoise
        // (255, 168, 0),   // dark orange
        (255, 168, 0), // dark orange
        (255, 168, 0), // dark orange
        (255, 168, 0), // dark orange
        (255, 189, 0), // bright orange
        (255, 189, 0), // bright orange
        (255, 189, 0), // bright orange
        // (255, 255, 0),   // yellow
        // (255, 255, 0),   // yellow
        // (255, 255, 0),   // yellow
        // (234, 10, 142), // Telekom Magenta
        // (234, 10, 142), // Telekom Magenta
        // (234, 10, 142), // Telekom Magenta
        (175, 0, 255), // purple
                       // (0, 150, 255),   // semi light blue
                       // (0, 198, 255),   // very light blue
                       // (0, 198, 255),   // very light blue
                       // (0, 198, 255),   // very light blue
                       // (255, 114, 114), // light red
                       // (255, 114, 114), // light red
                       // (255, 114, 114), // light red
    ];

    let i = rand::random::<u8>();
    let i = i % COLORS.len() as u8;

    COLORS[i as usize]
}

pub fn darken_rgb(r: u8, g: u8, b: u8, factor: f32) -> (u8, u8, u8) {
    (
        ((r as f32) * factor) as u8,
        ((g as f32) * factor) as u8,
        ((b as f32) * factor) as u8,
    )
}
