// Answer 0

#[test]
fn test_into_entries_empty() {
    struct DummyHashBuilder;

    #[cfg(not(feature = "std"))]
    let index_set: super::IndexSet<i32, DummyHashBuilder> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: DummyHashBuilder,
        },
    };

    let entries: Vec<super::Bucket<i32>> = index_set.into_entries();
    assert!(entries.is_empty());
}

#[test]
fn test_into_entries_single() {
    struct DummyHashBuilder;

    #[cfg(not(feature = "std"))]
    let mut index_set: super::IndexSet<i32, DummyHashBuilder> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::with_entries(vec![
                super::Bucket { hash: 1, key: 42, value: () }
            ]),
            hash_builder: DummyHashBuilder,
        },
    };

    let entries = index_set.into_entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].key, 42);
}

#[test]
fn test_into_entries_multiple() {
    struct DummyHashBuilder;

    #[cfg(not(feature = "std"))]
    let mut index_set: super::IndexSet<i32, DummyHashBuilder> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::with_entries(vec![
                super::Bucket { hash: 1, key: 1, value: () },
                super::Bucket { hash: 2, key: 2, value: () },
                super::Bucket { hash: 3, key: 3, value: () },
            ]),
            hash_builder: DummyHashBuilder,
        },
    };

    let entries = index_set.into_entries();
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].key, 1);
    assert_eq!(entries[1].key, 2);
    assert_eq!(entries[2].key, 3);
}

