// Answer 0

#[test]
fn test_visible_ascii_lower_bound() {
    let b: u8 = 32; // Testing the lower boundary
    assert_eq!(is_visible_ascii(b), true);
}

#[test]
fn test_visible_ascii_tab_character() {
    let b: u8 = b'\t'; // Testing the tab character
    assert_eq!(is_visible_ascii(b), true);
}

#[test]
fn test_visible_ascii_upper_bound() {
    let b: u8 = 126; // Testing the upper boundary, which is still visible
    assert_eq!(is_visible_ascii(b), true);
}

#[test]
fn test_visible_ascii_above_upper_bound() {
    let b: u8 = 127; // Testing just above the upper boundary
    assert_eq!(is_visible_ascii(b), false);
}

#[test]
fn test_visible_ascii_below_lower_bound() {
    let b: u8 = 31; // Testing just below the lower boundary
    assert_eq!(is_visible_ascii(b), false);
}

