// Answer 0

#[test]
fn test_unicode_literal_valid() {
    let valid_unicode_chars = [
        char::from_u32(0).unwrap(),  // U+0000: Null character
        char::from_u32(0x0041).unwrap(),  // U+0041: 'A'
        char::from_u32(0x03A9).unwrap(),  // U+03A9: Greek Capital Letter Omega
        char::from_u32(0x1F600).unwrap(),  // U+1F600: Grinning Face Emoji
        char::from_u32(0x10FFFF).unwrap(),  // U+10FFFF: Last valid Unicode scalar value
    ];

    for &ch in &valid_unicode_chars {
        let literal = Literal::Unicode(ch);
        literal.is_unicode();
    }
}

#[test]
fn test_byte_literal_valid() {
    let valid_byte_values = [0x00, 0x7F]; // Valid Byte range (0-127)

    for &b in &valid_byte_values {
        let literal = Literal::Byte(b);
        literal.is_unicode();
    }
}

