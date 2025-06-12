// Answer 0

#[test]
fn test_is_full_full_bucket() {
    struct ControlTag(u8);
    
    let tag = ControlTag(0x7F); // 0b0111_1111 -> should return true (full bucket)
    assert!(tag.is_full());
}

#[test]
fn test_is_full_empty_bucket() {
    struct ControlTag(u8);
    
    let tag = ControlTag(0x80); // 0b1000_0000 -> should return false (not a full bucket)
    assert!(!tag.is_full());
}

#[test]
fn test_is_full_boundary_case_just_above_full() {
    struct ControlTag(u8);
    
    let tag = ControlTag(0x81); // 0b1000_0001 -> should return false (not a full bucket)
    assert!(!tag.is_full());
}

#[test]
fn test_is_full_boundary_case_just_below_empty() {
    struct ControlTag(u8);
    
    let tag = ControlTag(0x7E); // 0b0111_1110 -> should return true (full bucket)
    assert!(tag.is_full());
}

#[test]
fn test_is_full_full_bucket_high_nibble() {
    struct ControlTag(u8);
    
    let tag = ControlTag(0x70); // 0b0111_0000 -> should return true (full bucket)
    assert!(tag.is_full());
}

#[test]
fn test_is_full_empty_bucket_high_nibble() {
    struct ControlTag(u8);
    
    let tag = ControlTag(0xF0); // 0b1111_0000 -> should return false (not a full bucket)
    assert!(!tag.is_full());
}

