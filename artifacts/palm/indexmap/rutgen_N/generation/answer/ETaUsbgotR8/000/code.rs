// Answer 0

#[test]
fn test_get_full_mut2_existing_key() {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    use indexmap::IndexMap;

    struct TestKey(usize);

    impl PartialEq for TestKey {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for TestKey {}

    impl Hash for TestKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    let mut map = IndexMap::new();
    map.insert(TestKey(1), "value1");
    
    let key = TestKey(1);
    let result = map.get_full_mut2(&key);

    assert!(result.is_some());
    let (index, key_ref, value_ref) = result.unwrap();
    assert_eq!(index, 0);
    assert_eq!(*key_ref, TestKey(1));
    assert_eq!(*value_ref, "value1");
}

#[test]
fn test_get_full_mut2_non_existing_key() {
    use std::hash::{Hash, Hasher};
    use indexmap::IndexMap;

    struct TestKey(usize);

    impl PartialEq for TestKey {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for TestKey {}

    impl Hash for TestKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    let mut map = IndexMap::new();
    map.insert(TestKey(1), "value1");
    
    let key = TestKey(2);
    let result = map.get_full_mut2(&key);

    assert!(result.is_none());
}

