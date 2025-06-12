// Answer 0

#[test]
fn test_from_escape_table_backspace() {
    let escape: u8 = 98; // self::BB
    for byte in 0..=255 {
        let result = CharEscape::from_escape_table(escape, byte);
    }
}

#[test]
fn test_from_escape_table_tab() {
    let escape: u8 = 9; // self::TT
    for byte in 0..=255 {
        let result = CharEscape::from_escape_table(escape, byte);
    }
}

#[test]
fn test_from_escape_table_line_feed() {
    let escape: u8 = 10; // self::NN
    for byte in 0..=255 {
        let result = CharEscape::from_escape_table(escape, byte);
    }
}

#[test]
fn test_from_escape_table_form_feed() {
    let escape: u8 = 12; // self::FF
    for byte in 0..=255 {
        let result = CharEscape::from_escape_table(escape, byte);
    }
}

#[test]
fn test_from_escape_table_carriage_return() {
    let escape: u8 = 13; // self::RR
    for byte in 0..=255 {
        let result = CharEscape::from_escape_table(escape, byte);
    }
}

#[test]
fn test_from_escape_table_quote() {
    let escape: u8 = 34; // self::QU
    for byte in 0..=255 {
        let result = CharEscape::from_escape_table(escape, byte);
    }
}

#[test]
fn test_from_escape_table_reverse_solidus() {
    let escape: u8 = 92; // self::BS
    for byte in 0..=255 {
        let result = CharEscape::from_escape_table(escape, byte);
    }
}

#[test]
fn test_from_escape_table_ascii_control() {
    let escape: u8 = 117; // self::UU
    for byte in 0..=255 {
        let result = CharEscape::from_escape_table(escape, byte);
    }
}

#[should_panic]
#[test]
fn test_from_escape_table_unreachable() {
    let escape: u8 = 255; // out of defined range
    let byte: u8 = 0; // arbitrary value
    let result = CharEscape::from_escape_table(escape, byte);
}

