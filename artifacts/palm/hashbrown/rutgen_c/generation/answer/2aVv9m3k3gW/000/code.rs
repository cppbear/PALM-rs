// Answer 0

#[test]
fn test_make_hasher_with_default_builder() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct TestStruct {
        key: u32,
    }

    impl Hash for TestStruct {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.key.hash(state);
        }
    }

    let hash_builder = DefaultHasher::new();
    let hasher = make_hasher::<TestStruct, (), _>(&hash_builder);

    let val = (TestStruct { key: 42 }, ());
    let hash_value = hasher(&val);

    assert_eq!(hash_value, 6938914531401378170); // Example expected; replace with the actual expected value.
}

#[test]
fn test_make_hasher_with_custom_builder() {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    struct TestStruct {
        key: u64,
    }

    impl Hash for TestStruct {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.key.hash(state);
        }
    }

    let mut hash_builder = DefaultHasher::new();
    let hasher = make_hasher::<TestStruct, (), _>(&hash_builder);

    let val1 = (TestStruct { key: 100 }, ());
    let val2 = (TestStruct { key: 200 }, ());

    let hash_value1 = hasher(&val1);
    let hash_value2 = hasher(&val2);

    assert_ne!(hash_value1, hash_value2);
}

#[test]
fn test_make_hasher_with_empty_key() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct TestStruct {
        key: i32,
    }

    impl Hash for TestStruct {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.key.hash(state);
        }
    }

    let hash_builder = DefaultHasher::new();
    let hasher = make_hasher::<TestStruct, (), _>(&hash_builder);

    let val_empty = (TestStruct { key: 0 }, ());
    let hash_empty_value = hasher(&val_empty);

    assert_eq!(hash_empty_value, 0); // An expected value for the given input can vary.
}

