// Answer 0

#[test]
fn test_lower_valid_range_1() {
    let unicode_range = ClassUnicodeRange { start: '\u{0000}', end: '\u{0001}' };
    let _ = unicode_range.lower();
}

#[test]
fn test_lower_valid_range_2() {
    let unicode_range = ClassUnicodeRange { start: '\u{D7FF}', end: '\u{E000}' };
    let _ = unicode_range.lower();
}

#[test]
fn test_lower_valid_range_3() {
    let unicode_range = ClassUnicodeRange { start: '\u{E000}', end: '\u{FFFF}' };
    let _ = unicode_range.lower();
}

#[test]
fn test_lower_valid_range_4() {
    let unicode_range = ClassUnicodeRange { start: '\u{10000}', end: '\u{10FFFF}' };
    let _ = unicode_range.lower();
}

#[test]
fn test_lower_same_character() {
    let unicode_range = ClassUnicodeRange { start: '\u{007F}', end: '\u{007F}' };
    let _ = unicode_range.lower();
}

