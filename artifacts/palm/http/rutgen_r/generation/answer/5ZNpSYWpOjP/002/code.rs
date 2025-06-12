// Answer 0

#[test]
fn test_or_try_insert_occupied() {
    struct HeaderMap<T> {
        entries: std::collections::HashMap<String, T>,
    }

    impl<T> Default for HeaderMap<T> {
        fn default() -> Self {
            HeaderMap {
                entries: std::collections::HashMap::new(),
            }
        }
    }

    enum Entry<'a, T> {
        Occupied(OccupiedEntry<'a, T>),
        Vacant,
    }

    struct OccupiedEntry<'a, T> {
        key: String,
        value: &'a mut T,
    }

    impl<'a, T> OccupiedEntry<'a, T> {
        fn into_mut(self) -> &'a mut T {
            self.value
        }
    }

    impl<T> HeaderMap<T> {
        fn entry(&mut self, key: &str) -> Entry<T> {
            if let Some(value) = self.entries.get_mut(key) {
                Entry::Occupied(OccupiedEntry {
                    key: key.to_string(),
                    value,
                })
            } else {
                Entry::Vacant
            }
        }
        
        fn insert(&mut self, key: String, value: T) {
            self.entries.insert(key, value);
        }
    }

    impl<'a, T> Entry<'a, T> {
        fn or_try_insert(self, default: T) -> Result<&'a mut T, &'static str> {
            match self {
                Entry::Occupied(e) => Ok(e.into_mut()),
                Entry::Vacant => Err("Vacant entry"),
            }
        }
    }

    let mut map: HeaderMap<u32> = HeaderMap::default();
    map.insert("content-length".to_string(), 1);

    let entry = map.entry("content-length");
    let result = entry.or_try_insert(0).unwrap();

    *result += 1; // Incrementing the value associated with "content-length"

    assert_eq!(map.entries["content-length"], 2);
}

