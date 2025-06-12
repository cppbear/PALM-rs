// Answer 0

#[test]
fn test_special_is_empty_with_empty_tag() {
    let tag = Tag::EMPTY;
    assert!(!tag.special_is_empty());
}

#[test]
fn test_special_is_empty_with_deleted_tag() {
    let tag = Tag(Tag::DELETED.0 | 0x01);
    assert!(tag.special_is_empty());
}

#[test]
fn test_special_is_empty_with_non_empty_special_tag() {
    let tag = Tag(Tag::DELETED.0 | 0x00);
    assert!(!tag.special_is_empty());
}

#[test]
#[should_panic]
fn test_special_is_empty_should_panic_on_non_special_tag() {
    let tag = Tag(0);
    tag.special_is_empty(); // This should cause a panic
}

