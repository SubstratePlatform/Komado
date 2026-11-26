//! CRC-32
//!
//! Ref: https://www.w3.org/TR/png-3/#5-5-crc-algorithm

#[derive(Clone, Copy, Debug)]
pub struct Crc32 {
    state: u32,
}

impl Crc32 {
    const POLYNOMIAL: u32 = 0xEDB8_8320;

    pub const fn new() -> Self {
        Self { state: u32::MAX }
    }

    /// Feeds `bytes` into the accumulator
    pub fn update(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.update_byte(byte);
        }
    }

    /// Feeds a streamed byte into the accumulator
    pub fn update_byte(&mut self, byte: u8) {
        self.state ^= byte as u32;
        for _ in 0..8 {
            self.state = if self.state & 1 != 0 {
                (self.state >> 1) ^ Self::POLYNOMIAL
            } else {
                self.state >> 1
            };
        }
    }

    /// Returns the CRC-32 of all bytes fed (so far)
    pub const fn value(&self) -> u32 {
        !self.state
    }
}

impl Default for Crc32 {
    fn default() -> Self {
        Self::new()
    }
}
