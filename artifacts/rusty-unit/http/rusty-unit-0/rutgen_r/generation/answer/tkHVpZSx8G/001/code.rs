// Answer 0

#[test]
fn test_or_insert_with_default_when_empty() {
    struct HeaderMap<T> {
        entries: std::collections::HashMap<String, T>,
        capacity: usize,
    }

    impl<T: Default + Copy> HeaderMap<T> {
        fn default() -> Self {
            Self {
                entries: std::collections::HashMap::new(),
                capacity: 10,
            }
        }

        fn entry(&mut self, key: &str) -> &mut T {
            if self.entries.len() >= self.capacity {
                panic!("size overflows MAX_SIZE");
            }
            self.entries.entry(key.to_string()).or_insert_with(T::default)
        }
    }

    let mut map: HeaderMap<u32> = HeaderMap::default();
    let header_key = "content-length";
    
    let counter = map.entry(header_key).or_insert(0);
    *counter += 1;

    assert_eq!(map.entries[header_key], 1);
}

#[test]
fn test_or_insert_with_default_when_present() {
    struct HeaderMap<T> {
        entries: std::collections::HashMap<String, T>,
        capacity: usize,
    }

    impl<T: Default + Copy> HeaderMap<T> {
        fn default() -> Self {
            Self {
                entries: std::collections::HashMap::new(),
                capacity: 10,
            }
        }

        fn entry(&mut self, key: &str) -> &mut T {
            if self.entries.len() >= self.capacity {
                panic!("size overflows MAX_SIZE");
            }
            self.entries.entry(key.to_string()).or_insert_with(T::default)
        }
    }

    let mut map: HeaderMap<u32> = HeaderMap::default();
    let header_key = "x-hello";
    
    let counter = map.entry(header_key).or_insert(0);
    *counter += 2;

    let new_counter = map.entry(header_key).or_insert(0);
    *new_counter += 3;

    assert_eq!(map.entries[header_key], 5);
}

#[should_panic(expected = "size overflows MAX_SIZE")]
#[test]
fn test_or_insert_panics_on_max_size_exceeded() {
    struct HeaderMap<T> {
        entries: std::collections::HashMap<String, T>,
        capacity: usize,
    }

    impl<T: Default + Copy> HeaderMap<T> {
        fn default() -> Self {
            Self {
                entries: std::collections::HashMap::new(),
                capacity: 10,
            }
        }

        fn entry(&mut self, key: &str) -> &mut T {
            if self.entries.len() >= self.capacity {
                panic!("size overflows MAX_SIZE");
            }
            self.entries.entry(key.to_string()).or_insert_with(T::default)
        }
    }

    let mut map: HeaderMap<u32> = HeaderMap::default();
    
    for i in 0..15 {
        let header_key = format!("header-{}", i);
        map.entry(&header_key).or_insert(0);
    }
}

