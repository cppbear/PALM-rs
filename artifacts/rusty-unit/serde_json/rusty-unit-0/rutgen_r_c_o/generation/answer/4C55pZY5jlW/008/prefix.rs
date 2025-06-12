// Answer 0

#[test]
fn test_from_escape_table_tab() {
    let escape: u8 = b't'; // corresponding to CharEscape::Tab
    let byte: u8 = 0; // testing with byte at lower edge
    let _ = CharEscape::from_escape_table(escape, byte);

    let byte: u8 = 255; // testing with byte at upper edge
    let _ = CharEscape::from_escape_table(escape, byte);
}

#[test]
#[should_panic] // testing out of range escape
fn test_from_escape_table_invalid_escape() {
    let escape: u8 = 256; // outside the valid range
    let byte: u8 = 0; 
    let _ = CharEscape::from_escape_table(escape, byte);
}

