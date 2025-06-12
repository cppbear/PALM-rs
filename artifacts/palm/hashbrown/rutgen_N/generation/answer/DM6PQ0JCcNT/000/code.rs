// Answer 0

#[derive(Debug)]
struct ControlTag(u8);

impl ControlTag {
    const fn is_special(self) -> bool {
        self.0 & 0x80 != 0
    }
}

#[test]
fn test_is_special_with_top_bit_set() {
    let tag = ControlTag(0x80);
    assert!(tag.is_special());
}

#[test]
fn test_is_special_with_top_bit_clear() {
    let tag = ControlTag(0x7F);
    assert!(!tag.is_special());
}

#[test]
fn test_is_special_with_zero() {
    let tag = ControlTag(0x00);
    assert!(!tag.is_special());
}

#[test]
fn test_is_special_with_value_above_special() {
    let tag = ControlTag(0xFF);
    assert!(tag.is_special());
}

#[test]
fn test_is_special_with_value_below_special() {
    let tag = ControlTag(0x40);
    assert!(!tag.is_special());
}

