// Answer 0

#[test]
fn test_shift_insert_hashed_nocheck_valid_case() {
    struct MockHashBuilder;

    impl BuildHasher for MockHashBuilder {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut entries: Vec<Bucket<i32, i32>> = Vec::new();
    let mut indices = Indices::new();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let mut entry = RawVacantEntryMut {
        map: ref_mut,
        hash_builder: &MockHashBuilder,
    };

    let index = 0;
    let hash = 42;
    let key = 1;
    let value = 10;

    entry.shift_insert_hashed_nocheck(index, hash, key, value);

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].key, key);
    assert_eq!(entries[0].value, value);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_shift_insert_hashed_nocheck_out_of_bounds() {
    struct MockHashBuilder;

    impl BuildHasher for MockHashBuilder {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut entries: Vec<Bucket<i32, i32>> = Vec::new();
    let mut indices = Indices::new();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let mut entry = RawVacantEntryMut {
        map: ref_mut,
        hash_builder: &MockHashBuilder,
    };

    let index = 1; // Out of bounds since entries is empty
    let hash = 42;
    let key = 1;
    let value = 10;

    entry.shift_insert_hashed_nocheck(index, hash, key, value);
}

#[test]
fn test_shift_insert_hashed_nocheck_boundary_case() {
    struct MockHashBuilder;

    impl BuildHasher for MockHashBuilder {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut entries: Vec<Bucket<i32, i32>> = Vec::new();
    let mut indices = Indices::new();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let mut entry = RawVacantEntryMut {
        map: ref_mut,
        hash_builder: &MockHashBuilder,
    };

    // Insert at index 0
    let index = 0;
    let hash = 100;
    let key = 2;
    let value = 20;

    entry.shift_insert_hashed_nocheck(index, hash, key, value);

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].key, key);
    assert_eq!(entries[0].value, value);
}

