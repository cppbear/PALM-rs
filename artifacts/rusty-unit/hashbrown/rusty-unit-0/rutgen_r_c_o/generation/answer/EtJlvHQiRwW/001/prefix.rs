// Answer 0

#[test]
fn test_special_is_empty_tag_128() {
    let tag = Tag(128);
    let _ = tag.special_is_empty();
}

#[test]
fn test_special_is_empty_tag_129() {
    let tag = Tag(129);
    let _ = tag.special_is_empty();
}

#[test]
fn test_special_is_empty_tag_130() {
    let tag = Tag(130);
    let _ = tag.special_is_empty();
}

#[test]
fn test_special_is_empty_tag_255() {
    let tag = Tag(255);
    let _ = tag.special_is_empty();
}

