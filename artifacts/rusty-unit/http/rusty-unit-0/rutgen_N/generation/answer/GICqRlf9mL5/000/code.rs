// Answer 0

#[test]
fn test_append_new_value_to_entry() {
    struct HeaderMap {
        entries: Vec<Entry>,
    }

    struct Entry {
        index: usize,
        values: Vec<String>,
    }

    impl HeaderMap {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
            }
        }

        fn insert(&mut self, key: &str, value: String) {
            let entry = Entry {
                index: self.entries.len(),
                values: vec![value],
            };
            self.entries.push(entry);
        }

        fn entry(&mut self, key: &str) -> Option<&mut Entry> {
            self.entries.iter_mut().find(|e| e.index == 0) // Simplified for the test
        }

        fn get_all(&self, key: &str) -> Vec<String> {
            if self.entries.is_empty() {
                return Vec::new();
            }
            self.entries[0].values.clone() // Simplified for the test
        }
    }

    impl Entry {
        fn append(&mut self, value: String) {
            self.values.push(value);
        }
    }

    const HOST: &str = "host";

    let mut map = HeaderMap::new();
    map.insert(HOST, "world".to_string());

    if let Some(e) = map.entry(HOST) {
        e.append("earth".to_string());
    }

    let values = map.get_all(HOST);
    let mut i = values.iter();
    assert_eq!("world", *i.next().unwrap());
    assert_eq!("earth", *i.next().unwrap());
}

#[test]
fn test_append_empty_entry() {
    struct HeaderMap {
        entries: Vec<Entry>,
    }

    struct Entry {
        index: usize,
        values: Vec<String>,
    }

    impl HeaderMap {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
            }
        }

        fn insert(&mut self, key: &str, value: String) {
            let entry = Entry {
                index: self.entries.len(),
                values: vec![value],
            };
            self.entries.push(entry);
        }

        fn entry(&mut self, key: &str) -> Option<&mut Entry> {
            self.entries.iter_mut().find(|e| e.index == 0) // Simplified for the test
        }

        fn get_all(&self, key: &str) -> Vec<String> {
            if self.entries.is_empty() {
                return Vec::new();
            }
            self.entries[0].values.clone() // Simplified for the test
        }
    }

    impl Entry {
        fn append(&mut self, value: String) {
            self.values.push(value);
        }
    }

    const HOST: &str = "host";

    let mut map = HeaderMap::new();
    
    if let Some(e) = map.entry(HOST) {
        e.append("earth".to_string());
    }

    let values = map.get_all(HOST);
    let i = values.iter();
    assert!(i.clone().count() == 0); // Expect no values since none were inserted.
}

