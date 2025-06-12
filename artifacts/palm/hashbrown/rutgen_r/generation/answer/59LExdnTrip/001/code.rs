// Answer 0

#[derive(Debug)]
struct Tag(u8);

#[test]
fn test_full_zero_hash() {
    let tag = full(0);
    assert_eq!(tag, Tag(0));
}

#[test]
fn test_full_max_hash() {
    let tag = full(u64::MAX);
    assert_eq!(tag, Tag(0x7F)); // Top 7 bits of u64::MAX are all 1s
}

#[test]
fn test_full_mid_hash() {
    let tag = full(0x7FFFFFFF_FFFFFFFF);
    assert_eq!(tag, Tag(0x3F)); // Top 7 bits should be 0111111
}

#[test]
fn test_full_hash_with_less_than_7_bits() {
    let hash = 0x0000000F; // 15 in decimal, only 4 bits are set
    let tag = full(hash);
    assert_eq!(tag, Tag(0)); // Top 7 bits should be 0000000
}

#[test]
fn test_full_hash_with_exactly_7_bits() {
    let hash = 0x7F; // Exact 7 bits set
    let tag = full(hash as u64);
    assert_eq!(tag, Tag(0x7F)); // Top 7 bits should be 1111111
}

