// Answer 0

#[test]
fn test_entry_with_valid_keys() {
    struct HeaderMap<T> {
        data: std::collections::HashMap<String, T>,
        capacity: usize,
        max_capacity: usize,
    }

    impl<T> HeaderMap<T> {
        fn default() -> Self {
            HeaderMap {
                data: std::collections::HashMap::new(),
                capacity: 0,
                max_capacity: 10,
            }
        }

        fn entry<K>(&mut self, key: K) -> &mut T
        where
            K: Into<String>,
        {
            let key = key.into();
            if self.capacity < self.max_capacity {
                self.capacity += 1;
                self.data.entry(key).or_insert(0)
            } else {
                panic!("size overflows MAX_SIZE");
            }
        }
    }

    let mut map: HeaderMap<u32> = HeaderMap::default();
    let headers = &["content-length", "x-hello", "Content-Length", "x-world"];

    for &header in headers {
        let counter = map.entry(header);
        *counter += 1;
    }

    assert_eq!(map.data["content-length"], 2);
    assert_eq!(map.data["x-hello"], 1);
}

#[should_panic]
#[test]
fn test_entry_exceeds_capacity() {
    struct HeaderMap<T> {
        data: std::collections::HashMap<String, T>,
        capacity: usize,
        max_capacity: usize,
    }

    impl<T> HeaderMap<T> {
        fn default() -> Self {
            HeaderMap {
                data: std::collections::HashMap::new(),
                capacity: 0,
                max_capacity: 2,
            }
        }

        fn entry<K>(&mut self, key: K) -> &mut T
        where
            K: Into<String>,
        {
            let key = key.into();
            if self.capacity < self.max_capacity {
                self.capacity += 1;
                self.data.entry(key).or_insert(0)
            } else {
                panic!("size overflows MAX_SIZE");
            }
        }
    }

    let mut map: HeaderMap<u32> = HeaderMap::default();
    let headers = &["key1", "key2", "key3"]; // This should cause panic

    for &header in headers {
        let _ = map.entry(header);
    }
}

#[test]
fn test_entry_with_repeated_keys() {
    struct HeaderMap<T> {
        data: std::collections::HashMap<String, T>,
        capacity: usize,
        max_capacity: usize,
    }

    impl<T> HeaderMap<T> {
        fn default() -> Self {
            HeaderMap {
                data: std::collections::HashMap::new(),
                capacity: 0,
                max_capacity: 5,
            }
        }

        fn entry<K>(&mut self, key: K) -> &mut T
        where
            K: Into<String>,
        {
            let key = key.into();
            if self.capacity < self.max_capacity {
                self.capacity += 1;
                self.data.entry(key).or_insert(0)
            } else {
                panic!("size overflows MAX_SIZE");
            }
        }
    }

    let mut map: HeaderMap<u32> = HeaderMap::default();
    let headers = &["key1", "key1", "key2", "key3", "key2"]; // keys have duplicates

    for &header in headers {
        let counter = map.entry(header);
        *counter += 1;
    }

    assert_eq!(map.data["key1"], 2);
    assert_eq!(map.data["key2"], 2);
    assert_eq!(map.data["key3"], 1);
}

