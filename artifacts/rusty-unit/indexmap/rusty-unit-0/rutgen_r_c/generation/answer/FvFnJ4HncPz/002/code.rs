// Answer 0

#[test]
fn test_get_range_mut_valid_range() {
    struct TestKeys;
    struct TestValues;

    let mut map = IndexMap::<TestKeys, TestValues, ()> {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: (),
    };

    let result = map.get_range_mut(0..1);
    assert!(result.is_some());
}

#[test]
fn test_get_range_mut_empty_map() {
    struct TestKeys;
    struct TestValues;

    let mut map = IndexMap::<TestKeys, TestValues, ()> {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: (),
    };

    let result = map.get_range_mut(0..1);
    assert!(result.is_none());
}

#[test]
fn test_get_range_mut_out_of_bounds() {
    struct TestKeys;
    struct TestValues;

    let mut map = IndexMap::<TestKeys, TestValues, ()> {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: (),
    };

    let result = map.get_range_mut(1..2);
    assert!(result.is_none());
}

#[test]
fn test_get_range_mut_unbounded_range() {
    struct TestKeys;
    struct TestValues;

    let mut map = IndexMap::<TestKeys, TestValues, ()> {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: (),
    };

    let result = map.get_range_mut(..);
    assert!(result.is_some());
}

#[test]
fn test_get_range_mut_partial_filled_map() {
    struct TestKeys;
    struct TestValues;

    let mut map = IndexMap::<TestKeys, TestValues, ()> {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: (),
    };

    // Add enough entries to test.
    for _ in 0..5 {
        map.insert(TestKeys, TestValues);
    }

    let result = map.get_range_mut(1..3);
    assert!(result.is_some());
}

