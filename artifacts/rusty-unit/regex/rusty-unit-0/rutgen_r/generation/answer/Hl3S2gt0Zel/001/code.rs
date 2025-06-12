// Answer 0

#[test]
fn test_escape_byte() {
    // Test for a printable ASCII character
    let byte_a: u8 = b'a';
    let result_a = escape_byte(byte_a);
    assert_eq!(result_a, "a");

    // Test for a whitespace character
    let byte_space: u8 = b' ';
    let result_space = escape_byte(byte_space);
    assert_eq!(result_space, " ");

    // Test for a control character: newline
    let byte_newline: u8 = b'\n';
    let result_newline = escape_byte(byte_newline);
    assert_eq!(result_newline, "\\n");

    // Test for a control character: tab
    let byte_tab: u8 = b'\t';
    let result_tab = escape_byte(byte_tab);
    assert_eq!(result_tab, "\\t");

    // Test for a special character: backslash
    let byte_backslash: u8 = b'\\';
    let result_backslash = escape_byte(byte_backslash);
    assert_eq!(result_backslash, "\\\\");

    // Test for a high ASCII character
    let byte_high: u8 = 200;
    let result_high = escape_byte(byte_high);
    assert_eq!(result_high, "\\xC8");
}

