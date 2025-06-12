// Answer 0

#[test]
fn test_insert_function() {
    use std::collections::hash_map::RandomState;
    use std::hash::{Hash, Hasher};

    struct TestMap {
        data: std::collections::HashMap<i32, String, RandomState>,
        hash_builder: RandomState,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                data: std::collections::HashMap::new(),
                hash_builder: RandomState::new(),
            }
        }

        pub fn insert(&mut self, key: i32, value: String) -> (&mut i32, &mut String) {
            use std::hash::Hasher;

            let mut h = self.hash_builder.build_hasher();
            key.hash(&mut h);
            let hash = h.finish();
            self.data.insert(key, value);
            let entry = self.data.get_mut(&key).unwrap();
            (&mut key, entry)
        }
    }
    
    let mut map = TestMap::new();
    let (key_ref, value_ref) = map.insert(1, "value1".to_string());

    assert_eq!(*key_ref, 1);
    assert_eq!(*value_ref, "value1");

    let (key_ref2, value_ref2) = map.insert(2, "value2".to_string());

    assert_eq!(*key_ref2, 2);
    assert_eq!(*value_ref2, "value2");

    let (key_ref3, value_ref3) = map.insert(1, "value3".to_string());

    assert_eq!(*key_ref3, 1);
    assert_eq!(*value_ref3, "value3");
}

#[test]
#[should_panic]
fn test_insert_function_panic() {
    struct PanicMap {
        data: std::collections::HashMap<i32, String>,
    }

    impl PanicMap {
        fn new() -> Self {
            PanicMap {
                data: std::collections::HashMap::new(),
            }
        }

        pub fn insert(&mut self, key: i32, value: String) -> (&mut i32, &mut String) {
            self.data.insert(key, value);
            let entry = self.data.get_mut(&key).unwrap();
            (&mut key, entry)
        }
    }

    let mut map = PanicMap::new();
    let (key_ref, value_ref) = map.insert(0, "value0".to_string());

    // This will panic since the key_ref will borrow a temporary value
    let _ = key_ref;
    let _ = *value_ref;
}

