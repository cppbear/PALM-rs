// Answer 0

#[test]
fn test_lower_valid_character() {
    let range = ClassUnicodeRange { start: 'a', end: 'z' };
    assert_eq!(range.lower(), 'a');
}

#[test]
fn test_lower_boundary_character() {
    let range = ClassUnicodeRange { start: '\u{0000}', end: '\u{007F}' };
    assert_eq!(range.lower(), '\u{0000}');
}

#[test]
fn test_lower_upper_boundary() {
    let range = ClassUnicodeRange { start: '\u{007F}', end: '\u{FFFF}' };
    assert_eq!(range.lower(), '\u{007F}');
}

#[test]
fn test_lower_same_character() {
    let range = ClassUnicodeRange { start: 'm', end: 'm' };
    assert_eq!(range.lower(), 'm');
}

