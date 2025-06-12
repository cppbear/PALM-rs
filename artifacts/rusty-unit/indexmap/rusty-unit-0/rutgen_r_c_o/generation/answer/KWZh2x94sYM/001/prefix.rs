// Answer 0

#[test]
fn test_with_hasher_random_state() {
    use std::collections::hash_map::RandomState;
    let hash_builder = RandomState::new();
    let map: IndexMap<i32, i32, RandomState> = IndexMap::with_hasher(hash_builder);
}

#[test]
fn test_with_hasher_custom_hasher() {
    use std::hash::{BuildHasher, Hash, Hasher};
    struct CustomHasher;
    impl BuildHasher for CustomHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let hash_builder = CustomHasher;
    let map: IndexMap<i32, i32, CustomHasher> = IndexMap::with_hasher(hash_builder);
}

#[test]
fn test_with_hasher_empty() {
    let map: IndexMap<i32, i32, RandomState> = IndexMap::with_hasher(RandomState::new());
}

#[test]
fn test_with_hasher_large_capacity() {
    use std::collections::hash_map::RandomState;
    let n = IndexMapCore::<i32, i32>::MAX_ENTRIES_CAPACITY;
    let hash_builder = RandomState::new();
    let map: IndexMap<i32, i32, RandomState> = IndexMap::with_hasher(hash_builder);
}

#[should_panic]
fn test_with_hasher_out_of_bounds_capacity() {
    let n = (isize::MAX as usize) / std::mem::size_of::<Bucket<i32, i32>>() + 1;
    let hash_builder = RandomState::new();
    let _map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(n, hash_builder);
}

