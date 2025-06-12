// Answer 0

#[test]
fn test_swap_remove_index_valid() {
    struct TestIndexMap {
        map: IndexMap<u32, String, RandomState>,
    }

    let mut test_map = TestIndexMap {
        map: IndexMap::new(),
    };

    test_map.map.insert(1, "one".to_string());
    test_map.map.insert(2, "two".to_string());
    test_map.map.insert(3, "three".to_string());

    let result = test_map.map.swap_remove_index(1);
    assert_eq!(result, Some((2, "two".to_string())));
    assert_eq!(test_map.map.len(), 2);
}

#[test]
fn test_swap_remove_index_first() {
    struct TestIndexMap {
        map: IndexMap<u32, String, RandomState>,
    }

    let mut test_map = TestIndexMap {
        map: IndexMap::new(),
    };

    test_map.map.insert(4, "four".to_string());
    test_map.map.insert(5, "five".to_string());

    let result = test_map.map.swap_remove_index(0);
    assert_eq!(result, Some((4, "four".to_string())));
    assert_eq!(test_map.map.len(), 1);
}

#[test]
fn test_swap_remove_index_last() {
    struct TestIndexMap {
        map: IndexMap<u32, String, RandomState>,
    }

    let mut test_map = TestIndexMap {
        map: IndexMap::new(),
    };

    test_map.map.insert(6, "six".to_string());
    
    let result = test_map.map.swap_remove_index(0);
    assert_eq!(result, Some((6, "six".to_string())));
    assert_eq!(test_map.map.len(), 0);
}

#[should_panic(expected = "index out of bounds")]
#[test]
fn test_swap_remove_index_out_of_bounds() {
    struct TestIndexMap {
        map: IndexMap<u32, String, RandomState>,
    }

    let mut test_map = TestIndexMap {
        map: IndexMap::new(),
    };

    test_map.map.insert(7, "seven".to_string());
    // Attempt to remove index that is out of bounds
    let _ = test_map.map.swap_remove_index(1);
}

#[test]
fn test_swap_remove_index_empty() {
    struct TestIndexMap {
        map: IndexMap<u32, String, RandomState>,
    }

    let mut test_map = TestIndexMap {
        map: IndexMap::new(),
    };

    let result = test_map.map.swap_remove_index(0);
    assert_eq!(result, None);
}

