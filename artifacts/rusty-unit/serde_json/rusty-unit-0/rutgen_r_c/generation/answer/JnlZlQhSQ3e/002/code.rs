// Answer 0

#[test]
fn test_position_of_index_no_newline_before_index() {
    let data = b"Hello World! This is a test."; 
    let slice_read = SliceRead::new(data);
    
    // Test with an index that is within bounds and where there is no newline.
    let index = 5;  // 'o' in "Hello"
    
    let position = slice_read.position_of_index(index);
    
    assert_eq!(position.line, 1);
    assert_eq!(position.column, index); // column should equal the index since no newlines.
}

#[test]
fn test_position_of_index_with_newline() {
    let data = b"Hello World!\nThis is a test."; 
    let slice_read = SliceRead::new(data);
    
    // Test with an index where there is a newline before it.
    let index = 13;  // After the '!' in "Hello World!"
    
    let position = slice_read.position_of_index(index);
    
    assert_eq!(position.line, 2); // There's one newline, so line should be 2.
    assert_eq!(position.column, 0); // The column should be 0 since this is right after the newline.
}

#[test]
fn test_position_of_index_index_at_start() {
    let data = b"";
    let slice_read = SliceRead::new(data);
    
    // Test with an index of 0 on an empty slice.
    let index = 0;
    
    let position = slice_read.position_of_index(index);
    
    assert_eq!(position.line, 1); // Even with an empty slice, it should return line 1.
    assert_eq!(position.column, 0); // Column should be 0 since there's no character at index.
}

#[test]
fn test_position_of_index_single_character() {
    let data = b"a";
    let slice_read = SliceRead::new(data);
    
    // Test with an index of 0 on a slice with a single character.
    let index = 0;
    
    let position = slice_read.position_of_index(index);
    
    assert_eq!(position.line, 1); // Line count should still be 1.
    assert_eq!(position.column, 0); // Column should be 0 for index 0.
}

#[test]
fn test_position_of_index_with_various_indices() {
    let data = b"Line1\nLine2\nLine3\n"; 
    let slice_read = SliceRead::new(data);
    
    // Testing multiple indices for their positions
    let indexes = vec![0, 5, 11, 17]; // Start of Line1, 'n' of Line1, start of Line2, start of Line3
    let expected_positions = vec![(1, 0), (1, 1), (2, 0), (3, 0)]; // Expected (line, column)

    for (i, &index) in indexes.iter().enumerate() {
        let position = slice_read.position_of_index(index);
        assert_eq!(position.line, expected_positions[i].0);
        assert_eq!(position.column, expected_positions[i].1);
    }
}

