//! Example that definitely works on Raspberry Pi.
//! Make sure you have "SPI" on your Pi enabled and that MOSI-Pin is connected
//! with DIN-Pin. You just need DIN pin, no clock. WS2818 uses one-wire-protocol.
//! See the specification for details

use rand::Rng;
use rust_led_lights::{darken_rgb, get_random_pixel_val, sleep_busy_waiting_ms};
use std::ops::Add;
use std::time::{Duration, Instant};
use ws2818_rgb_led_spi_driver::adapter_gen::WS28xxAdapter;
use ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter;

use anyhow::Result;

pub const FREQUENCY: u64 = 30; // Hz
pub const FREQUENCY_MS: u64 = 1000 / FREQUENCY;

// This animation sends moving light impulses via the LED strip
fn main() -> Result<()> {
    let mut adapter = WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap();
    let num_leds = 100;
    let mut anim = MovingLightStripsAnimation::new(num_leds);
    let mut rng = rand::thread_rng();

    let mut next_light_time = Instant::now();
    let mut burst_len = 0;
    let mut pause_len = 0;
    loop {
        let now = Instant::now();
        if now >= next_light_time {
            burst_len += rng.gen_range(1..21);
            println!("Burst len: {}", burst_len);
        }
        if burst_len > 0 && pause_len == 0 {
            burst_len -= 1;
            pause_len += rng.gen_range(1..21);
            anim.add_next_light_impulse();
            next_light_time = now.add(Duration::from_secs(rng.gen_range(1..6)))
        } else if pause_len > 0 {
            pause_len -= 1;
        }
        anim.shift_all_pixels();
        adapter
            .write_rgb(&anim.rgb_data[MOVING_LIGHT_IMPULSE_LEN..])
            .unwrap();

        sleep_busy_waiting_ms(FREQUENCY_MS);
    }
}

const MOVING_LIGHT_IMPULSE_LEN: usize = 15;

pub struct MovingLightStripsAnimation {
    rgb_data: Vec<(u8, u8, u8)>,
}

impl MovingLightStripsAnimation {
    pub fn new(led_count: usize) -> Self {
        MovingLightStripsAnimation {
            rgb_data: vec![(0, 0, 0); led_count + MOVING_LIGHT_IMPULSE_LEN],
        }
    }
}

impl MovingLightStripsAnimation {
    /// Shifts all pixel to the next position. Beginning is filled
    /// with black (0, 0, 0).
    fn shift_all_pixels(&mut self) {
        let upper_border = self.rgb_data.len();
        for i in 0..upper_border {
            // loop backwards
            let i = upper_border - 1 - i;

            if i == 0 {
                let _ = std::mem::take(&mut self.rgb_data[i]);
            } else {
                let prev = self.rgb_data[i - 1];
                let _ = std::mem::replace(&mut self.rgb_data[i], prev);
            }
        }
    }
    fn add_next_light_impulse(&mut self) {
        let (r, g, b) = get_random_pixel_val();
        let _ = std::mem::replace(&mut self.rgb_data[0], darken_rgb(r, g, b, 0.1));
        let _ = std::mem::replace(&mut self.rgb_data[1], darken_rgb(r, g, b, 0.2));
        let _ = std::mem::replace(&mut self.rgb_data[2], darken_rgb(r, g, b, 0.4));
        let _ = std::mem::replace(&mut self.rgb_data[3], darken_rgb(r, g, b, 0.6));
        let _ = std::mem::replace(&mut self.rgb_data[4], darken_rgb(r, g, b, 0.7));
        let _ = std::mem::replace(&mut self.rgb_data[5], darken_rgb(r, g, b, 0.8));
        let _ = std::mem::replace(&mut self.rgb_data[6], darken_rgb(r, g, b, 0.9));
        let _ = std::mem::replace(&mut self.rgb_data[7], (r, g, b));
        let _ = std::mem::replace(&mut self.rgb_data[8], darken_rgb(r, g, b, 0.9));
        let _ = std::mem::replace(&mut self.rgb_data[9], darken_rgb(r, g, b, 0.8));
        let _ = std::mem::replace(&mut self.rgb_data[10], darken_rgb(r, g, b, 0.7));
        let _ = std::mem::replace(&mut self.rgb_data[11], darken_rgb(r, g, b, 0.6));
        let _ = std::mem::replace(&mut self.rgb_data[12], darken_rgb(r, g, b, 0.4));
        let _ = std::mem::replace(&mut self.rgb_data[13], darken_rgb(r, g, b, 0.2));
        let _ = std::mem::replace(&mut self.rgb_data[14], darken_rgb(r, g, b, 0.1));
    }
}
