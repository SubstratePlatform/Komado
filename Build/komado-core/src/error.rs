#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// The input ended before the requested data was available
    UnexpectedEof,
    /// A checksum did not match the data it covered
    ChecksumMismatch,
    /// A chunk header or chunk ordering rule was invalid
    InvalidChunk,
    /// IHDR fields described an invalid PNG image
    InvalidIhdr,
    /// A PNG filter byte was outside the range 0 through 4
    InvalidFilter,
    /// A Huffman table was malformed or a code was not present in the table
    InvalidHuffmanCode,
    /// The zlib header was malformed or used an unsupported feature
    InvalidZlibHeader,
    /// The zlib stream failed validation or ended before its checksum
    InvalidZlibStream,
    /// An unknown chunk with the critical bit set
    UnknownCriticalChunk,
    /// An APNG chunk or frame layout was invalid
    InvalidApng,
    /// An fcTL or fdAT sequence number was missing, duplicated, or out of order
    InvalidSequenceNumber,
    /// An fcTL dispose operation was outside it's valid range of 0 through 2
    InvalidDisposeOp,
    /// An fcTL blend operation was outside it's valid range of 0 through 1
    InvalidBlendOp,
    /// A caller-owned backend failed. Mapped by callers.
    Io,
    /// The caller-supplied canvas is too small for the image
    CanvasTooSmall,
    /// A DEFLATE block header used a reserved block type
    InvalidDeflateBlock,
    /// A value exceeded the range allowed
    OutOfRange,
}

/// Result type
pub type Result<T> = core::result::Result<T, Error>;
