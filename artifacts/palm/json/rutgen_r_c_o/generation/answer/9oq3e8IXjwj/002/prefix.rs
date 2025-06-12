// Answer 0

#[test]
fn test_skip_to_escape_slow_index_within_bounds() {
    let mut slice: &[u8] = &[1, 2, 3, 4, 5];
    let mut reader = SliceRead::new(slice);
    reader.index = 0;
    reader.skip_to_escape_slow();
}

#[test]
fn test_skip_to_escape_slow_index_equal_to_length() {
    let mut slice: &[u8] = &[1, 2, 3, 4, 5];
    let mut reader = SliceRead::new(slice);
    reader.index = 5;
    reader.skip_to_escape_slow();
}

#[test]
fn test_skip_to_escape_slow_index_stepping_through_non_escape_chars() {
    let slice: &[u8] = &[1, 2, 3, 4, b'a', b'b', 3, 4];
    let mut reader = SliceRead::new(slice);
    reader.index = 0;
    reader.skip_to_escape_slow();
}

#[test]
fn test_skip_to_escape_slow_no_escape_characters() {
    let slice: &[u8] = &[1, 2, 3, 4, 5];
    let mut reader = SliceRead::new(slice);
    reader.index = 0;
    reader.skip_to_escape_slow();
}

#[test]
fn test_skip_to_escape_slow_escape_at_end() {
    let slice: &[u8] = &[1, 2, 3, 4, b'\\'];
    let mut reader = SliceRead::new(slice);
    reader.index = 0;
    reader.skip_to_escape_slow();
}

#[test]
fn test_skip_to_escape_slow_escape_in_middle() {
    let slice: &[u8] = &[1, 2, 3, b'"', 5];
    let mut reader = SliceRead::new(slice);
    reader.index = 0;
    reader.skip_to_escape_slow();
}

#[test]
fn test_skip_to_escape_slow_all_escape_characters() {
    let slice: &[u8] = &[b'"', b'\\', b'\\', b'"'];
    let mut reader = SliceRead::new(slice);
    reader.index = 0;
    reader.skip_to_escape_slow();
}

