// Answer 0

#[test]
fn test_char_return_value() {
    // Arrange
    let input_at = InputAt {
        pos: 0,
        c: Char(97), // 'a' in Unicode
        byte: Some(97), // ASCII value of 'a'
        len: 1,
    };

    // Act
    let result = input_at.char();

    // Assert
    assert_eq!(result, Char(97));
}

#[test]
fn test_char_boundary_conditions() {
    // Test input at a position that is neither start nor end.
    let input_at_middle = InputAt {
        pos: 1,
        c: Char(98), // 'b' in Unicode
        byte: Some(98),
        len: 3,
    };
    assert_eq!(input_at_middle.char(), Char(98));

    // Test input at start
    let input_at_start = InputAt {
        pos: 0,
        c: Char(99), // 'c' in Unicode
        byte: None,
        len: 3,
    };
    assert_eq!(input_at_start.char(), Char(99));

    // Test input at end with empty byte
    let input_at_end = InputAt {
        pos: 3,
        c: Char(100), // 'd' in Unicode
        byte: None,
        len: 3,
    };
    assert_eq!(input_at_end.char(), Char(100));
}

