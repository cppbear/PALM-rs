// Answer 0

#[test]
fn test_full_with_zero_hash() {
    let tag = Tag::full(0);
    assert_eq!(tag, Tag(0b0000_0000));
}

#[test]
fn test_full_with_minimum_hash() {
    let tag = Tag::full(1);
    assert_eq!(tag, Tag(0b0000_0000));
}

#[test]
fn test_full_with_max_hash() {
    let tag = Tag::full(u64::MAX);
    assert_eq!(tag, Tag(0b0111_1111)); // top 7 bits of u64::MAX is 0b0111_1111
}

#[test]
fn test_full_with_edge_case_hash() {
    let tag = Tag::full(0x7F); // 0b0111_1111
    assert_eq!(tag, Tag(0b0111_1111));
}

#[test]
fn test_full_with_high_value_hash() {
    let tag = Tag::full(0x1234567890ABCDEF);
    assert_eq!(tag, Tag(0b0010_0101)); // top 7 bits of this hash
}

