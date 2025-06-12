// Answer 0

#[test]
fn test_position_of_index_with_no_newline() {
    let data = b"HelloWorld"; // No new lines
    let slice_read = SliceRead::new(data);
    let position = slice_read.position_of_index(5);
    assert_eq!(position.line, 1); // There should be 1 line
    assert_eq!(position.column, 5); // Column should be 5 for character index 5
}

#[test]
fn test_position_of_index_with_single_newline() {
    let data = b"Hello\nWorld"; // Contains one newline
    let slice_read = SliceRead::new(data);
    let position = slice_read.position_of_index(6); // Position after '\n'
    assert_eq!(position.line, 2); // 2 lines
    assert_eq!(position.column, 0); // Column after newline should be 0
}

#[test]
fn test_position_of_index_with_multiple_newlines() {
    let data = b"Line1\nLine2\nLine3"; // Contains multiple newlines
    let slice_read = SliceRead::new(data);
    let position = slice_read.position_of_index(11); // Position at '2'
    assert_eq!(position.line, 2); // 2 lines total
    assert_eq!(position.column, 1); // Column should be 1 for index 11
}

#[test]
fn test_position_of_index_with_start_of_slice() {
    let data = b"\nHello\n"; // Newline at the start
    let slice_read = SliceRead::new(data);
    let position = slice_read.position_of_index(1); // Position after the first byte '\n'
    assert_eq!(position.line, 1); // 1 line only
    assert_eq!(position.column, 1); // Column should be 1 for index 1
}

#[test]
fn test_position_of_index_with_end_of_slice() {
    let data = b"Hello\nWorld"; // End of slice after characters
    let slice_read = SliceRead::new(data);
    let position = slice_read.position_of_index(10); // Position at the end of the slice
    assert_eq!(position.line, 2); // 2 lines total
    assert_eq!(position.column, 5); // Column at end of second line
}

#[should_panic]
fn test_position_of_index_panic_condition() {
    let data = b"Hello\nWorld"; // Valid data
    let slice_read = SliceRead::new(data);
    let _ = slice_read.position_of_index(12); // Out of bounds index, should panic
}

