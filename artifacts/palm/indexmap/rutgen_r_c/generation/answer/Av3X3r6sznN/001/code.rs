// Answer 0

#[test]
fn test_hasher_non_empty() {
    use std::collections::hash_map::RandomState;

    struct TestIndexMap {
        map: IndexMap<i32, i32, RandomState>,
    }

    let map = TestIndexMap {
        map: IndexMap::with_capacity_and_hasher(10, RandomState::new()),
    };

    let hasher = map.map.hasher();
    assert!(hasher != std::ptr::null());
}

#[test]
fn test_hasher_empty() {
    use std::collections::hash_map::RandomState;

    struct TestIndexMap {
        map: IndexMap<i32, i32, RandomState>,
    }

    let map = TestIndexMap {
        map: IndexMap::with_capacity_and_hasher(0, RandomState::new()),
    };

    let hasher = map.map.hasher();
    assert!(hasher != std::ptr::null());
}

