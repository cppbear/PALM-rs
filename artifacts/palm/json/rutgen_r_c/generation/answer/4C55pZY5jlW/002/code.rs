// Answer 0

#[test]
fn test_from_escape_table_ascii_control() {
    struct Test {
        escape: u8,
        byte: u8,
        expected: CharEscape,
    }

    let tests = vec![
        Test { escape: UU, byte: 0x00, expected: CharEscape::AsciiControl(0x00) },
        Test { escape: UU, byte: 0x7F, expected: CharEscape::AsciiControl(0x7F) },
        Test { escape: UU, byte: 0x1F, expected: CharEscape::AsciiControl(0x1F) },
        Test { escape: UU, byte: 0x20, expected: CharEscape::AsciiControl(0x20) },
        Test { escape: UU, byte: 0x2D, expected: CharEscape::AsciiControl(0x2D) },
        Test { escape: UU, byte: 0xFF, expected: CharEscape::AsciiControl(0xFF) },
    ];

    for test in tests {
        let result = CharEscape::from_escape_table(test.escape, test.byte);
        assert_eq!(result, test.expected);
    }
}

#[test]
#[should_panic]
fn test_from_escape_table_unreachable() {
    let escape = 0x10; // Any byte that does not map to a valid CharEscape.
    let byte = 0x00; 
    CharEscape::from_escape_table(escape, byte);
}

