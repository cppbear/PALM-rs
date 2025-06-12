// Answer 0

#[test]
fn test_skip_to_escape_slow_index_at_start() {
    let mut reader = SliceRead::new(&[0x00, 0x01, 0x02, 0x22, 0x5C]);
    reader.skip_to_escape_slow();
}

#[test]
fn test_skip_to_escape_slow_index_before_escape() {
    let mut reader = SliceRead::new(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x22, 0x5C]);
    reader.index = 0;
    reader.skip_to_escape_slow();
}

#[test]
fn test_skip_to_escape_slow_index_at_escape_character() {
    let mut reader = SliceRead::new(&[0x00, 0x01, 0x22, 0x02, 0x03]);
    reader.index = 2;
    reader.skip_to_escape_slow();
}

#[test]
fn test_skip_to_escape_slow_index_at_end() {
    let mut reader = SliceRead::new(&[0x22, 0x00, 0x01, 0x02]);
    reader.index = 0;
    reader.skip_to_escape_slow();
}

#[test]
fn test_skip_to_escape_slow_multiple_escapes() {
    let mut reader = SliceRead::new(&[0x00, 0x01, 0x02, 0x22, 0x5C, 0x03]);
    reader.index = 0;
    reader.skip_to_escape_slow();
}

#[test]
fn test_skip_to_escape_slow_no_escapes() {
    let mut reader = SliceRead::new(&[0x00, 0x01, 0x02, 0x03, 0x04]);
    reader.index = 0;
    reader.skip_to_escape_slow();
}

