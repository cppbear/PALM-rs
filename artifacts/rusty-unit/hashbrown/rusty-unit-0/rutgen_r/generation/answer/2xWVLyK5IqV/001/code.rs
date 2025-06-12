// Answer 0

#[test]
fn test_or_insert_with_key_vacant_entry() {
    use hashbrown::HashMap;
    use std::collections::hash_map::Entry;

    struct Wrapper<'a, K, V> {
        map: HashMap<K, V>,
        key: &'a K,
    }

    impl<'a, K, V> Wrapper<'a, K, V>
    where
        K: std::hash::Hash + std::cmp::Eq + Copy + 'a,
        V: Default,
    {
        fn new(key: &'a K) -> Self {
            Self {
                map: HashMap::new(),
                key,
            }
        }

        fn entry_ref(&mut self) -> Entry<&K, V> {
            self.map.entry(*self.key)
        }
    }

    let key: String = "test_entry".to_string();
    let mut wrapper = Wrapper::new(&key);

    // Test inserting with a closure that counts the number of characters in the key.
    let value = wrapper.entry_ref().or_insert_with_key(|k| k.len());
    assert_eq!(*value, key.len()); // key is "test_entry", which has length 11
}

#[test]
fn test_or_insert_with_key_vacant_entry_multiple_entries() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, usize> = HashMap::new();

    // Insert for a vacant entry "poneyland"
    let value = map.entry("poneyland".to_string()).or_insert_with_key(|key| key.chars().count());
    assert_eq!(*value, 9); // "poneyland" has 9 characters

    // Now insert for another vacant entry "unicorn"
    let value = map.entry("unicorn".to_string()).or_insert_with_key(|key| key.chars().count());
    assert_eq!(*value, 7); // "unicorn" has 7 characters
}

