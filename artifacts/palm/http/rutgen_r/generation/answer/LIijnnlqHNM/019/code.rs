// Answer 0

#[test]
fn test_try_insert2_vacant() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct HeaderName(String);
    
    impl Hash for HeaderName {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }
    
    impl PartialEq<HeaderName> for HeaderName {
        fn eq(&self, other: &HeaderName) -> bool {
            self.0 == other.0
        }
    }

    struct MaxSizeReached;

    struct Map<T> {
        entries: Vec<(HeaderName, T)>,
        indices: Vec<Option<usize>>,
        max_size: usize,
    }

    impl<T> Map<T> {
        fn new(max_size: usize) -> Self {
            Self {
                entries: Vec::new(),
                indices: vec![None; max_size],
                max_size,
            }
        }

        fn try_reserve_one(&mut self) -> Result<(), MaxSizeReached> {
            if self.entries.len() < self.max_size {
                Ok(())
            } else {
                Err(MaxSizeReached)
            }
        }

        fn try_insert_entry(&mut self, _: usize, key: HeaderName, value: T) -> Result<(), MaxSizeReached> {
            self.entries.push((key, value));
            Ok(())
        }

        fn try_insert2<K>(&mut self, key: K, value: T) -> Result<Option<T>, MaxSizeReached>
        where
            K: Hash + Into<HeaderName>,
            HeaderName: PartialEq<K>,
        {
            self.try_reserve_one()?;

            let probe = 0;
            let pos = 0;
            let hash = 0;
            let danger = 0;

            Ok(Some(self.insert_occupied(pos, value)))
        }

        fn insert_occupied(&mut self, pos: usize, value: T) -> T {
            self.entries[pos].1 = value;
            value
        }
    }

    let mut map = Map::new(10);
    let key = HeaderName("test_key".to_string());
    let value = "test_value";

    let result = map.try_insert2(key.clone(), value).unwrap();

    assert!(result.is_none());
    assert_eq!(map.entries.len(), 1);
    assert_eq!(map.entries[0].0, key);
}

#[test]
fn test_try_insert2_occupied() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct HeaderName(String);
    
    impl Hash for HeaderName {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }
    
    impl PartialEq<HeaderName> for HeaderName {
        fn eq(&self, other: &HeaderName) -> bool {
            self.0 == other.0
        }
    }

    struct MaxSizeReached;

    struct Map<T> {
        entries: Vec<(HeaderName, T)>,
        indices: Vec<Option<usize>>,
        max_size: usize,
    }

    impl<T> Map<T> {
        fn new(max_size: usize) -> Self {
            Self {
                entries: Vec::new(),
                indices: vec![None; max_size],
                max_size,
            }
        }

        fn try_reserve_one(&mut self) -> Result<(), MaxSizeReached> {
            if self.entries.len() < self.max_size {
                Ok(())
            } else {
                Err(MaxSizeReached)
            }
        }

        fn try_insert_entry(&mut self, _: usize, key: HeaderName, value: T) -> Result<(), MaxSizeReached> {
            self.entries.push((key, value));
            Ok(())
        }

        fn try_insert2<K>(&mut self, key: K, value: T) -> Result<Option<T>, MaxSizeReached>
        where
            K: Hash + Into<HeaderName>,
            HeaderName: PartialEq<K>,
        {
            self.try_reserve_one()?;
            let hash = 0;
            let probe = 0;
            let danger = 0;
            let existing_value = T::default(); // Simulating existing occupied value

            self.insert_occupied(probe, existing_value);
            Ok(Some(self.insert_occupied(probe, value)))
        }

        fn insert_occupied(&mut self, pos: usize, value: T) -> T {
            if pos < self.entries.len() {
                self.entries[pos].1 = value;
            }
            value
        }
    }

    let mut map = Map::new(10);
    let key = HeaderName("test_key".to_string());
    let value = "new_test_value";

    map.try_insert2(key.clone(), "initial_value").unwrap();
    let result = map.try_insert2(key.clone(), value);

    assert!(result.is_ok());
    assert_eq!(map.entries.len(), 1);
    assert_eq!(map.entries[0].1, value);
}

