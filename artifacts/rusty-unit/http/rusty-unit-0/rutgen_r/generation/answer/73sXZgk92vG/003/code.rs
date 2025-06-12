// Answer 0

#[test]
fn test_is_visible_ascii_below_lower_bound() {
    let input = 31; // b < 32 is false
    let result = is_visible_ascii(input);
    assert_eq!(result, false); // Expecting false since b is below the visible ASCII range
}

#[test]
fn test_is_visible_ascii_tab_character() {
    let input = b'\t'; // Tab character, b == b'\t' should return true
    let result = is_visible_ascii(input);
    assert_eq!(result, true); // Expecting true for tab character
}

#[test]
fn test_is_visible_ascii_exactly_32() {
    let input = 32; // b >= 32 is true
    let result = is_visible_ascii(input);
    assert_eq!(result, true); // Expecting true since 32 is the lower bound of visible ASCII
}

#[test]
fn test_is_visible_ascii_exactly_126() {
    let input = 126; // Upper bound of visible ASCII
    let result = is_visible_ascii(input);
    assert_eq!(result, true); // Expecting true for 126 as it is in the visible range
}

#[test]
fn test_is_visible_ascii_above_upper_bound() {
    let input = 128; // b >= 32 is true but not visible
    let result = is_visible_ascii(input);
    assert_eq!(result, false); // Expecting false since 128 is outside visible ASCII range
}

