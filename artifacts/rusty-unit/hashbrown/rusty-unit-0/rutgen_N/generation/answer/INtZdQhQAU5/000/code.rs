// Answer 0

#[test]
fn test_make_hash_with_default_hasher() {
    use std::collections::hash_map::RandomState;
    use std::hash::{Hash, Hasher};

    struct MyStruct {
        value: String,
    }

    impl Hash for MyStruct {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.value.hash(state);
        }
    }

    let hasher = RandomState::new();
    let my_value = MyStruct {
        value: String::from("test"),
    };

    let hash_value = make_hash(&hasher, &my_value);
    assert!(hash_value > 0);
}

#[test]
fn test_make_hash_empty_string() {
    use std::collections::hash_map::RandomState;
    use std::hash::{Hash, Hasher};

    struct MyStruct {
        value: String,
    }

    impl Hash for MyStruct {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.value.hash(state);
        }
    }

    let hasher = RandomState::new();
    let my_value = MyStruct {
        value: String::from(""),
    };

    let hash_value = make_hash(&hasher, &my_value);
    assert!(hash_value > 0);
}

#[test]
fn test_make_hash_with_different_values() {
    use std::collections::hash_map::RandomState;
    use std::hash::{Hash, Hasher};

    struct MyStruct {
        value: String,
    }

    impl Hash for MyStruct {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.value.hash(state);
        }
    }

    let hasher = RandomState::new();
    let my_value1 = MyStruct {
        value: String::from("test1"),
    };
    let my_value2 = MyStruct {
        value: String::from("test2"),
    };

    let hash_value1 = make_hash(&hasher, &my_value1);
    let hash_value2 = make_hash(&hasher, &my_value2);

    assert!(hash_value1 != hash_value2);
}

