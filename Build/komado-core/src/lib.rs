//! The core engine shared by Komado image decoders (`no_std` and `no_alloc`)

#![no_std]
#![forbid(unsafe_code)]

mod bits;
mod canvas;
mod checksum;
mod error;
mod huffman;
mod source;

pub use bits::BitReader;
pub use canvas::{Canvas, FlatCanvas, PixelSink};
pub use checksum::{Adler32, Crc32};
pub use error::{Error, Result};
pub use huffman::{Huffman, MAX_CODE_LENGTH};
pub use source::{ByteSource, SliceSource};
