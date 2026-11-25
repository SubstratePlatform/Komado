//! Huffman codes
//!
//! Ref: https://www.rfc-editor.org/rfc/rfc1951#section-3.2.2

use crate::{BitReader, ByteSource, Error, Result};

/// Longest code length permitted by DEFLATE and JPEG (spec detail)
pub const MAX_CODE_LENGTH: usize = 15;

/// Huffman decoder
#[derive(Clone, Debug)]
pub struct Huffman<const N: usize> {
    counts: [u16; MAX_CODE_LENGTH + 1],
    first_code: [u32; MAX_CODE_LENGTH + 1],
    first_index: [u16; MAX_CODE_LENGTH + 1],
    symbols: [u16; N],
}

impl<const N: usize> Huffman<N> {
    /// Builds a decoder from one code length per symbol thing
    pub fn new(lengths: &[u8]) -> Result<Self> {
        if lengths.len() != N {
            return Err(Error::InvalidHuffmanCode);
        }

        let mut counts = [0u16; MAX_CODE_LENGTH + 1];
        for &length in lengths {
            let length = length as usize;
            if length > MAX_CODE_LENGTH {
                return Err(Error::InvalidHuffmanCode);
            }
            counts[length] += 1;
        }

        let mut left = 1i32;
        for &count in counts.iter().skip(1) {
            left = (left << 1) - count as i32;
            if left < 0 {
                return Err(Error::InvalidHuffmanCode);
            }
        }

        let mut first_code = [0u32; MAX_CODE_LENGTH + 1];
        let mut first_index = [0u16; MAX_CODE_LENGTH + 1];
        let mut code = 0u32;
        let mut index = 0u16;
        for length in 1..=MAX_CODE_LENGTH {
            first_code[length] = code;
            first_index[length] = index;
            code = (code + counts[length] as u32) << 1;
            index += counts[length];
        }

        let mut cursor = first_index;
        let mut symbols = [0u16; N];
        for (symbol, &length) in lengths.iter().enumerate() {
            if length != 0 {
                let slot = cursor[length as usize] as usize;
                symbols[slot] = symbol as u16;
                cursor[length as usize] += 1;
            }
        }

        Ok(Self {
            counts,
            first_code,
            first_index,
            symbols,
        })
    }

    /// Decodes one symbol from `reader`
    pub fn decode<S: ByteSource>(&self, reader: &mut BitReader<S>) -> Result<u16> {
        let mut code = 0u32;
        for length in 1..=MAX_CODE_LENGTH {
            code = (code << 1) | reader.read_bits_lsb(1)? as u32;
            let count = self.counts[length] as u32;
            if code >= self.first_code[length] && code - self.first_code[length] < count {
                let offset = (code - self.first_code[length]) as usize;
                return Ok(self.symbols[self.first_index[length] as usize + offset]);
            }
        }
        Err(Error::InvalidHuffmanCode)
    }
}
