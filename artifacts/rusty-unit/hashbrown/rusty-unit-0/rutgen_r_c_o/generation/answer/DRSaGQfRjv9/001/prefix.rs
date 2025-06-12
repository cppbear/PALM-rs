// Answer 0

#[test]
fn test_is_full_zero() {
    let tag = Tag(0);
    let result = tag.is_full();
}

#[test]
fn test_is_full_one() {
    let tag = Tag(1);
    let result = tag.is_full();
}

#[test]
fn test_is_full_fifty_seven() {
    let tag = Tag(57);
    let result = tag.is_full();
}

#[test]
fn test_is_full_one_hundred_twenty_seven() {
    let tag = Tag(127);
    let result = tag.is_full();
}

#[test]
fn test_is_full_one_hundred_twenty_eight() {
    let tag = Tag(128);
    let result = tag.is_full();
}

#[test]
fn test_is_full_one_hundred() {
    let tag = Tag(100);
    let result = tag.is_full();
}

#[test]
fn test_is_full_sixty_four() {
    let tag = Tag(64);
    let result = tag.is_full();
}

#[test]
fn test_is_full_empty_tag() {
    let tag = Tag(Tag::EMPTY.0);
    let result = tag.is_full();
}

#[test]
fn test_is_full_deleted_tag() {
    let tag = Tag(Tag::DELETED.0);
    let result = tag.is_full();
}

