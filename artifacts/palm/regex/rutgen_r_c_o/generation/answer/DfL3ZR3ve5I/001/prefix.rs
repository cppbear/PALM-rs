// Answer 0

#[test]
fn test_fmt_start_whitespace_end_whitespace() {
    let range = ClassUnicodeRange {
        start: char::from(0x00), // Whitespace
        end: char::from(0x20)    // Whitespace
    };
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{:?}", range);  // Trigger the fmt function
}

#[test]
fn test_fmt_start_whitespace_end_normal() {
    let range = ClassUnicodeRange {
        start: char::from(0x00), // Whitespace
        end: char::from(0x80)    // Non-whitespace
    };
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{:?}", range);  // Trigger the fmt function
}

#[test]
fn test_fmt_start_normal_end_whitespace() {
    let range = ClassUnicodeRange {
        start: char::from(0x30), // Non-whitespace
        end: char::from(0x20)    // Whitespace
    };
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{:?}", range);  // Trigger the fmt function
}

#[test]
fn test_fmt_start_whitespace_end_whitespace_range() {
    let range = ClassUnicodeRange {
        start: char::from(0x10), // Whitespace
        end: char::from(0x1F)    // Whitespace
    };
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{:?}", range);  // Trigger the fmt function
}

#[test]
fn test_fmt_start_normal_end_normal() {
    let range = ClassUnicodeRange {
        start: char::from(0x30), // Normal character
        end: char::from(0x7F)    // Normal character
    };
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{:?}", range);  // Trigger the fmt function
}

