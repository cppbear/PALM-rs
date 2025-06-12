// Answer 0

#[test]
fn test_get_full_mut_with_nonexistent_key() {
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};
    
    struct CustomKey {
        value: String,
    }
    
    impl PartialEq for CustomKey {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
        }
    }
    
    impl Eq for CustomKey {}
    
    impl Hash for CustomKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            state.write(self.value.as_bytes());
        }
    }
    
    struct CustomMap {
        entries: HashMap<CustomKey, usize>,
    }
    
    impl CustomMap {
        pub fn new() -> Self {
            Self {
                entries: HashMap::new(),
            }
        }
        
        pub fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
        where
            Q: ?Sized + Hash + Eq,
        {
            self.entries.keys().position(|k| k == key)
        }

        pub fn as_entries_mut(&mut self) -> Vec<(CustomKey, usize)> {
            self.entries.iter().map(|(k, v)| (k.clone(), *v)).collect()
        }

        pub fn get_full_mut<Q>(&mut self, key: &Q) -> Option<(usize, &CustomKey, &mut usize)>
        where
            Q: ?Sized + Hash + Eq,
        {
            if let Some(i) = self.get_index_of(key) {
                let entry = &mut self.as_entries_mut()[i];
                Some((i, &entry.0, &mut entry.1))
            } else {
                None
            }
        }
    }
    
    let mut map = CustomMap::new();
    map.entries.insert(CustomKey { value: String::from("key1") }, 42);

    let result = map.get_full_mut(&CustomKey { value: String::from("nonexistent_key") });
    assert_eq!(result, None);
}

