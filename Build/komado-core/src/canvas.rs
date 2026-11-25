use crate::{Error, Result};

/// Random-access byte storage given by caller (aka, the output area)
pub trait Canvas {
    /// Reads the byte at row `row`, byte column `column`
    fn get_byte(&self, row: u32, column: u32) -> Result<u8>;

    /// Writes `value` at row `row`, byte column `column`
    fn set_byte(&mut self, row: u32, column: u32, value: u8) -> Result<()>;
}

/// A flat [`Canvas`] slice
pub struct FlatCanvas<'a> {
    bytes: &'a mut [u8],
    row_bytes: usize,
}

impl<'a> FlatCanvas<'a> {
    pub const fn new(bytes: &'a mut [u8], row_bytes: usize) -> Self {
        Self { bytes, row_bytes }
    }

    // just a helper for offsets
    fn offset(&self, row: u32, column: u32) -> Result<usize> {
        let column = column as usize;
        if column >= self.row_bytes {
            return Err(Error::CanvasTooSmall);
        }
        let start = (row as usize)
            .checked_mul(self.row_bytes)
            .and_then(|value| value.checked_add(column))
            .ok_or(Error::CanvasTooSmall)?;
        if start >= self.bytes.len() {
            return Err(Error::CanvasTooSmall);
        }
        Ok(start)
    }
}

impl Canvas for FlatCanvas<'_> {
    fn get_byte(&self, row: u32, column: u32) -> Result<u8> {
        let offset = self.offset(row, column)?;
        Ok(self.bytes[offset])
    }

    fn set_byte(&mut self, row: u32, column: u32, value: u8) -> Result<()> {
        let offset = self.offset(row, column)?;
        self.bytes[offset] = value;
        Ok(())
    }
}
