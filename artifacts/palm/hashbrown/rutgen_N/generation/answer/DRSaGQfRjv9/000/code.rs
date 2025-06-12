// Answer 0

#[derive(Debug)]
struct ControlTag(u8);

impl ControlTag {
    const fn is_full(self) -> bool {
        self.0 & 0x80 == 0
    }
}

#[test]
fn test_is_full_when_full_bucket() {
    let tag = ControlTag(0x7F); // Top bit is clear (0x80 is 0)
    assert!(tag.is_full());
}

#[test]
fn test_is_full_when_not_full_bucket() {
    let tag = ControlTag(0x80); // Top bit is set (0x80 is 1)
    assert!(!tag.is_full());
}

#[test]
fn test_is_full_with_boundary_value() {
    let tag = ControlTag(0x00); // All bits clear, should be full
    assert!(tag.is_full());
    
    let tag = ControlTag(0xFF); // All bits set, should not be full
    assert!(!tag.is_full());
}

