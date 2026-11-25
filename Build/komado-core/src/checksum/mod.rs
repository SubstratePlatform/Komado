//! All checksum algorithms supported by Komado

mod adler32;
mod crc32;

pub use adler32::Adler32;
pub use crc32::Crc32;
