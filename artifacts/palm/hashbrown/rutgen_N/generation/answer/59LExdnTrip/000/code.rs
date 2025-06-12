// Answer 0

#[derive(Debug, PartialEq)]
struct Tag(u8);

const fn full(hash: u64) -> Tag {
    const MIN_HASH_LEN: usize = if std::mem::size_of::<usize>() < std::mem::size_of::<u64>() {
        std::mem::size_of::<usize>()
    } else {
        std::mem::size_of::<u64>()
    };

    let top7 = hash >> (MIN_HASH_LEN * 8 - 7);
    Tag((top7 & 0x7f) as u8) // truncation
}

#[test]
fn test_full_with_zero_hash() {
    let tag = full(0);
    assert_eq!(tag, Tag(0));
}

#[test]
fn test_full_with_small_hash() {
    let tag = full(1);
    assert_eq!(tag, Tag(0)); // Top 7 bits of 1 should still be 0
}

#[test]
fn test_full_with_large_hash() {
    let tag = full(0xFFFFFFFFFFFFFFFF); // Maximum 64-bit value
    assert_eq!(tag, Tag(0x7F)); // Top 7 bits should be 0x7F
}

#[test]
fn test_full_with_specific_hash() {
    let tag = full(0x7F00000000000000); // Example hash
    assert_eq!(tag, Tag(0x7F)); // Top 7 bits extraction
}

#[test]
fn test_full_with_boundary_hash() {
    let tag = full(0x3FFFFFFFFFFFFFFF); // Just below the max
    assert_eq!(tag, Tag(0x3F)); // Top 7 bits should reflect the value
}

