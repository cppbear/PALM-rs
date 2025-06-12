// Answer 0

#[test]
fn test_hasher() {
    use std::collections::hash_map::RandomState;

    struct TestIndexSet {
        map: IndexMap<i32, (), RandomState>,
    }

    impl TestIndexSet {
        fn new() -> Self {
            let map = IndexMap::with_capacity_and_hasher(10, RandomState::new());
            TestIndexSet { map }
        }

        fn hasher(&self) -> &RandomState {
            self.map.hasher()
        }
    }

    let set = TestIndexSet::new();
    let hasher = set.hasher();
    assert!(std::ptr::eq(hasher, &RandomState::new())); // Check if the default hasher is the one created
}

#[test]
fn test_hasher_after_modifications() {
    use std::collections::hash_map::RandomState;

    struct TestIndexSet {
        map: IndexMap<i32, (), RandomState>,
    }

    impl TestIndexSet {
        fn new() -> Self {
            let map = IndexMap::with_capacity_and_hasher(10, RandomState::new());
            TestIndexSet { map }
        }

        fn hasher(&self) -> &RandomState {
            self.map.hasher()
        }
        
        fn insert(&mut self, value: i32) {
            self.map.insert(value, ());
        }
    }

    let mut set = TestIndexSet::new();
    set.insert(1);
    let hasher_after_insert = set.hasher();
    assert!(std::ptr::eq(hasher_after_insert, &RandomState::new())); // Ensure the hasher reference remains valid

    set.insert(2);
    let hasher_after_insert_second = set.hasher();
    assert!(std::ptr::eq(hasher_after_insert_second, &RandomState::new())); // Ensure remains valid after multiple inserts
} 

#[test]
fn test_hasher_with_different_capacity() {
    use std::collections::hash_map::RandomState;

    struct TestIndexSet {
        map: IndexMap<i32, (), RandomState>,
    }

    impl TestIndexSet {
        fn new(capacity: usize) -> Self {
            let map = IndexMap::with_capacity_and_hasher(capacity, RandomState::new());
            TestIndexSet { map }
        }

        fn hasher(&self) -> &RandomState {
            self.map.hasher()
        }
    }

    let set = TestIndexSet::new(20);
    let hasher = set.hasher();
    assert!(std::ptr::eq(hasher, &RandomState::new())); // Check if hasher returned correctly even with different initial capacity
}

