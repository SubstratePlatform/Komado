//! Bit packing for the streams
//!
//! Ref: https://www.rfc-editor.org/rfc/rfc1951#section-3.1.1

use crate::{ByteSource, Error, Result};

/// Reads bits from a [`ByteSource`] in either bit order.
///
/// DEFLATE stores Huffman codes MSB (most significant bit) first inside bytes whose
/// bits are otherwise consumed LSB first which is... weird
pub struct BitReader<'a, S: ?Sized> {
    source: &'a mut S,
    buffer: u32,
    bits: u8,
}

/// ever wanted to see esoteric Rust? congrats you have
impl<'a, S: ByteSource + ?Sized> BitReader<'a, S> {
    /// Creates a reader over `source` w/o buffered bits
    pub const fn new(source: &'a mut S) -> Self {
        Self {
            source,
            buffer: 0,
            bits: 0,
        }
    }

    /// Reads up to 16 bits, LSB first
    pub fn read_bits_lsb(&mut self, count: u8) -> Result<u16> {
        if count > 16 {
            return Err(Error::OutOfRange);
        }
        if count == 0 {
            return Ok(0);
        }

        while self.bits < count {
            let byte = self.source.read_byte()?;
            self.buffer |= (byte as u32) << self.bits;
            self.bits += 8;
        }

        let mask = if count == 16 {
            u32::MAX
        } else {
            (1u32 << count) - 1
        };
        let value = (self.buffer & mask) as u16;
        self.buffer >>= count;
        self.bits -= count;
        Ok(value)
    }

    /// Reads up to 16 bits, MSB first
    pub fn read_bits_msb(&mut self, count: u8) -> Result<u16> {
        if count > 16 {
            return Err(Error::OutOfRange);
        }
        if count == 0 {
            return Ok(0);
        }

        while self.bits < count {
            let byte = self.source.read_byte()?;
            self.buffer = (self.buffer << 8) | byte as u32;
            self.bits += 8;
        }

        let shift = self.bits - count;
        let mask = if count == 16 {
            u32::MAX
        } else {
            (1u32 << count) - 1
        };
        let value = ((self.buffer >> shift) & mask) as u16;
        self.bits -= count;
        self.buffer &= if self.bits == 0 {
            0
        } else {
            (1u32 << self.bits) - 1
        };
        Ok(value)
    }

    /// Discards the remaining bits of the current byte (optimization)
    pub fn align_to_byte(&mut self) {
        self.buffer = 0;
        self.bits = 0;
    }

    /// Returns a mutable reference to src
    pub fn source_mut(&mut self) -> &mut S {
        self.source
    }

    /// Consumes reader and return sourcen
    pub fn into_inner(self) -> &'a mut S {
        self.source
    }
}
