// Answer 0

#[test]
fn test_or_try_insert_with_empty_entry() {
    struct HeaderMap<T> {
        map: std::collections::HashMap<String, T>,
        max_capacity: usize,
    }

    impl<T: Default> HeaderMap<T> {
        fn default() -> Self {
            HeaderMap {
                map: std::collections::HashMap::new(),
                max_capacity: 5, // Arbitrary max capacity for testing
            }
        }

        fn entry(&mut self, key: &str) -> Entry<T> {
            if self.map.contains_key(key) {
                Entry::Occupied(key.to_string(), self.map.get_mut(key).unwrap())
            } else {
                Entry::Vacant(key.to_string(), self)
            }
        }
    }

    enum Entry<T> {
        Occupied(String, *mut T),
        Vacant(String, *mut HeaderMap<T>),
    }

    impl<T: Default> Entry<T> {
        fn or_try_insert(self, default: T) -> Result<&mut T, &'static str> {
            match self {
                Entry::Occupied(_, value) => Ok(value),
                Entry::Vacant(key, header_map) => {
                    if header_map.map.len() < header_map.max_capacity {
                        header_map.map.insert(key, default);
                        Ok(header_map.map.get_mut(&key).unwrap())
                    } else {
                        Err("MaxSizeReached")
                    }
                }
            }
        }
    }

    let mut map: HeaderMap<u32> = HeaderMap::default();
    let counter = map.entry("x-hello").or_try_insert(0).unwrap();
    *counter += 1;
    assert_eq!(map.map["x-hello"], 1);
}

#[test]
fn test_or_try_insert_with_occupied_entry() {
    struct HeaderMap<T> {
        map: std::collections::HashMap<String, T>,
        max_capacity: usize,
    }

    impl<T: Default> HeaderMap<T> {
        fn default() -> Self {
            HeaderMap {
                map: std::collections::HashMap::new(),
                max_capacity: 5,
            }
        }

        fn entry(&mut self, key: &str) -> Entry<T> {
            if self.map.contains_key(key) {
                Entry::Occupied(key.to_string(), self.map.get_mut(key).unwrap())
            } else {
                Entry::Vacant(key.to_string(), self)
            }
        }
    }

    enum Entry<T> {
        Occupied(String, *mut T),
        Vacant(String, *mut HeaderMap<T>),
    }

    impl<T: Default> Entry<T> {
        fn or_try_insert(self, default: T) -> Result<&mut T, &'static str> {
            match self {
                Entry::Occupied(_, value) => Ok(value),
                Entry::Vacant(key, header_map) => {
                    if header_map.map.len() < header_map.max_capacity {
                        header_map.map.insert(key, default);
                        Ok(header_map.map.get_mut(&key).unwrap())
                    } else {
                        Err("MaxSizeReached")
                    }
                }
            }
        }
    }

    let mut map: HeaderMap<u32> = HeaderMap::default();
    map.map.insert("x-hello".to_string(), 1);
    let counter = map.entry("x-hello").or_try_insert(0).unwrap();
    *counter += 1;
    assert_eq!(map.map["x-hello"], 2);
}

#[test]
#[should_panic]
fn test_or_try_insert_reaches_max_capacity() {
    struct HeaderMap<T> {
        map: std::collections::HashMap<String, T>,
        max_capacity: usize,
    }

    impl<T: Default> HeaderMap<T> {
        fn default() -> Self {
            HeaderMap {
                map: std::collections::HashMap::new(),
                max_capacity: 2, // Small max capacity for this test
            }
        }

        fn entry(&mut self, key: &str) -> Entry<T> {
            if self.map.contains_key(key) {
                Entry::Occupied(key.to_string(), self.map.get_mut(key).unwrap())
            } else {
                Entry::Vacant(key.to_string(), self)
            }
        }
    }

    enum Entry<T> {
        Occupied(String, *mut T),
        Vacant(String, *mut HeaderMap<T>),
    }

    impl<T: Default> Entry<T> {
        fn or_try_insert(self, default: T) -> Result<&mut T, &'static str> {
            match self {
                Entry::Occupied(_, value) => Ok(value),
                Entry::Vacant(key, header_map) => {
                    if header_map.map.len() < header_map.max_capacity {
                        header_map.map.insert(key, default);
                        Ok(header_map.map.get_mut(&key).unwrap())
                    } else {
                        panic!("MaxSizeReached");
                    }
                }
            }
        }
    }

    let mut map: HeaderMap<u32> = HeaderMap::default();
    map.entry("x-hello").or_try_insert(0).unwrap(); // 1st insert
    map.entry("x-world").or_try_insert(0).unwrap(); // 2nd insert
    map.entry("x-another").or_try_insert(0).unwrap(); // Should panic
}

