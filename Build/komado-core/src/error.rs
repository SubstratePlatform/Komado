#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// The input ended before the requested data was available
    UnexpectedEof,
    /// A checksum did not match the data it covered
    ChecksumMismatch,
    /// A chunk header or chunk ordering rule was invalid
    InvalidChunk,
    /// A PNG filter byte was outside the range 0 through 4
    InvalidFilter,
    /// A Huffman table was malformed or a code was not present in the table
    InvalidHuffmanCode,
    /// The caller-supplied canvas is too small for the image
    CanvasTooSmall,
    /// A DEFLATE block header used a reserved block type
    InvalidDeflateBlock,
    /// A value exceeded the range allowed
    OutOfRange,
}

/// Result type
pub type Result<T> = core::result::Result<T, Error>;
