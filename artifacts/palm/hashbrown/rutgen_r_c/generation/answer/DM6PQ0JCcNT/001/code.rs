// Answer 0

#[test]
fn test_is_special_when_tag_is_special() {
    let tag = Tag(0b1000_0000); // Top bit is set
    assert!(tag.is_special());
}

#[test]
fn test_is_special_when_tag_is_not_special() {
    let tag = Tag(0b0111_1111); // Top bit is not set
    assert!(!tag.is_special());
}

#[test]
fn test_is_special_when_tag_is_empty() {
    let tag = Tag(0b1111_1111); // All bits are set, not special
    assert!(!tag.is_special());
}

#[test]
fn test_is_special_when_tag_is_deleted() {
    let tag = Tag(0b1000_0000); // Top bit is set, representing DELETED
    assert!(tag.is_special());
}

#[test]
fn test_is_special_with_min_value() {
    let tag = Tag(0b0000_0000); // Minimum value for Tag
    assert!(!tag.is_special());
}

#[test]
fn test_is_special_with_max_value() {
    let tag = Tag(0b1111_1111); // Maximum value for Tag
    assert!(!tag.is_special());
}

