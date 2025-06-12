// Answer 0

#[test]
fn test_escape_byte_valid_bytes() {
    let test_cases: Vec<(u8, &str)> = vec![
        (0, "\\0"),
        (1, "\\x01"),
        (10, "\\n"),
        (13, "\\r"),
        (27, "\\x1b"),
        (255, "\\xff"),
    ];

    for (input, expected) in test_cases {
        let result = escape_byte(input);
        assert_eq!(result, expected);
    }
}

#[test]
fn test_escape_byte_non_printable_bytes() {
    let non_printable_bytes: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    for byte in non_printable_bytes {
        let result = escape_byte(byte);
        assert!(result.starts_with("\\x") || result == "\\0" || result == "\\n" || result == "\\r");
    }
}

#[test]
fn test_escape_byte_high_byte() {
    let high_byte: u8 = 128;
    let result = escape_byte(high_byte);
    assert_eq!(result, "\\x80");
}

#[test]
fn test_escape_byte_boundary_value() {
    let result_zero = escape_byte(0);
    assert_eq!(result_zero, "\\0");

    let result_255 = escape_byte(255);
    assert_eq!(result_255, "\\xff");
}

