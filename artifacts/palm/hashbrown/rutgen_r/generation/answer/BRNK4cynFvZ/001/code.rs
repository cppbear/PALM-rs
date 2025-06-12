// Answer 0

#[cfg(test)]
mod tests {
    use hashbrown::HashMap;
    use std::collections::hash_map::Entry;

    #[test]
    fn test_or_insert_with_key_vacant_entry() {
        let mut map: HashMap<&str, usize> = HashMap::new();
        
        let entry = map.entry("poneyland");
        let value = entry.or_insert_with_key(|key| key.chars().count());
        
        assert_eq!(*value, 9);
        assert_eq!(map["poneyland"], 9);
    }

    #[test]
    fn test_or_insert_with_key_existing_entry() {
        let mut map: HashMap<&str, usize> = HashMap::new();
        map.insert("poneyland", 9);
        
        let entry = map.entry("poneyland");
        let value = entry.or_insert_with_key(|key| key.chars().count() * 10);
        
        assert_eq!(*value, 9);
        assert_eq!(map["poneyland"], 9);
    }

    #[test]
    #[should_panic]
    fn test_or_insert_with_key_panic_on_closure() {
        let mut map: HashMap<&str, usize> = HashMap::new();
        
        // Attempt to use a key that causes a panic (e.g., trying to access an index of an empty string)
        let entry = map.entry("invalid_key");
        entry.or_insert_with_key(|key| {
            let _ = key.chars().nth(10); // This will panic
            0
        });
    }
}

