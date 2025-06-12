// Answer 0

#[test]
fn test_swap_remove_full_single_entry() {
    struct TestMap {
        data: IndexMap<i32, i32, RandomState>,
    }

    let mut map = TestMap {
        data: IndexMap::new(),
    };
    map.data.insert(1, 100);

    let result = map.data.swap_remove_full(&1);
}

#[test]
fn test_swap_remove_full_multiple_entries() {
    struct TestMap {
        data: IndexMap<i32, i32, RandomState>,
    }

    let mut map = TestMap {
        data: IndexMap::new(),
    };
    map.data.insert(1, 100);
    map.data.insert(2, 200);
    
    let result = map.data.swap_remove_full(&1);
}

#[test]
fn test_swap_remove_full_not_found() {
    struct TestMap {
        data: IndexMap<i32, i32, RandomState>,
    }

    let mut map = TestMap {
        data: IndexMap::new(),
    };
    map.data.insert(1, 100);
    map.data.insert(2, 200);

    let result = map.data.swap_remove_full(&3);
}

#[test]
fn test_swap_remove_full_empty_map() {
    struct TestMap {
        data: IndexMap<i32, i32, RandomState>,
    }

    let mut map = TestMap {
        data: IndexMap::new(),
    };

    let result = map.data.swap_remove_full(&1);
}

