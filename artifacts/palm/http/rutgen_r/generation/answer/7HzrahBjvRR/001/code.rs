// Answer 0

#[test]
fn test_remove_entry_mult_valid_entry() {
    struct MockMap {
        entries: Vec<Entry>,
        probe: usize,
        raw_links: Vec<RawLink>,
        extra_values: Vec<Value>,
    }

    struct Entry {
        key: HeaderName,
        value: Value,
        links: Option<Link>,
    }

    struct Link {
        next: usize,
    }

    struct ValueDrain<'a, T> {
        first: Option<Value>,
        next: Option<Box<dyn Iterator<Item = Value> + 'a>>,
        lt: std::marker::PhantomData<T>,
    }

    struct HeaderName(String);

    struct Value(i32);

    impl MockMap {
        fn remove_found(&mut self, probe: usize, index: usize) -> Entry {
            self.entries.remove(index)
        }

        fn raw_links(&self) -> &Vec<RawLink> {
            &self.raw_links
        }
    }

    let mut mock_map = MockMap {
        entries: vec![
            Entry {
                key: HeaderName("Key1".to_string()),
                value: Value(1),
                links: Some(Link { next: 1 }),
            },
            Entry {
                key: HeaderName("Key2".to_string()),
                value: Value(2),
                links: None,
            },
        ],
        probe: 0,
        raw_links: vec![],
        extra_values: vec![Value(100)],
    };

    let entry = mock_map.remove_found(0, 0);
    let drain = ValueDrain {
        first: Some(entry.value),
        next: None,
        lt: std::marker::PhantomData,
    };

    assert_eq!(entry.key.0, "Key1");
    assert_eq!(drain.first.unwrap().0, 1);
}

#[test]
#[should_panic]
fn test_remove_entry_mult_invalid_index() {
    struct MockMap {
        entries: Vec<Entry>,
        probe: usize,
    }

    struct Entry {
        key: HeaderName,
        value: Value,
    }

    struct HeaderName(String);
    struct Value(i32);

    impl MockMap {
        fn remove_found(&mut self, probe: usize, index: usize) -> Entry {
            self.entries.remove(index)
        }
    }

    let mut mock_map = MockMap {
        entries: vec![
            Entry {
                key: HeaderName("Key1".to_string()),
                value: Value(1),
            },
        ],
        probe: 0,
    };

    // This will trigger panic due to accessing invalid index
    let _ = mock_map.remove_found(0, 1);
}

