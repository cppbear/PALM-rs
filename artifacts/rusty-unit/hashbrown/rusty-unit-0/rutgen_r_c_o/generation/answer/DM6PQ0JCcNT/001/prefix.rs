// Answer 0

#[test]
fn test_is_special_with_zero() {
    let tag = Tag(0);
    let result = tag.is_special();
}

#[test]
fn test_is_special_with_one() {
    let tag = Tag(1);
    let result = tag.is_special();
}

#[test]
fn test_is_special_with_127() {
    let tag = Tag(127);
    let result = tag.is_special();
}

#[test]
fn test_is_special_with_128() {
    let tag = Tag(128);
    let result = tag.is_special();
}

#[test]
fn test_is_special_with_255() {
    let tag = Tag(255);
    let result = tag.is_special();
}

#[test]
fn test_is_special_with_64() {
    let tag = Tag(64);
    let result = tag.is_special();
}

#[test]
fn test_is_special_with_192() {
    let tag = Tag(192);
    let result = tag.is_special();
}

#[test]
fn test_is_special_with_255() {
    let tag = Tag(255);
    let result = tag.is_special();
}

