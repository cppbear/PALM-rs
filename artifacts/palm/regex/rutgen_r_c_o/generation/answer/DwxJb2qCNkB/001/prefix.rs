// Answer 0

#[test]
fn test_set_upper_valid_range_1() {
    let mut range = ClassUnicodeRange::default();
    range.set_upper(char::from('\u{0000}'));
}

#[test]
fn test_set_upper_valid_range_2() {
    let mut range = ClassUnicodeRange::default();
    range.set_upper(char::from('\u{007F}'));
}

#[test]
fn test_set_upper_valid_range_3() {
    let mut range = ClassUnicodeRange::default();
    range.set_upper(char::from('\u{00FF}'));
}

#[test]
fn test_set_upper_valid_range_4() {
    let mut range = ClassUnicodeRange::default();
    range.set_upper(char::from('\u{FFFF}'));
}

#[test]
fn test_set_upper_valid_range_5() {
    let mut range = ClassUnicodeRange::default();
    range.set_upper(char::from('\u{10FFFF}'));
}

