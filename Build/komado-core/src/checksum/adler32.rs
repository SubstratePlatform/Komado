//! Adler-32 implementation
//!
//! Ref: https://www.rfc-editor.org/rfc/rfc1950#section-2.2

/// Incremental Adler-32
#[derive(Clone, Copy, Debug)]
pub struct Adler32 {
    a: u32,
    b: u32,
}

impl Adler32 {
    const BASE: u32 = 65_521;

    pub const fn new() -> Self {
        Self { a: 1, b: 0 }
    }

    /// Feeds `bytes` into the accumulator thing
    pub fn update(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.a += byte as u32;
            if self.a >= Self::BASE {
                self.a -= Self::BASE;
            }
            self.b += self.a;
            if self.b >= Self::BASE {
                self.b -= Self::BASE;
            }
        }
    }

    /// Returns the Adler-32 of all bytes fed (so far)
    pub const fn value(&self) -> u32 {
        (self.b << 16) | self.a
    }
}

impl Default for Adler32 {
    fn default() -> Self {
        Self::new()
    }
}
