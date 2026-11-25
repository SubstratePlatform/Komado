use komado_core::{Adler32, BitReader, Canvas, Crc32, Error, FlatCanvas, Huffman, SliceSource};

#[test]
fn lsb_reader_consumes_bits_in_stream_order() {
    let mut reader = BitReader::new(SliceSource::new(&[0xB3]));
    assert_eq!(reader.read_bits_lsb(3), Ok(3));
    assert_eq!(reader.read_bits_lsb(2), Ok(2));
    assert_eq!(reader.read_bits_lsb(3), Ok(5));
    assert_eq!(reader.read_bits_lsb(1), Err(Error::UnexpectedEof));
}

#[test]
fn msb_reader_consumes_bits_in_stream_order() {
    let mut reader = BitReader::new(SliceSource::new(&[0xB3]));
    assert_eq!(reader.read_bits_msb(3), Ok(5));
    assert_eq!(reader.read_bits_msb(2), Ok(2));
    assert_eq!(reader.read_bits_msb(3), Ok(3));
}

#[test]
fn lsb_reader_spans_byte_boundaries() {
    let mut reader = BitReader::new(SliceSource::new(&[0xB3, 0x4C]));
    assert_eq!(reader.read_bits_lsb(5), Ok(19));
    assert_eq!(reader.read_bits_lsb(8), Ok(101));
    assert_eq!(reader.read_bits_lsb(3), Ok(2));
}

#[test]
fn msb_reader_spans_byte_boundaries() {
    let mut reader = BitReader::new(SliceSource::new(&[0xB3, 0x4C]));
    assert_eq!(reader.read_bits_msb(5), Ok(22));
    assert_eq!(reader.read_bits_msb(8), Ok(105));
    assert_eq!(reader.read_bits_msb(3), Ok(4));
}

#[test]
fn byte_alignment_skips_the_rest_of_the_current_byte() {
    let mut reader = BitReader::new(SliceSource::new(&[0xB3, 0x4C]));
    assert_eq!(reader.read_bits_lsb(3), Ok(3));
    reader.align_to_byte();
    assert_eq!(reader.read_bits_lsb(8), Ok(0x4C));
}

#[test]
fn reader_rejects_widths_above_sixteen_bits() {
    let mut reader = BitReader::new(SliceSource::new(&[0]));
    assert_eq!(reader.read_bits_lsb(17), Err(Error::OutOfRange));
    assert_eq!(reader.read_bits_msb(17), Err(Error::OutOfRange));
}

#[test]
fn huffman_decodes_codes() {
    let table = Huffman::<3>::new(&[1, 2, 2]).expect("valid table");
    let mut reader = BitReader::new(SliceSource::new(&[0x1A]));
    assert_eq!(table.decode(&mut reader), Ok(0));
    assert_eq!(table.decode(&mut reader), Ok(1));
    assert_eq!(table.decode(&mut reader), Ok(2));
}

#[test]
fn huffman_matches_rfc_example() {
    let table = Huffman::<8>::new(&[3, 3, 3, 3, 3, 2, 4, 4]).expect("valid table");
    let mut reader = BitReader::new(SliceSource::new(&[0x72, 0x3A, 0xEE, 0x01]));
    for expected in 0..8u16 {
        assert_eq!(table.decode(&mut reader), Ok(expected));
    }
}

#[test]
fn huffman_rejects_oversubscribed_tables() {
    assert_eq!(
        Huffman::<3>::new(&[1, 1, 1]).err(),
        Some(Error::InvalidHuffmanCode)
    );
}

#[test]
fn crc32_matches_reference_vector() {
    let mut crc = Crc32::new();
    crc.update(b"123456789");
    assert_eq!(crc.value(), 0xCBF4_3926);
}

#[test]
fn adler32_matches_reference_vector() {
    let mut adler = Adler32::new();
    adler.update(b"123456789");
    assert_eq!(adler.value(), 0x091E_01DE);
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
