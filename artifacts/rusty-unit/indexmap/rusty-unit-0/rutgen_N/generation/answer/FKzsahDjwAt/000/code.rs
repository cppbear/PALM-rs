// Answer 0

#[test]
fn test_insert() {
    use std::collections::hash_map::RandomState;
    use std::hash::{Hash, Hasher};
    
    struct MyMap<K, V> {
        data: indexmap::IndexMap<K, V, RandomState>,
        hash_builder: RandomState,
    }

    impl<K: Hash + Eq, V> MyMap<K, V> {
        pub fn new() -> Self {
            MyMap {
                data: indexmap::IndexMap::new(),
                hash_builder: RandomState::new(),
            }
        }

        pub fn insert(&mut self, key: K, value: V) -> (&mut K, &mut V) {
            let mut h = self.hash_builder.build_hasher();
            key.hash(&mut h);
            self.data.insert(key, value);
            let entry = self.data.get_mut(&key).expect("Failed to insert");
            (&mut entry.0, &mut entry.1)
        }
    }

    let mut my_map = MyMap::new();
    let key = "test_key";
    let value = 42;

    let (k_ref, v_ref) = my_map.insert(key, value);
    assert_eq!(*k_ref, key);
    assert_eq!(*v_ref, value);
}

#[test]
fn test_insert_overwrite() {
    use std::collections::hash_map::RandomState;
    use std::hash::{Hash, Hasher};

    struct MyMap<K, V> {
        data: indexmap::IndexMap<K, V, RandomState>,
        hash_builder: RandomState,
    }

    impl<K: Hash + Eq, V> MyMap<K, V> {
        pub fn new() -> Self {
            MyMap {
                data: indexmap::IndexMap::new(),
                hash_builder: RandomState::new(),
            }
        }

        pub fn insert(&mut self, key: K, value: V) -> (&mut K, &mut V) {
            let mut h = self.hash_builder.build_hasher();
            key.hash(&mut h);
            self.data.insert(key, value);
            let entry = self.data.get_mut(&key).expect("Failed to insert");
            (&mut entry.0, &mut entry.1)
        }
    }

    let mut my_map = MyMap::new();
    let key = "test_key";
    let value1 = 42;
    let value2 = 84;

    let (k_ref1, v_ref1) = my_map.insert(key, value1);
    assert_eq!(*k_ref1, key);
    assert_eq!(*v_ref1, value1);

    let (k_ref2, v_ref2) = my_map.insert(key, value2);
    assert_eq!(*k_ref2, key);
    assert_eq!(*v_ref2, value2);
    assert_eq!(*v_ref1, value2);
}

