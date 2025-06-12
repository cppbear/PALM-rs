// Answer 0

#[test]
fn test_is_visible_ascii_with_visible_character() {
    assert_eq!(is_visible_ascii(b'A'), true);
}

#[test]
fn test_is_visible_ascii_with_visible_ascii_boundary() {
    assert_eq!(is_visible_ascii(32), true);  // space
    assert_eq!(is_visible_ascii(126), true); // tilde
}

#[test]
fn test_is_visible_ascii_with_non_visible_character() {
    assert_eq!(is_visible_ascii(b'\n'), false);  // newline
    assert_eq!(is_visible_ascii(b'\r'), false);  // carriage return
    assert_eq!(is_visible_ascii(31), false);     // unit separator
    assert_eq!(is_visible_ascii(127), false);    // delete
}

#[test]
fn test_is_visible_ascii_with_tab() {
    assert_eq!(is_visible_ascii(b'\t'), true); // tab
}

