// Answer 0

#[test]
fn test_or_try_insert_vacant() {
    struct HeaderMap<T> {
        entries: std::collections::HashMap<String, T>,
        max_capacity: usize,
    }

    impl<T> Default for HeaderMap<T> {
        fn default() -> Self {
            HeaderMap {
                entries: std::collections::HashMap::new(),
                max_capacity: 5,
            }
        }
    }

    impl<T: std::fmt::Debug + Copy + Default> HeaderMap<T> {
        pub fn entry(&mut self, key: &str) -> Entry<T> {
            if self.entries.len() < self.max_capacity {
                Entry::Vacant {
                    key: key.to_string(),
                    map: self,
                }
            } else {
                panic!("Max size reached");
            }
        }
    }

    enum Entry<T> {
        Occupied { key: String, value: T },
        Vacant { key: String, map: *mut HeaderMap<T> },
    }

    impl<T: Default> Entry<T> {
        pub fn try_insert(self, default: T) -> Result<&'static mut T, ()> {
            if let Entry::Vacant { key, map } = self {
                unsafe {
                    let map = &mut *map;
                    map.entries.insert(key, default);
                    Ok(map.entries.get_mut(&key).unwrap())
                }
            } else {
                panic!("Entry already occupied");
            }
        }
    }

    let mut map: HeaderMap<u32> = HeaderMap::default();
    let header = "x-header";

    let counter = map.entry(header)
        .try_insert(0)
        .unwrap();
    
    *counter += 1;

    assert_eq!(map.entries[header], 1);
}

#[test]
#[should_panic(expected = "Max size reached")]
fn test_or_try_insert_max_size_reached() {
    struct HeaderMap<T> {
        entries: std::collections::HashMap<String, T>,
        max_capacity: usize,
    }

    impl<T> Default for HeaderMap<T> {
        fn default() -> Self {
            HeaderMap {
                entries: std::collections::HashMap::new(),
                max_capacity: 2,
            }
        }
    }

    impl<T> HeaderMap<T> {
        pub fn entry(&mut self, key: &str) -> Entry<T> {
            if self.entries.len() < self.max_capacity {
                Entry::Vacant {
                    key: key.to_string(),
                    map: self,
                }
            } else {
                panic!("Max size reached");
            }
        }
    }

    enum Entry<T> {
        Occupied { key: String, value: T },
        Vacant { key: String, map: *mut HeaderMap<T> },
    }

    impl<T: Default> Entry<T> {
        pub fn try_insert(self, default: T) -> Result<&'static mut T, ()> {
            if let Entry::Vacant { key, map } = self {
                unsafe {
                    let map = &mut *map;
                    map.entries.insert(key, default);
                    Ok(map.entries.get_mut(&key).unwrap())
                }
            } else {
                panic!("Entry already occupied");
            }
        }
    }

    let mut map: HeaderMap<u32> = HeaderMap::default();
    let headers = ["header1", "header2", "header3"];

    for &header in &headers {
        map.entry(header)
            .try_insert(0)
            .unwrap();
    }
}

