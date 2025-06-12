// Answer 0

#[test]
fn test_set_lower_valid_range() {
    let mut range = ClassUnicodeRange::default();
    range.set_lower(char::from_u32(0).unwrap());
}

#[test]
fn test_set_lower_mid_range() {
    let mut range = ClassUnicodeRange::default();
    range.set_lower(char::from_u32(50000).unwrap());
}

#[test]
fn test_set_lower_max_boundary() {
    let mut range = ClassUnicodeRange::default();
    range.set_lower(char::from_u32(char::MAX as u32).unwrap());
}

#[test]
fn test_set_lower_min_boundary() {
    let mut range = ClassUnicodeRange::default();
    range.set_lower(char::from_u32(1).unwrap());
}

#[test]
fn test_set_lower_non_ascii() {
    let mut range = ClassUnicodeRange::default();
    range.set_lower(char::from_u32(0x20AC).unwrap());
}

