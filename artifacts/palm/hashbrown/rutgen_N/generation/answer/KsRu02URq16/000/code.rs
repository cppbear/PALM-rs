// Answer 0

#[test]
fn test_get_many_mut_inner_with_matching_keys() {
    use hashbrown::HashMap;
    use std::hash::{Hash, Hasher};

    struct Key {
        value: i32,
    }

    impl Hash for Key {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.value.hash(state);
        }
    }

    struct Value {
        data: String,
    }

    let mut map: HashMap<Key, Value> = HashMap::new();
    map.insert(Key { value: 1 }, Value { data: "one".to_string() });
    map.insert(Key { value: 2 }, Value { data: "two".to_string() });

    let keys = [&Key { value: 1 }, &Key { value: 2 }];
    let result: [Option<&mut (Key, Value)>; 2] = map.get_many_mut_inner(&keys);

    assert!(result[0].is_some());
    assert!(result[1].is_some());

    if let Some((k, v)) = result[0] {
        assert_eq!(k.value, 1);
        assert_eq!(v.data, "one");
    }

    if let Some((k, v)) = result[1] {
        assert_eq!(k.value, 2);
        assert_eq!(v.data, "two");
    }
}

#[test]
fn test_get_many_mut_inner_with_non_matching_keys() {
    use hashbrown::HashMap;
    use std::hash::{Hash, Hasher};

    struct Key {
        value: i32,
    }

    impl Hash for Key {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.value.hash(state);
        }
    }

    struct Value {
        data: String,
    }

    let mut map: HashMap<Key, Value> = HashMap::new();
    map.insert(Key { value: 1 }, Value { data: "one".to_string() });
    map.insert(Key { value: 2 }, Value { data: "two".to_string() });

    let keys = [&Key { value: 3 }, &Key { value: 4 }];
    let result: [Option<&mut (Key, Value)>; 2] = map.get_many_mut_inner(&keys);

    assert!(result[0].is_none());
    assert!(result[1].is_none());
}

#[should_panic]
#[test]
fn test_get_many_mut_inner_with_empty_keys() {
    use hashbrown::HashMap;
    use std::hash::{Hash, Hasher};

    struct Key {
        value: i32,
    }

    impl Hash for Key {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.value.hash(state);
        }
    }

    struct Value {
        data: String,
    }

    let mut map: HashMap<Key, Value> = HashMap::new();
    
    let keys: [&Key; 0] = [];
    let _result: [Option<&mut (Key, Value)>; 0] = map.get_many_mut_inner(&keys);
}

