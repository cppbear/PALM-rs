// Answer 0

#[test]
fn test_try_entry2_should_return_err_when_try_reserve_one_fails() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct HeaderName(String);

    impl PartialEq<HeaderName> for HeaderName {
        fn eq(&self, other: &HeaderName) -> bool {
            self.0 == other.0
        }
    }

    struct Map {
        capacity: usize,
        size: usize,
    }

    #[derive(Debug)]
    struct MaxSizeReached;

    impl Map {
        fn new(capacity: usize) -> Self {
            Map {
                capacity,
                size: 0,
            }
        }

        fn try_reserve_one(&mut self) -> Result<(), MaxSizeReached> {
            if self.size >= self.capacity {
                Err(MaxSizeReached)
            } else {
                self.size += 1;
                Ok(())
            }
        }

        fn try_entry2<K>(&mut self, key: K) -> Result<Entry<'_, ()>, MaxSizeReached>
        where
            K: Hash + Into<HeaderName>,
            HeaderName: PartialEq<K>,
        {
            self.try_reserve_one()?;

            Ok(Entry::Vacant(VacantEntry {
                map: self,
                key: key.into(),
            }))
        }
    }

    enum Entry<'a, T> {
        Vacant(VacantEntry<'a, T>),
        Occupied(OccupiedEntry<'a, T>),
    }

    struct VacantEntry<'a, T> {
        map: &'a mut Map,
        key: HeaderName,
    }

    struct OccupiedEntry<'a, T> {
        map: &'a Map,
    }

    let mut map = Map::new(1);
    map.size = 1; // Force the map to be "full"

    let result = map.try_entry2(HeaderName("test".to_string()));
    assert!(result.is_err());
}

#[test]
#[should_panic(expected = "MaxSizeReached")]
fn test_try_entry2_should_panic_on_full_map() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct HeaderName(String);

    impl PartialEq<HeaderName> for HeaderName {
        fn eq(&self, other: &HeaderName) -> bool {
            self.0 == other.0
        }
    }

    struct Map {
        capacity: usize,
        size: usize,
    }

    #[derive(Debug)]
    struct MaxSizeReached;

    impl Map {
        fn new(capacity: usize) -> Self {
            Map {
                capacity,
                size: 0,
            }
        }

        fn try_reserve_one(&mut self) -> Result<(), MaxSizeReached> {
            if self.size >= self.capacity {
                return Err(MaxSizeReached);
            }
            self.size += 1;
            Ok(())
        }

        fn try_entry2<K>(&mut self, key: K) -> Result<Entry<'_, ()>, MaxSizeReached>
        where
            K: Hash + Into<HeaderName>,
            HeaderName: PartialEq<K>,
        {
            self.try_reserve_one()?;

            Ok(Entry::Vacant(VacantEntry {
                map: self,
                key: key.into(),
            }))
        }
    }

    enum Entry<'a, T> {
        Vacant(VacantEntry<'a, T>),
        Occupied(OccupiedEntry<'a, T>),
    }

    struct VacantEntry<'a, T> {
        map: &'a mut Map,
        key: HeaderName,
    }

    struct OccupiedEntry<'a, T> {
        map: &'a Map,
    }

    let mut map = Map::new(1);
    map.size = 1; // Force the map to be "full"

    let _ = map.try_entry2(HeaderName("test".to_string())); // Will panic
}

