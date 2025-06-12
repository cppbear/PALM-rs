// Answer 0

#[test]
fn test_retain_with_all_elements_kept() {
    struct DummyHashBuilder;

    let mut map = IndexMap::<i32, String, DummyHashBuilder> {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: DummyHashBuilder,
    };

    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());

    map.retain(|_, _| true);

    assert_eq!(map.len(), 3);
}

#[test]
fn test_retain_with_no_elements_kept() {
    struct DummyHashBuilder;

    let mut map = IndexMap::<i32, String, DummyHashBuilder> {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: DummyHashBuilder,
    };

    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());

    map.retain(|_, _| false);

    assert_eq!(map.len(), 0);
}

#[test]
fn test_retain_with_some_elements_kept() {
    struct DummyHashBuilder;

    let mut map = IndexMap::<i32, String, DummyHashBuilder> {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: DummyHashBuilder,
    };

    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());

    map.retain(|k, _| *k != 2);

    assert_eq!(map.len(), 2);
    assert!(map.contains_key(&1));
    assert!(map.contains_key(&3));
    assert!(!map.contains_key(&2));
}

