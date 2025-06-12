// Answer 0

#[test]
fn test_tag_is_full_when_full() {
    let tag = Tag(0b0111_1111);
    assert_eq!(tag.is_full(), true);
}

#[test]
fn test_tag_is_full_when_empty() {
    let tag = Tag(Tag::EMPTY.0);
    assert_eq!(tag.is_full(), true);
}

#[test]
fn test_tag_is_full_when_deleted() {
    let tag = Tag(Tag::DELETED.0);
    assert_eq!(tag.is_full(), false);
}

#[test]
fn test_tag_is_full_when_special() {
    let tag = Tag(0b1011_1111); // Setting the second bit to 1 for special
    assert_eq!(tag.is_full(), false);
}

#[test]
fn test_tag_is_full_boundary() {
    let tag = Tag(0b1111_1111); // Testing the boundary condition with all bits set
    assert_eq!(tag.is_full(), false);
}

