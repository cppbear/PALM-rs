// Answer 0

#[test]
fn test_swap_remove_entry_existing_key() {
    use std::collections::hash_map::RandomState;
    use std::hash::BuildHasher;

    struct TestMap {
        map: IndexMap<i32, String, RandomState>,
    }

    let mut test_map = TestMap {
        map: IndexMap::with_hasher(RandomState::new()),
    };
    test_map.map.insert(1, "one".to_string());
    test_map.map.insert(2, "two".to_string());
    test_map.map.insert(3, "three".to_string());

    let result = test_map.map.swap_remove_entry(&2);
    assert_eq!(result, Some((2, "two".to_string())));
    assert!(test_map.map.get(&2).is_none());
}

#[test]
fn test_swap_remove_entry_non_existing_key() {
    use std::collections::hash_map::RandomState;
    use std::hash::BuildHasher;

    struct TestMap {
        map: IndexMap<i32, String, RandomState>,
    }

    let mut test_map = TestMap {
        map: IndexMap::with_hasher(RandomState::new()),
    };
    test_map.map.insert(1, "one".to_string());
    test_map.map.insert(2, "two".to_string());

    let result = test_map.map.swap_remove_entry(&3);
    assert_eq!(result, None);
}

#[test]
fn test_swap_remove_entry_empty_map() {
    use std::collections::hash_map::RandomState;
    use std::hash::BuildHasher;

    struct TestMap {
        map: IndexMap<i32, String, RandomState>,
    }

    let mut test_map = TestMap {
        map: IndexMap::with_hasher(RandomState::new()),
    };

    let result = test_map.map.swap_remove_entry(&1);
    assert_eq!(result, None);
}

#[test]
fn test_swap_remove_entry_multiple_removals() {
    use std::collections::hash_map::RandomState;
    use std::hash::BuildHasher;

    struct TestMap {
        map: IndexMap<i32, String, RandomState>,
    }

    let mut test_map = TestMap {
        map: IndexMap::with_hasher(RandomState::new()),
    };
    test_map.map.insert(1, "one".to_string());
    test_map.map.insert(2, "two".to_string());
    test_map.map.insert(3, "three".to_string());

    let first_result = test_map.map.swap_remove_entry(&1);
    assert_eq!(first_result, Some((1, "one".to_string())));
    assert!(test_map.map.get(&1).is_none());

    let second_result = test_map.map.swap_remove_entry(&3);
    assert_eq!(second_result, Some((3, "three".to_string())));
    assert!(test_map.map.get(&3).is_none());
}

