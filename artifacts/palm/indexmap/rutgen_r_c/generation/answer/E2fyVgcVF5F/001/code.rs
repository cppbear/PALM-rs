// Answer 0

#[test]
fn test_indexed_entry_get_valid_index() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    let mut map = TestMap {
        entries: vec![
            Bucket { hash: HashValue::default(), key: "key1", value: 10 },
            Bucket { hash: HashValue::default(), key: "key2", value: 20 },
        ],
    };

    let indexed_entry = IndexedEntry::new(&mut map, 1);
    assert_eq!(indexed_entry.get(), &20);
}

#[should_panic(expected = "index out of bounds")]
#[test]
fn test_indexed_entry_get_out_of_bounds_index() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    let mut map = TestMap {
        entries: vec![],
    };

    let indexed_entry = IndexedEntry::new(&mut map, 0);
    indexed_entry.get();
}

#[test]
fn test_indexed_entry_get_empty_map() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    let mut map = TestMap {
        entries: vec![],
    };

    let indexed_entry = IndexedEntry::new(&mut map, 0);
    let result = std::panic::catch_unwind(|| indexed_entry.get());
    assert!(result.is_err());
}

