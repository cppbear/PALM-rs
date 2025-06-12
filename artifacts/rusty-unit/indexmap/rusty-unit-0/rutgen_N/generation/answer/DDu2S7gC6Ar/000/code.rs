// Answer 0

#[test]
fn test_replace_full_insert_new_entry() {
    struct TestEntry {
        entries: Vec<(String, i32)>,
        indices: HashMap<u64, usize>,
    }

    impl TestEntry {
        fn new() -> Self {
            TestEntry {
                entries: Vec::new(),
                indices: HashMap::new(),
            }
        }

        fn replace_full(
            &mut self,
            hash: u64,
            key: String,
            value: i32,
        ) -> (usize, Option<(String, i32)>) {
            let eq = self.entries.iter().position(|(k, _)| k == &key);
            let result = match eq {
                Some(i) => {
                    let kv = (self.entries[i].0.clone(), self.entries[i].1);
                    self.entries[i] = (key, value);
                    (i, Some(kv))
                }
                None => {
                    let i = self.entries.len();
                    self.indices.insert(hash, i);
                    self.entries.push((key, value));
                    (i, None)
                }
            };
            result
        }
    }

    let mut test_entry = TestEntry::new();
    let (index, old_value) = test_entry.replace_full(1_u64, String::from("key1"), 100);
    assert_eq!(index, 0);
    assert_eq!(old_value, None);
    assert_eq!(test_entry.entries.len(), 1);
    assert_eq!(test_entry.entries[0], (String::from("key1"), 100));
}

#[test]
fn test_replace_full_update_existing_entry() {
    struct TestEntry {
        entries: Vec<(String, i32)>,
        indices: HashMap<u64, usize>,
    }

    impl TestEntry {
        fn new() -> Self {
            TestEntry {
                entries: Vec::new(),
                indices: HashMap::new(),
            }
        }

        fn replace_full(
            &mut self,
            hash: u64,
            key: String,
            value: i32,
        ) -> (usize, Option<(String, i32)>) {
            let eq = self.entries.iter().position(|(k, _)| k == &key);
            let result = match eq {
                Some(i) => {
                    let kv = (self.entries[i].0.clone(), self.entries[i].1);
                    self.entries[i] = (key, value);
                    (i, Some(kv))
                }
                None => {
                    let i = self.entries.len();
                    self.indices.insert(hash, i);
                    self.entries.push((key, value));
                    (i, None)
                }
            };
            result
        }
    }

    let mut test_entry = TestEntry::new();
    let _ = test_entry.replace_full(1_u64, String::from("key1"), 100);
    let (index, old_value) = test_entry.replace_full(1_u64, String::from("key1"), 200);
    assert_eq!(index, 0);
    assert_eq!(old_value, Some((String::from("key1"), 100)));
    assert_eq!(test_entry.entries.len(), 1);
    assert_eq!(test_entry.entries[0], (String::from("key1"), 200));
}

