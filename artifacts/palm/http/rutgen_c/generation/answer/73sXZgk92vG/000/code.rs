// Answer 0

#[test]
fn test_is_visible_ascii_visible_character() {
    assert_eq!(is_visible_ascii(b'A'), true);
}

#[test]
fn test_is_visible_ascii_space_character() {
    assert_eq!(is_visible_ascii(b' '), true);
}

#[test]
fn test_is_visible_ascii_tab_character() {
    assert_eq!(is_visible_ascii(b'\t'), true);
}

#[test]
fn test_is_visible_ascii_non_visible_character() {
    assert_eq!(is_visible_ascii(b'\n'), false);
}

#[test]
fn test_is_visible_ascii_upper_boundary() {
    assert_eq!(is_visible_ascii(b'~'), true);
}

#[test]
fn test_is_visible_ascii_lower_boundary() {
    assert_eq!(is_visible_ascii(b'!'), true);
}

#[test]
fn test_is_visible_ascii_below_lower_boundary() {
    assert_eq!(is_visible_ascii(b'\x1F'), false);
}

#[test]
fn test_is_visible_ascii_above_upper_boundary() {
    assert_eq!(is_visible_ascii(b'\x80'), false);
}

