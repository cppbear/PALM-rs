// Answer 0

#[test]
fn test_key_mut() {
    struct Entry<K> {
        key: K,
    }

    struct Map<K> {
        entries: Vec<Entry<K>>,
    }

    struct TestStruct<K> {
        map: Map<K>,
        index: usize,
    }

    impl<K> TestStruct<K> {
        pub(crate) fn key_mut(&mut self) -> &mut K {
            &mut self.map.entries[self.index].key
        }
    }

    let mut test_struct = TestStruct {
        map: Map {
            entries: vec![Entry { key: 42 }],
        },
        index: 0,
    };

    *test_struct.key_mut() = 100;
    assert_eq!(test_struct.map.entries[0].key, 100);
}

#[test]
fn test_key_mut_empty() {
    struct Entry<K> {
        key: K,
    }

    struct Map<K> {
        entries: Vec<Entry<K>>,
    }

    struct TestStruct<K> {
        map: Map<K>,
        index: usize,
    }

    impl<K> TestStruct<K> {
        pub(crate) fn key_mut(&mut self) -> &mut K {
            &mut self.map.entries[self.index].key
        }
    }

    let mut test_struct = TestStruct {
        map: Map {
            entries: Vec::new(),
        },
        index: 0,
    };

    // This should panic because there are no entries
    let result = std::panic::catch_unwind(|| {
        test_struct.key_mut();
    });

    assert!(result.is_err());
}

