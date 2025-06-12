// Answer 0

#[test]
fn test_entry_occupied() {
    struct DummyIndices {
        entries: Vec<Option<usize>>,
    }

    impl DummyIndices {
        fn find_entry(&self, _hash: u64, eq: impl Fn(&usize) -> bool) -> Result<usize, DummyAbsent> {
            for (i, entry) in self.entries.iter().enumerate() {
                if let Some(_) = entry {
                    if eq(&i) {
                        return Ok(i);
                    }
                }
            }
            Err(DummyAbsent)
        }
    }

    struct DummyEntries {
        buckets: Vec<Bucket<String, i32>>,
    }

    impl DummyEntries {
        fn as_entries(&self) -> &[Bucket<String, i32>] {
            &self.buckets
        }

        fn as_entries_mut(&mut self) -> &mut [Bucket<String, i32>] {
            &mut self.buckets
        }
    }

    struct DummyAbsent;

    let mut indices = DummyIndices {
        entries: vec![Some(0), None, Some(2)],
    };
    let mut entries = DummyEntries {
        buckets: vec![
            Bucket { hash: HashValue(10), key: "key1".to_string(), value: 1 },
            Bucket { hash: HashValue(20), key: "key2".to_string(), value: 2 },
            Bucket { hash: HashValue(30), key: "key3".to_string(), value: 3 },
        ],
    };
    
    let mut map = IndexMapCore {
        indices,
        entries,
    };

    let hash = HashValue(10);
    let key = "key1".to_string();

    let entry = map.entry(hash, key);
    match entry {
        Entry::Occupied(_) => {},
        _ => panic!("Expected an Occupied entry"),
    }
}

