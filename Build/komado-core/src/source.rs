use crate::Error;

/// Pull source of input bytes
///
/// Doesn't allocate
/// A source that has no more bytes returns [`Error::UnexpectedEof`]
pub trait ByteSource {
    /// Reads the next byte
    fn read_byte(&mut self) -> Result<u8, Error>;
}

/// A [`ByteSource`] over a borrowed slice
pub struct SliceSource<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> SliceSource<'a> {
    /// Creates a source positioned at the start of `bytes`.
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    /// Consumed bytes
    pub const fn position(&self) -> usize {
        self.position
    }

    /// Remaining unconsumed bytes
    pub const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }
}

impl ByteSource for SliceSource<'_> {
    fn read_byte(&mut self) -> Result<u8, Error> {
        let byte = *self.bytes.get(self.position).ok_or(Error::UnexpectedEof)?;
        self.position += 1;
        Ok(byte)
    }
}
