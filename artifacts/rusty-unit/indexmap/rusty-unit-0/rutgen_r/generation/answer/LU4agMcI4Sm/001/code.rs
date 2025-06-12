// Answer 0

#[test]
fn test_key_mut_valid_index() {
    struct MapEntry<K> {
        key: K,
    }

    struct Map<K> {
        entries: Vec<MapEntry<K>>,
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

    let mut entries = vec![
        MapEntry { key: 10 },
        MapEntry { key: 20 },
        MapEntry { key: 30 },
    ];
    let mut test_struct = TestStruct {
        map: Map { entries },
        index: 1,
    };

    let key = test_struct.key_mut();
    *key += 5;
    assert_eq!(test_struct.map.entries[1].key, 25);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_key_mut_invalid_index() {
    struct MapEntry<K> {
        key: K,
    }

    struct Map<K> {
        entries: Vec<MapEntry<K>>,
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

    let entries = vec![
        MapEntry { key: 10 },
    ];
    let mut test_struct = TestStruct {
        map: Map { entries },
        index: 5,
    };

    // This will panic as index 5 is out of bounds for the entries vector
    let _key = test_struct.key_mut();
}

