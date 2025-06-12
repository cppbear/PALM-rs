// Answer 0

#[test]
fn test_full_with_zero_hash() {
    let tag = Tag::full(0);
    assert_eq!(tag, Tag(0));
}

#[test]
fn test_full_with_low_hash() {
    let tag = Tag::full(100);
    let expected_top7 = 0b0000001; // 100 >> (8 - 7) = 12 -> 0b0000000 -> masked with 0x7f
    assert_eq!(tag, Tag(expected_top7));
}

#[test]
fn test_full_with_high_hash() {
    let tag = Tag::full(0xFFFFFFFFFFFFFFFF);
    let expected_top7 = 0b0111111; // 0xFFFFFFFFFFFFFFFF >> (8 - 7) = 0x3FFFFFFFFFFFFF -> masked with 0x7f
    assert_eq!(tag, Tag(expected_top7));
}

#[test]
fn test_full_with_mid_hash() {
    let tag = Tag::full(0x7FFFFFFFFFFFFFFF);
    let expected_top7 = 0b0111111; // 0x7FFFFFFFFFFFFFFF >> (8 - 7) = 0x3FFFFFFFFFFFFFFF -> masked with 0x7f
    assert_eq!(tag, Tag(expected_top7));
}

#[test]
fn test_full_with_nonce_hash() {
    let tag = Tag::full(0x1FFFFFFFFFFFFFFF);
    let expected_top7 = 0b0000111; // 0x1FFFFFFFFFFFFFFF >> (8 - 7) = 0x0FFFFFFFFFFFFFFF -> masked with 0x7f
    assert_eq!(tag, Tag(expected_top7));
}

