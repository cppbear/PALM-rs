// Answer 0

#[test]
fn test_shift_remove_full_success() {
    #[derive(Debug, PartialEq)]
    struct Key(usize);
    #[derive(Debug, PartialEq)]
    struct Value(String);
    
    impl Equivalent<Key> for Key {
        fn equivalent(&self, other: &Key) -> bool {
            self.0 == other.0
        }
    }

    let mut map = IndexMapCore::new();
    let hash_value = HashValue(1);
    let key1 = Key(1);
    let value1 = Value("value1".to_string());
    let key2 = Key(2);
    let value2 = Value("value2".to_string());

    // Push some data into the map
    map.push_entry(hash_value, key1.clone(), value1.clone());
    map.push_entry(hash_value, key2.clone(), value2.clone());

    // Expect to successfully remove the first entry
    let result = map.shift_remove_full(hash_value, &key1);
    assert_eq!(result, Some((0, key1, value1)));
}

#[test]
fn test_shift_remove_full_not_found() {
    #[derive(Debug, PartialEq)]
    struct Key(usize);
    
    impl Equivalent<Key> for Key {
        fn equivalent(&self, other: &Key) -> bool {
            self.0 == other.0
        }
    }

    let mut map = IndexMapCore::new();
    let hash_value = HashValue(1);
    let key1 = Key(1);
    let value1 = Value("value1".to_string());

    // Push some data into the map
    map.push_entry(hash_value, key1.clone(), value1.clone());

    // Attempt to remove a non-existing key
    let key_not_found = Key(3);
    let result = map.shift_remove_full(hash_value, &key_not_found);
    assert_eq!(result, None);
}

