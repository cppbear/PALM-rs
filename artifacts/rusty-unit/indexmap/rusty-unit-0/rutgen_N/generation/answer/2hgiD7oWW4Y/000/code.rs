// Answer 0

#[test]
fn test_get_mut_existing_key() {
    use indexmap::IndexMap;
    use std::hash::{Hash, Hasher};
    
    struct Key {
        id: usize,
    }
    
    impl Hash for Key {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id.hash(state);
        }
    }
    
    impl PartialEq for Key {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }
    
    impl Eq for Key {}

    let mut map: IndexMap<Key, i32> = IndexMap::new();
    
    map.insert(Key { id: 1 }, 10);
    
    let value = map.get_mut(&Key { id: 1 });
    assert!(value.is_some());
    assert_eq!(*value.unwrap(), 10);
}

#[test]
fn test_get_mut_non_existing_key() {
    use indexmap::IndexMap;
    
    struct Key {
        id: usize,
    }
    
    impl PartialEq for Key {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }
    
    impl Eq for Key {}

    let mut map: IndexMap<Key, i32> = IndexMap::new();
    
    map.insert(Key { id: 1 }, 10);
    
    let value = map.get_mut(&Key { id: 2 });
    assert!(value.is_none());
}

