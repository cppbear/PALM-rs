// Answer 0

#[derive(Hash, PartialEq, Eq)]
struct Key;

struct MockHasher;

impl HashBuilder for MockHasher {
    // Assume this is implemented correctly to satisfy the Hash trait
}

#[test]
fn test_build_hashes_inner_valid() {
    let hasher = MockHasher;
    let keys: [&Key; 3] = [&Key, &Key, &Key];

    let hashes = hasher.build_hashes_inner(&keys);
    
    assert_eq!(hashes.len(), 3);
    for hash in hashes.iter() {
        assert_ne!(*hash, 0_u64);
    }
}

#[test]
#[should_panic]
fn test_build_hashes_inner_out_of_bounds() {
    let hasher = MockHasher;
    let keys: [&Key; 0] = [];

    let _hashes = hasher.build_hashes_inner(&keys);
}

#[derive(Hash, PartialEq, Eq)]
struct AnotherKey;

#[test]
fn test_build_hashes_inner_mixed_keys() {
    let hasher = MockHasher;
    let keys: [&dyn std::hash::Hash; 2] = [&Key, &AnotherKey];

    let hashes = hasher.build_hashes_inner(&keys);

    assert_eq!(hashes.len(), 2);
}

