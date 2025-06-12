// Answer 0

#[test]
fn test_is_visible_ascii_boundary_condition_lower() {
    let b = 32; // Lower boundary condition: should return true
    assert_eq!(is_visible_ascii(b), true);
}

#[test]
fn test_is_visible_ascii_boundary_condition_upper() {
    let b = 126; // Upper boundary condition: should return true
    assert_eq!(is_visible_ascii(b), true);
}

#[test]
fn test_is_visible_ascii_tab_character() {
    let b = b'\t'; // Tab character: should return true
    assert_eq!(is_visible_ascii(b), true);
}

#[test]
fn test_is_visible_ascii_below_lower_boundary() {
    let b = 31; // Below lower boundary: should return false
    assert_eq!(is_visible_ascii(b), false);
}

#[test]
fn test_is_visible_ascii_above_upper_boundary() {
    let b = 127; // Above upper boundary: should return false
    assert_eq!(is_visible_ascii(b), false);
}

