// Answer 0

#[test]
fn test_remove_found() {
    struct TestHeaderValue;
    struct TestHeaderMap {
        mask: Size,
        indices: Box<[Pos]>,
        entries: Vec<Bucket<TestHeaderValue>>,
        extra_values: Vec<ExtraValue<TestHeaderValue>>,
        danger: Danger,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            Self {
                mask: 15,
                indices: Box::new([Pos::none(); 16]),
                entries: Vec::new(),
                extra_values: Vec::new(),
                danger: Danger::Green,
            }
        }

        fn add_entry(&mut self, hash: HashValue, key: HeaderName, value: TestHeaderValue) {
            let entry = Bucket {
                hash,
                key,
                value,
                links: None,
            };
            self.entries.push(entry);
            let pos = self.entries.len() - 1;
            self.indices[pos] = Pos::new(pos, hash);
        }

        fn remove_found(&mut self, probe: usize, found: usize) -> Bucket<TestHeaderValue> {
            // implementation of the remove_found function goes here
            // (the one provided in the context)
            self.indices[probe] = Pos::none();
            let entry = self.entries.swap_remove(found);

            // correct index that points to the entry that had to swap places
            if let Some(entry) = self.entries.get(found) {
                let mut probe = desired_pos(self.mask, entry.hash);
                
                probe_loop!(probe < self.indices.len(), {
                    if let Some((i, _)) = self.indices[probe].resolve() {
                        if i >= self.entries.len() {
                            self.indices[probe] = Pos::new(found, entry.hash);
                            break;
                        }
                    }
                });

                // Update links
                if let Some(links) = entry.links {
                    self.extra_values[links.next].prev = Link::Entry(found);
                    self.extra_values[links.tail].next = Link::Entry(found);
                }
            }

            if !self.entries.is_empty() {
                let mut last_probe = probe;
                let mut probe = probe + 1;

                probe_loop!(probe < self.indices.len(), {
                    if let Some((_, entry_hash)) = self.indices[probe].resolve() {
                        if probe_distance(self.mask, entry_hash, probe) > 0 {
                            self.indices[last_probe] = self.indices[probe];
                            self.indices[probe] = Pos::none();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }

                    last_probe = probe;
                });
            }

            entry
        }
    }

    let mut map = TestHeaderMap::new();
    // Add sample entries for testing
    let key1 = HeaderName::from_static("Key1");
    map.add_entry(HashValue(1), key1.clone(), TestHeaderValue);
    
    let key2 = HeaderName::from_static("Key2");
    map.add_entry(HashValue(2), key2.clone(), TestHeaderValue);

    assert_eq!(map.entries.len(), 2);
    
    // Test removal of the first entry (found index 0)
    let removed_entry = map.remove_found(0, 0);
    assert_eq!(removed_entry.key, key1);

    assert_eq!(map.entries.len(), 1);
    assert!(map.indices[0].is_none());
}

#[test]
fn test_remove_found_empty_map() {
    struct TestHeaderValue;
    
    let mut map = TestHeaderMap {
        mask: 15,
        indices: Box::new([Pos::none(); 16]),
        entries: Vec::new(),
        extra_values: Vec::new(),
        danger: Danger::Green,
    };

    // Attempt to remove from empty map should panic or return a default value depending on implementation
    let result = std::panic::catch_unwind(|| {
        map.remove_found(0, 0);
    });
    
    assert!(result.is_err());
}

