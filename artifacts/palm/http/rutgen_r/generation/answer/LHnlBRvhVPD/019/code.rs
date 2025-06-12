// Answer 0

#[test]
fn test_try_append2_vacant_entry() {
    use std::collections::HashSet;
    use std::hash::Hash;

    struct HeaderName(String);
    struct MyMap<K, V> {
        entries: Vec<(K, V)>,
        indices: Vec<Option<usize>>,
        extra_values: Vec<V>,
    }

    impl<K, V> MyMap<K, V>
    where
        K: Hash + Eq,
    {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
                indices: Vec::new(),
                extra_values: Vec::new(),
            }
        }

        fn try_reserve_one(&mut self) -> Result<(), &'static str> {
            if self.entries.len() < 10 {
                self.entries.push((Default::default(), Default::default()));
                Ok(())
            } else {
                Err("MaxSizeReached")
            }
        }

        fn try_insert_entry(&mut self, _hash: usize, key: K, value: V) -> Result<(), &'static str> {
            self.entries.push((key, value));
            Ok(())
        }
    }

    impl<K: Hash + Into<HeaderName> + Default, V> MyMap<K, V> {
        fn try_append2(&mut self, key: K, value: V) -> Result<bool, &'static str> {
            self.try_reserve_one()?;

            let index = self.entries.len();
            self.try_insert_entry(0, key, value)?;

            self.indices.push(Some(index));
            Ok(false)
        }
    }

    let mut map = MyMap::new();
    let key = HeaderName("key1".to_string());
    let value = "value1";
    
    let result = map.try_append2(key, value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
}

#[test]
fn test_try_append2_occupied_entry() {
    use std::collections::HashSet;
    use std::hash::Hash;

    struct HeaderName(String);
    struct MyMap<K, V> {
        entries: Vec<(K, V)>,
        indices: Vec<Option<usize>>,
        extra_values: Vec<V>,
    }

    impl<K, V> MyMap<K, V>
    where
        K: Hash + Eq,
    {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
                indices: Vec::new(),
                extra_values: Vec::new(),
            }
        }

        fn try_reserve_one(&mut self) -> Result<(), &'static str> {
            if self.entries.len() < 10 {
                self.entries.push((Default::default(), Default::default()));
                Ok(())
            } else {
                Err("MaxSizeReached")
            }
        }

        fn try_insert_entry(&mut self, _hash: usize, key: K, value: V) -> Result<(), &'static str> {
            self.entries.push((key, value));
            Ok(())
        }
    }

    impl<K: Hash + Into<HeaderName> + Default, V> MyMap<K, V> {
        fn try_append2(&mut self, key: K, value: V) -> Result<bool, &'static str> {
            self.try_reserve_one()?;

            let pos = 0; // Assume we find an occupied entry at position 0
            let index = self.entries.len();
            let existing_key = Default::default(); // Simulating existing key
            
            if existing_key == key {
                // Simulating appending a value to an occupied entry
                self.extra_values.push(value);
                return Ok(true);
            }

            self.try_insert_entry(0, key, value)?;
            self.indices.push(Some(index));
            Ok(false)
        }
    }

    let mut map = MyMap::new();
    let key1 = HeaderName("key1".to_string());
    let value1 = "value1";
    let key2 = HeaderName("key1".to_string());
    let value2 = "value2";

    let _ = map.try_append2(key1, value1); // Insert first entry
    let result = map.try_append2(key2, value2); // Attempt to append to occupied entry
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

