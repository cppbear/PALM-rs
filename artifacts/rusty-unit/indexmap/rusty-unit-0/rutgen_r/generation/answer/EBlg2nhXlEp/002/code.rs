// Answer 0

#[test]
fn test_from_key_hashed_nocheck_existing_entry() {
    struct KeyStruct(u32);
    
    impl std::hash::Hash for KeyStruct {
        fn hash<H>(&self, state: &mut H)
        where
            H: std::hash::Hasher,
        {
            self.0.hash(state);
        }
    }
    
    impl PartialEq for KeyStruct {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for KeyStruct {}

    struct TestMap {
        core: TestCore,
    }

    struct TestCore {
        keys: Vec<KeyStruct>,
        values: Vec<u32>,
    }

    impl TestCore {
        fn get_index_of(&self, hash: HashValue, key: &KeyStruct) -> Option<usize> {
            self.keys.iter().position(|k| k == key)
        }
    }

    impl TestMap {
        fn get_index(&self, index: usize) -> Option<(&KeyStruct, &u32)> {
            Some((&self.core.keys[index], &self.core.values[index]))
        }
    }

    struct HashValue(usize);

    let key1 = KeyStruct(1);
    let value1 = 100;

    let test_map = TestMap {
        core: TestCore {
            keys: vec![key1.clone()],
            values: vec![value1],
        },
    };

    let result = test_map.from_key_hashed_nocheck(1, &key1);
    assert!(result.is_some());
    if let Some((k, v)) = result {
        assert_eq!(k.0, key1.0);
        assert_eq!(*v, value1);
    }
}

#[test]
#[should_panic]
fn test_from_key_hashed_nocheck_non_existing_entry() {
    struct KeyStruct(u32);
    
    impl std::hash::Hash for KeyStruct {
        fn hash<H>(&self, state: &mut H)
        where
            H: std::hash::Hasher,
        {
            self.0.hash(state);
        }
    }
    
    impl PartialEq for KeyStruct {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for KeyStruct {}

    struct TestMap {
        core: TestCore,
    }

    struct TestCore {
        keys: Vec<KeyStruct>,
        values: Vec<u32>,
    }

    impl TestCore {
        fn get_index_of(&self, hash: HashValue, key: &KeyStruct) -> Option<usize> {
            if self.keys.contains(key) {
                Some(self.keys.iter().position(|k| k == key).unwrap())
            } else {
                None
            }
        }
    }

    impl TestMap {
        fn get_index(&self, index: usize) -> Option<(&KeyStruct, &u32)> {
            Some((&self.core.keys[index], &self.core.values[index]))
        }
    }

    struct HashValue(usize);

    let key2 = KeyStruct(2);
    let test_map = TestMap {
        core: TestCore {
            keys: vec![KeyStruct(1)],
            values: vec![100],
        },
    };

    // This should panic as key2 does not exist in the map
    let _ = test_map.from_key_hashed_nocheck(2, &key2);
}

