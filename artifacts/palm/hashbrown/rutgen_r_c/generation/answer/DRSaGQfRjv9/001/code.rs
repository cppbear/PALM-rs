// Answer 0

#[test]
fn test_is_full_with_full_bucket() {
    let tag = Tag(0b0111_1111); // Top bit is clear, should return true
    assert!(tag.is_full());
}

#[test]
fn test_is_full_with_empty_bucket() {
    let tag = Tag(0b1111_1111); // Represents EMPTY, should return true
    assert!(tag.is_full());
}

#[test]
fn test_is_full_with_deleted_bucket() {
    let tag = Tag(0b1000_0000); // Represents DELETED, should return false
    assert!(!tag.is_full());
}

#[test]
fn test_is_full_with_special_tag() {
    let tag = Tag(0b1010_1010); // Non-full special tag, should return false
    assert!(!tag.is_full());
}

#[test]
fn test_is_full_with_non_special_full_tag() {
    let tag = Tag(0b0000_0000); // Fully empty, should return true
    assert!(tag.is_full());
}

