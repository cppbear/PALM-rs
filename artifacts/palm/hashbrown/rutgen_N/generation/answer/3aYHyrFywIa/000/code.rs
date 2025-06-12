// Answer 0

#[cfg(test)]
mod tests {
    use hashbrown::HashMap;
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::Entry;

    struct MyDefault;

    impl Default for MyDefault {
        fn default() -> Self {
            MyDefault
        }
    }

    #[test]
    fn test_or_default_vacant_entry() {
        let mut map: HashMap<&str, Option<MyDefault>> = HashMap::new();
        
        // Test with a nonexistent key
        let entry = map.entry("unknown_key");
        let value = entry.or_default();
        assert_eq!(*value, MyDefault::default());
    }

    #[test]
    fn test_or_default_occupied_entry() {
        let mut map: HashMap<&str, Option<u32>> = HashMap::new();
        
        // Insert a value
        map.insert("key_one", Some(1));

        // Test with an existing key
        let entry = map.entry("key_one");
        let value = entry.or_default();
        assert_eq!(*value, Some(1));
    }

    #[test]
    fn test_or_default_before_after_insert() {
        let mut map: HashMap<&str, Option<u32>> = HashMap::new();
        
        // Initially checking for a nonexistent key
        let entry = map.entry("key_two");
        let value = entry.or_default();
        assert_eq!(*value, None);
        
        // Now insert a value
        *value = Some(2);
        
        // Verify the inserted value
        assert_eq!(map.get("key_two"), Some(&Some(2)));
    }
}

