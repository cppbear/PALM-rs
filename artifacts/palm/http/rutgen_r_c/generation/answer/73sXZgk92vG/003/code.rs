// Answer 0

#[test]
fn test_is_visible_ascii_false_with_tab() {
    let b: u8 = b'\t';
    let result = is_visible_ascii(b);
    assert!(result, "Expected true for tab character");
}

#[test]
fn test_is_visible_ascii_false_below_visible_ascii() {
    let b: u8 = 31;  // Below the visible ASCII range
    let result = is_visible_ascii(b);
    assert!(!result, "Expected false for value below visible ASCII");
}

#[test]
fn test_is_visible_ascii_false_out_of_bounds() {
    let b: u8 = 128;  // Above the visible ASCII range
    let result = is_visible_ascii(b);
    assert!(!result, "Expected false for value above visible ASCII");
}

