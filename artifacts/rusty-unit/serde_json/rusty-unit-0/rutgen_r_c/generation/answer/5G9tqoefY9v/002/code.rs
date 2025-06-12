// Answer 0

#[test]
fn test_skip_to_escape_no_escape_characters() {
    let mut slice_read = SliceRead::new(b"abcde");
    slice_read.index = 0;
    
    // Call the function with forbid_control_characters set to true
    slice_read.skip_to_escape(true);

    // Assert that the index remains the same since there are no escape characters
    assert_eq!(slice_read.index, 1);
}

#[test]
fn test_skip_to_escape_with_escape_character() {
    let mut slice_read = SliceRead::new(b"abc\\de");
    slice_read.index = 0;

    // Call the function with forbid_control_characters set to true
    slice_read.skip_to_escape(true);

    // The index should now point to the escape character
    assert_eq!(slice_read.index, 3);  // the escape character is at index 3
}

#[test]
fn test_skip_to_escape_with_quote_character() {
    let mut slice_read = SliceRead::new(b"abc\"de");
    slice_read.index = 0;

    // Call the function with forbid_control_characters set to true
    slice_read.skip_to_escape(true);

    // The index should now point to the quote character
    assert_eq!(slice_read.index, 3);  // the quote character is at index 3
}

#[test]
fn test_skip_to_escape_with_control_characters() {
    let mut slice_read = SliceRead::new(b"abc\x1Fde");
    slice_read.index = 0;  // Valid index

    // Call the function with forbid_control_characters set to true
    slice_read.skip_to_escape(true);

    // The index should now point to the control character
    assert_eq!(slice_read.index, 3);  // the control character is at index 3
}

#[test]
fn test_skip_to_escape_end_of_slice() {
    let mut slice_read = SliceRead::new(b"");
    slice_read.index = 0;  // Edge case for empty slice

    // Call the function with forbid_control_characters set to true
    slice_read.skip_to_escape(true);

    // The index should remain unchanged since the slice is empty
    assert_eq!(slice_read.index, 0);
}

#[test]
fn test_skip_to_escape_with_only_escape_characters() {
    let mut slice_read = SliceRead::new(b"\\\\\\");
    slice_read.index = 0;  // Start at the beginning

    // Call the function with forbid_control_characters set to false
    slice_read.skip_to_escape(false);

    // The index should point to the end since all characters are escapes
    assert_eq!(slice_read.index, 3);  // all characters are backslashes
}

