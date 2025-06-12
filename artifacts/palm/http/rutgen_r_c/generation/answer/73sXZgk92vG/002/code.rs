// Answer 0

#[test]
fn test_is_visible_ascii_lower_bound() {
    let b: u8 = 32;
    assert_eq!(is_visible_ascii(b), true);
}

#[test]
fn test_is_visible_ascii_upper_bound() {
    let b: u8 = 127;
    assert_eq!(is_visible_ascii(b), false);
}

#[test]
fn test_is_visible_ascii_tab() {
    let b: u8 = b'\t'; // ASCII tab character
    assert_eq!(is_visible_ascii(b), true);
}

