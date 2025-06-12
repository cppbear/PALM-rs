// Answer 0

#[test]
fn test_sort_keys_empty() {
    struct TestMap {
        core: IndexMapCore<i32, i32>,
    }
    
    let mut map = TestMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
    };

    map.sort_keys();
    assert_eq!(map.core.entries.as_entries(), &[]);
}

#[test]
fn test_sort_keys_single_element() {
    struct TestMap {
        core: IndexMapCore<i32, i32>,
    }

    let mut map = TestMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::from(vec![(1, 10)]),
        },
    };

    map.sort_keys();
    assert_eq!(map.core.entries.as_entries(), &[(1, 10)]);
}

#[test]
fn test_sort_keys_multiple_elements_sorted() {
    struct TestMap {
        core: IndexMapCore<i32, i32>,
    }

    let mut map = TestMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::from(vec![(1, 10), (2, 20), (3, 30)]),
        },
    };

    map.sort_keys();
    assert_eq!(map.core.entries.as_entries(), &[(1, 10), (2, 20), (3, 30)]);
}

#[test]
fn test_sort_keys_multiple_elements_unsorted() {
    struct TestMap {
        core: IndexMapCore<i32, i32>,
    }

    let mut map = TestMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::from(vec![(3, 30), (1, 10), (2, 20)]),
        },
    };

    map.sort_keys();
    assert_eq!(map.core.entries.as_entries(), &[(1, 10), (2, 20), (3, 30)]);
}

#[test]
#[should_panic]
fn test_sort_keys_with_equivalent_keys() {
    struct TestMap {
        core: IndexMapCore<i32, i32>,
    }

    let mut map = TestMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::from(vec![(1, 10), (1, 20)]),
        },
    };

    map.sort_keys();
}

