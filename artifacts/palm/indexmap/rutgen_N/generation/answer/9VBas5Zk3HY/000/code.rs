// Answer 0

#[test]
fn test_get_full_mut2_found() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct MyKey {
        id: usize,
    }

    struct MyValue {
        data: String,
    }

    impl Hash for MyKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id.hash(state);
        }
    }

    trait Equivalent<T> {
        fn equivalent(&self, other: &T) -> bool;
    }

    impl Equivalent<MyValue> for MyKey {
        fn equivalent(&self, other: &MyValue) -> bool {
            self.id == other.data.len()
        }
    }

    let mut map = indexmap::IndexMap::new();
    map.insert(MyKey { id: 3 }, MyValue { data: "test".to_string() });

    let key = MyKey { id: 3 };
    let result = map.get_full_mut2(&key);

    assert!(result.is_some());
    if let Some((index, value)) = result {
        assert_eq!(index, 0);
        assert_eq!(value.data, "test");
    }
}

#[test]
fn test_get_full_mut2_not_found() {
    struct MyKey {
        id: usize,
    }

    struct MyValue {
        data: String,
    }

    impl Hash for MyKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id.hash(state);
        }
    }

    trait Equivalent<T> {
        fn equivalent(&self, other: &T) -> bool;
    }

    impl Equivalent<MyValue> for MyKey {
        fn equivalent(&self, other: &MyValue) -> bool {
            self.id == other.data.len()
        }
    }

    let mut map = indexmap::IndexMap::new();
    map.insert(MyKey { id: 3 }, MyValue { data: "test".to_string() });

    let key = MyKey { id: 4 };
    let result = map.get_full_mut2(&key);

    assert!(result.is_none());
}

