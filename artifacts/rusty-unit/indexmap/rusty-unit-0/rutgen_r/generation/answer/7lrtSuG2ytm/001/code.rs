// Answer 0

#[test]
fn test_key_mut_valid_index() {
    struct Entry<K> {
        key: K,
    }

    struct Container<K> {
        entries: Vec<Entry<K>>,
        index: usize,
    }

    impl<K> Container<K> {
        fn new(entries: Vec<Entry<K>>, index: usize) -> Self {
            Container { entries, index }
        }

        pub(crate) fn key_mut(&mut self) -> &mut K {
            let index = self.index;
            &mut self.entries[index].key
        }
    }

    let mut entries = vec![
        Entry { key: 1 },
        Entry { key: 2 },
        Entry { key: 3 },
    ];
    let mut container = Container::new(entries, 1);
    let key_mut = container.key_mut();
    *key_mut = 42;
    assert_eq!(container.entries[1].key, 42);
}

#[test]
#[should_panic]
fn test_key_mut_invalid_index() {
    struct Entry<K> {
        key: K,
    }

    struct Container<K> {
        entries: Vec<Entry<K>>,
        index: usize,
    }

    impl<K> Container<K> {
        fn new(entries: Vec<Entry<K>>, index: usize) -> Self {
            Container { entries, index }
        }

        pub(crate) fn key_mut(&mut self) -> &mut K {
            let index = self.index;
            &mut self.entries[index].key
        }
    }

    let entries = vec![Entry { key: 1 }];
    let mut container = Container::new(entries, 1); // Index out of bounds
    container.key_mut(); // This should panic
}

