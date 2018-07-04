#[macro_use]
extern crate structopt;

use std::time::{SystemTime, UNIX_EPOCH};

pub mod opt;

pub const RADIOTAP_HEADER: [u8; 25] = [
    0x00, 0x00, // <-- radiotap version
    0x19, 0x00, // <-- radiotap header length
    0x6f, 0x08, 0x00, 0x00, // <-- bitmap
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // <-- timestamp
    0x00, // <-- flags
    0x0c, // <-- rate
    0x71, 0x09, 0xc0, 0x00, // <-- channel
    0xde, // <-- antsignal
    0x00, // <-- antnoise
    0x01, // <-- antenna
];

pub const WIFI_HEADER: [u8; 26] = [
    0x00, 0x00, 0x30, 0x00,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0x23, 0x23, 0x23, 0x23, 0x23, 0x23,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xc0, 0x20, 0x20, 0x00
];

pub const LLC_HEADER: [u8; 8] = [
    0xaa, 0xaa, 0x03,
    0x00, 0x00, 0x00,
    0x88, 0xb5
];

pub struct XorShift {
    state: [u64; 4]
}

impl Default for XorShift {
    fn default() -> Self {
        // Try to initialize the state randomly with the clock time
        // If that fails, use a predefined state
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(dur) => {
                XorShift { state: [1, dur.as_secs(), dur.subsec_nanos() as u64, 21] }
            },
            Err(_) => XorShift { state: [1, 21, 41, 51] }
        }
    }
}

impl XorShift {
    pub fn rand(&mut self) -> u64 {
        let mut t;

        t = self.state[3];
        t ^= t << 11;
        t ^= t << 8;
        self.state[3] = self.state[2];
        self.state[2] = self.state[1];
        self.state[1] = self.state[0];
        t ^= self.state[0];
        t ^= self.state[0] >> 19;
        self.state[0] = t;

        t
    }
    pub fn rand_u8(&mut self) -> u8 {
        self.rand() as u8
    }
}
