// Answer 0

#[test]
fn test_is_visible_ascii_below_lower_bound() {
    let b: u8 = 0;
    is_visible_ascii(b);
}

#[test]
fn test_is_visible_ascii_below_tab() {
    let b: u8 = 31;
    is_visible_ascii(b);
}

#[test]
fn test_is_visible_ascii_tab_character() {
    let b: u8 = b'\t';
    is_visible_ascii(b);
}

#[test]
fn test_is_visible_ascii_above_upper_bound() {
    let b: u8 = 128;
    is_visible_ascii(b);
}

#[test]
fn test_is_visible_ascii_high_value() {
    let b: u8 = 255;
    is_visible_ascii(b);
}

