use komado_core::{
    Adler32, BitReader, Canvas, Crc32, Error, FlatCanvas, Huffman, PixelSink, Result, SliceSource,
};

#[test]
fn lsb_reader_consumes_bits_in_stream_order() {
    let mut source = SliceSource::new(&[0xB3]);
    let mut reader = BitReader::new(&mut source);
    assert_eq!(reader.read_bits_lsb(3), Ok(3));
    assert_eq!(reader.read_bits_lsb(2), Ok(2));
    assert_eq!(reader.read_bits_lsb(3), Ok(5));
    assert_eq!(reader.read_bits_lsb(1), Err(Error::UnexpectedEof));
}

#[test]
fn msb_reader_consumes_bits_in_stream_order() {
    let mut source = SliceSource::new(&[0xB3]);
    let mut reader = BitReader::new(&mut source);
    assert_eq!(reader.read_bits_msb(3), Ok(5));
    assert_eq!(reader.read_bits_msb(2), Ok(2));
    assert_eq!(reader.read_bits_msb(3), Ok(3));
}

#[test]
fn lsb_reader_spans_byte_boundaries() {
    let mut source = SliceSource::new(&[0xB3, 0x4C]);
    let mut reader = BitReader::new(&mut source);
    assert_eq!(reader.read_bits_lsb(5), Ok(19));
    assert_eq!(reader.read_bits_lsb(8), Ok(101));
    assert_eq!(reader.read_bits_lsb(3), Ok(2));
}

#[test]
fn msb_reader_spans_byte_boundaries() {
    let mut source = SliceSource::new(&[0xB3, 0x4C]);
    let mut reader = BitReader::new(&mut source);
    assert_eq!(reader.read_bits_msb(5), Ok(22));
    assert_eq!(reader.read_bits_msb(8), Ok(105));
    assert_eq!(reader.read_bits_msb(3), Ok(4));
}

#[test]
fn byte_alignment_skips_the_rest_of_the_current_byte() {
    let mut source = SliceSource::new(&[0xB3, 0x4C]);
    let mut reader = BitReader::new(&mut source);
    assert_eq!(reader.read_bits_lsb(3), Ok(3));
    reader.align_to_byte();
    assert_eq!(reader.read_bits_lsb(8), Ok(0x4C));
}

#[test]
fn reader_rejects_widths_above_sixteen_bits() {
    let mut source = SliceSource::new(&[0]);
    let mut reader = BitReader::new(&mut source);
    assert_eq!(reader.read_bits_lsb(17), Err(Error::OutOfRange));
    assert_eq!(reader.read_bits_msb(17), Err(Error::OutOfRange));
}

#[test]
fn bit_reader_returns_the_borrowed_source() {
    let mut source = SliceSource::new(&[0xB3, 0x4C]);
    let mut reader = BitReader::new(&mut source);
    assert_eq!(reader.read_bits_lsb(8), Ok(0xB3));
    assert_eq!(reader.into_inner().position(), 1);
}

#[test]
fn huffman_decodes_codes() {
    let table = Huffman::<3>::new(&[1, 2, 2]).expect("valid table");
    let mut source = SliceSource::new(&[0x1A]);
    let mut reader = BitReader::new(&mut source);
    assert_eq!(table.decode(&mut reader), Ok(0));
    assert_eq!(table.decode(&mut reader), Ok(1));
    assert_eq!(table.decode(&mut reader), Ok(2));
}

#[test]
fn huffman_matches_rfc_example() {
    let table = Huffman::<8>::new(&[3, 3, 3, 3, 3, 2, 4, 4]).expect("valid table");
    let mut source = SliceSource::new(&[0x72, 0x3A, 0xEE, 0x01]);
    let mut reader = BitReader::new(&mut source);
    for expected in 0..8u16 {
        assert_eq!(table.decode(&mut reader), Ok(expected));
    }
}

#[test]
fn huffman_builds_into_an_existing_table() {
    let mut table = Huffman::<3>::empty();
    assert_eq!(table.build_into(&[1, 2, 2]), Ok(()));

    let mut source = SliceSource::new(&[0x1A]);
    let mut reader = BitReader::new(&mut source);
    assert_eq!(table.decode(&mut reader), Ok(0));
    assert_eq!(table.decode(&mut reader), Ok(1));
    assert_eq!(table.decode(&mut reader), Ok(2));
}

#[test]
fn huffman_rejects_oversubscribed_tables() {
    assert_eq!(
        Huffman::<3>::new(&[1, 1, 1]).err(),
        Some(Error::InvalidHuffmanCode)
    );
}

#[test]
fn huffman_build_failure_leaves_table_usable() {
    let mut table = Huffman::<3>::empty();
    table.build_into(&[1, 2, 2]).expect("valid table");
    assert_eq!(table.build_into(&[1, 1, 1]), Err(Error::InvalidHuffmanCode));

    let mut source = SliceSource::new(&[0x1A]);
    let mut reader = BitReader::new(&mut source);
    assert_eq!(table.decode(&mut reader), Ok(0));
    assert_eq!(table.decode(&mut reader), Ok(1));
    assert_eq!(table.decode(&mut reader), Ok(2));
}

#[test]
fn crc32_matches_reference_vector() {
    let mut crc = Crc32::new();
    crc.update(b"123456789");
    assert_eq!(crc.value(), 0xCBF4_3926);
}

#[test]
fn crc32_updates_one_byte_at_a_time() {
    let mut crc = Crc32::new();
    for &byte in b"a" {
        crc.update_byte(byte);
    }
    assert_eq!(crc.value(), 0xE8B7_BE43);
}

#[test]
fn adler32_matches_reference_vector() {
    let mut adler = Adler32::new();
    adler.update(b"123456789");
    assert_eq!(adler.value(), 0x091E_01DE);
}

#[test]
fn adler32_updates_one_byte_at_a_time() {
    let mut adler = Adler32::new();
    for &byte in b"a" {
        adler.update_byte(byte);
    }
    assert_eq!(adler.value(), 0x0062_0062);
}

#[test]
fn flat_canvas_round_trips_bytes_and_rejects_bad_addresses() {
    let mut storage = [0u8; 8];
    let mut canvas = FlatCanvas::new(&mut storage, 4);
    canvas.set_byte(1, 2, 0x7F).expect("address is in range");
    assert_eq!(canvas.get_byte(1, 2), Ok(0x7F));
    assert_eq!(canvas.get_byte(2, 0), Err(Error::CanvasTooSmall));
    assert_eq!(canvas.get_byte(0, 4), Err(Error::CanvasTooSmall));
}

#[test]
fn canvas_reads_can_mutate_caching_backend() {
    struct CachedCanvas {
        bytes: [u8; 4],
        reads: u32,
    }

    /// technically read/write, but it counts reads for testing ig
    impl Canvas for CachedCanvas {
        fn get_byte(&mut self, row: u32, column: u32) -> Result<u8> {
            self.reads += 1;
            Ok(self.bytes[(row * 2 + column) as usize])
        }

        fn set_byte(&mut self, row: u32, column: u32, value: u8) -> Result<()> {
            self.bytes[(row * 2 + column) as usize] = value;
            Ok(())
        }
    }

    let mut canvas = CachedCanvas {
        bytes: [0; 4],
        reads: 0,
    };
    canvas.set_byte(1, 1, 0x5A).expect("address is in range");
    assert_eq!(canvas.get_byte(1, 1), Ok(0x5A));
    assert_eq!(canvas.reads, 1);
}

#[test]
fn canvas_flush_defaults_to_no_op() {
    let mut storage = [0u8; 4];
    let mut canvas = FlatCanvas::new(&mut storage, 2);
    assert_eq!(canvas.flush(), Ok(()));
}

#[test]
fn pixel_sink_receives_rows_and_flush() {
    struct RowSink {
        rows: [[u8; 2]; 2],
        flushed: bool,
    }

    impl PixelSink for RowSink {
        fn present_row(&mut self, row: u32, bytes: &[u8]) -> Result<()> {
            self.rows[row as usize].copy_from_slice(bytes);
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            self.flushed = true;
            Ok(())
        }
    }

    let mut sink = RowSink {
        rows: [[0; 2]; 2],
        flushed: false,
    };
    /// if this fails... i reallly messed up
    /// later: phew it didn't
    sink.present_row(1, &[0x11, 0x22]).expect("row is in range");
    assert_eq!(sink.flush(), Ok(()));
    assert_eq!(sink.rows[1], [0x11, 0x22]);
    assert!(sink.flushed);
}
