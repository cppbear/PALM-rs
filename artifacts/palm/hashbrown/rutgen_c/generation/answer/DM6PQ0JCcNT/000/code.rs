// Answer 0

#[test]
fn test_is_special_empty() {
    let tag = Tag::EMPTY;
    assert!(!tag.is_special());
}

#[test]
fn test_is_special_deleted() {
    let tag = Tag(Tag::DELETED.0);
    assert!(tag.is_special());
}

#[test]
fn test_is_special_non_special_value() {
    let tag = Tag(0b0111_1111); // Non-special value
    assert!(!tag.is_special());
}

#[test]
fn test_is_special_special_value() {
    let tag = Tag(0b1000_0001); // Special value with additional bits
    assert!(tag.is_special());
}

