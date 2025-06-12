// Answer 0

#[test]
fn test_try_insert_vacant_entry() {
    struct MockHeaderMap {
        entries: Vec<(String, String)>,
        max_size: usize,
    }

    impl MockHeaderMap {
        fn new() -> Self {
            MockHeaderMap {
                entries: Vec::new(),
                max_size: 10,
            }
        }

        fn try_insert(&mut self, key: &str, value: String) -> Result<usize, ()> {
            if self.entries.len() >= self.max_size {
                return Err(());
            }
            self.entries.push((key.to_string(), value));
            Ok(self.entries.len() - 1)
        }
    }

    struct Entry<'a> {
        key: &'a str,
        map: &'a mut MockHeaderMap,
    }

    impl<'a> Entry<'a> {
        fn try_insert(self, value: String) -> Result<&'a mut String, ()> {
            let index = self.map.try_insert(self.key, value)?;
            Ok(&mut self.map.entries[index].1)
        }
    }

    let mut map = MockHeaderMap::new();
    let entry = Entry { key: "x-hello", map: &mut map };

    let inserted_value = entry.try_insert("world".to_string()).unwrap();
    assert_eq!(*inserted_value, "world");
    assert_eq!(map.entries.len(), 1);
    assert_eq!(map.entries[0].0, "x-hello");
}

#[test]
#[should_panic]
fn test_try_insert_max_size_reached() {
    struct MockHeaderMap {
        entries: Vec<(String, String)>,
        max_size: usize,
    }

    impl MockHeaderMap {
        fn new() -> Self {
            MockHeaderMap {
                entries: Vec::new(),
                max_size: 2,
            }
        }

        fn try_insert(&mut self, key: &str, value: String) -> Result<usize, ()> {
            if self.entries.len() >= self.max_size {
                return Err(());
            }
            self.entries.push((key.to_string(), value));
            Ok(self.entries.len() - 1)
        }
    }

    struct Entry<'a> {
        key: &'a str,
        map: &'a mut MockHeaderMap,
    }

    impl<'a> Entry<'a> {
        fn try_insert(self, value: String) -> Result<&'a mut String, ()> {
            let index = self.map.try_insert(self.key, value)?;
            Ok(&mut self.map.entries[index].1)
        }
    }

    let mut map = MockHeaderMap::new();
    let entry1 = Entry { key: "x-hello", map: &mut map };
    let _ = entry1.try_insert("world".to_string()).unwrap();
    
    let entry2 = Entry { key: "x-foo", map: &mut map };
    let _ = entry2.try_insert("bar".to_string()).unwrap();

    // This next insertion should panic due to max size being reached
    let entry3 = Entry { key: "x-baz", map: &mut map };
    let _ = entry3.try_insert("qux".to_string()).unwrap();
}

