// Answer 0

#[test]
fn test_or_insert_default_value_in_empty_entry() {
    struct HeaderMap<T> {
        entries: std::collections::HashMap<String, T>,
    }

    impl<T: Default + Copy> HeaderMap<T> {
        fn default() -> Self {
            HeaderMap {
                entries: std::collections::HashMap::new(),
            }
        }

        fn entry(&mut self, key: &str) -> Entry<T> {
            Entry { map: self, key: key.to_string() }
        }
    }

    struct Entry<'a, T> {
        map: &'a mut HeaderMap<T>,
        key: String,
    }

    impl<'a, T: Default + Copy> Entry<'a, T> {
        fn or_insert(self, default: T) -> &'a mut T {
            if self.map.entries.get(&self.key).is_none() {
                self.map.entries.insert(self.key.clone(), default);
            }
            self.map.entries.get_mut(&self.key).unwrap()
        }
    }

    let mut map: HeaderMap<u32> = HeaderMap::default();
    let value = map.entry("test-header").or_insert(0);
    *value += 1;

    assert_eq!(map.entries["test-header"], 1);
}

#[test]
#[should_panic(expected = "size overflows MAX_SIZE")]
fn test_or_insert_panic_on_capacity_exceed() {
    struct HeaderMap<T> {
        entries: std::collections::HashMap<String, T>,
        max_size: usize,
    }

    impl<T: Default + Copy> HeaderMap<T> {
        fn default() -> Self {
            HeaderMap {
                entries: std::collections::HashMap::new(),
                max_size: 2, // Assume max size is 2 for this test
            }
        }

        fn entry(&mut self, key: &str) -> Entry<T> {
            Entry { map: self, key: key.to_string() }
        }
    }

    struct Entry<'a, T> {
        map: &'a mut HeaderMap<T>,
        key: String,
    }

    impl<'a, T: Default + Copy> Entry<'a, T> {
        fn or_insert(self, default: T) -> &'a mut T {
            if self.map.entries.len() >= self.map.max_size {
                panic!("size overflows MAX_SIZE");
            }
            if self.map.entries.get(&self.key).is_none() {
                self.map.entries.insert(self.key.clone(), default);
            }
            self.map.entries.get_mut(&self.key).unwrap()
        }
    }

    let mut map: HeaderMap<u32> = HeaderMap::default();
    map.entry("header1").or_insert(0);
    map.entry("header2").or_insert(0);
    // This should cause a panic since max size is 2
    map.entry("header3").or_insert(0);
}

