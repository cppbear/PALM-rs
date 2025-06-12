// Answer 0

#[test]
fn test_clear_empty_map() {
    struct TestMap {
        core: crate::IndexMapCore<i32, i32>,
    }

    let mut map = TestMap {
        core: crate::IndexMapCore::new(),
    };

    let initial_capacity = map.core.capacity();
    map.clear();
    assert_eq!(map.core.len(), 0);
    assert_eq!(map.core.capacity(), initial_capacity);
}

#[test]
fn test_clear_non_empty_map() {
    struct TestMap {
        core: crate::IndexMapCore<i32, i32>,
    }

    let mut map = TestMap {
        core: crate::IndexMapCore::with_capacity(10),
    };

    map.core.entries.push((1, 10));
    map.core.entries.push((2, 20));
    map.core.entries.push((3, 30));

    let initial_capacity = map.core.capacity();
    map.clear();
    assert_eq!(map.core.len(), 0);
    assert_eq!(map.core.capacity(), initial_capacity);
}

#[test]
fn test_clear_filled_map() {
    struct TestMap {
        core: crate::IndexMapCore<i32, i32>,
    }

    let mut map = TestMap {
        core: crate::IndexMapCore::with_capacity(5),
    };

    for i in 1..=5 {
        map.core.entries.push((i, i * 10));
    }

    map.clear();
    assert_eq!(map.core.len(), 0);
    assert!(map.core.entries.is_empty());
}

