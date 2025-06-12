// Answer 0

fn test_rebuild_with_valid_entries() {
    struct Entry {
        key: usize,
        hash: usize,
    }

    struct Pos {
        index: usize,
        hash: usize,
    }

    struct Map {
        entries: Vec<Entry>,
        indices: Vec<Option<Pos>>,
        mask: usize,
        danger: usize,
    }

    impl Map {
        fn new(entries: Vec<Entry>, mask: usize, danger: usize) -> Self {
            let indices = vec![None; entries.len() * 2]; // Create a larger index size for probing
            Map {
                entries,
                indices,
                mask,
                danger,
            }
        }

        fn rebuild(&mut self) {
            'outer: for (index, entry) in self.entries.iter_mut().enumerate() {
                let hash = entry.key; // Simplified for the test context
                let mut probe = hash & self.mask; // Simplified hashing
                let mut dist = 0;

                entry.hash = hash;

                while probe < self.indices.len() {
                    if let Some((_, entry_hash)) = self.indices[probe] {
                        let their_dist = (entry_hash.hash as isize - probe as isize).abs() as usize;

                        if their_dist < dist {
                            break;
                        }
                    } else {
                        self.indices[probe] = Some(Pos { index, hash });
                        continue 'outer;
                    }

                    dist += 1;
                    probe = (probe + 1) & self.mask; // Probing next position
                }
            }
        }
    }

    let entries = vec![Entry { key: 1, hash: 0 }, Entry { key: 2, hash: 0 }];
    let mask = 1; // Assuming we have a simplistic mask
    let danger = 0; // Arbitrary value for the test

    let mut map = Map::new(entries, mask, danger);
    map.rebuild();

    assert_eq!(map.indices.len(), 4); // Check that indices length is correct
    assert!(map.indices[0].is_some()); // Ensure indices have entries after rebuild
}

fn test_rebuild_with_empty_map() {
    struct Entry {
        key: usize,
        hash: usize,
    }

    struct Pos {
        index: usize,
        hash: usize,
    }

    struct Map {
        entries: Vec<Entry>,
        indices: Vec<Option<Pos>>,
        mask: usize,
        danger: usize,
    }

    impl Map {
        fn new(entries: Vec<Entry>, mask: usize, danger: usize) -> Self {
            let indices = vec![None; entries.len() * 2];
            Map {
                entries,
                indices,
                mask,
                danger,
            }
        }

        fn rebuild(&mut self) {
            'outer: for (index, entry) in self.entries.iter_mut().enumerate() {
                let hash = entry.key;
                let mut probe = hash & self.mask;
                let mut dist = 0;

                entry.hash = hash;

                while probe < self.indices.len() {
                    if let Some((_, entry_hash)) = self.indices[probe] {
                        let their_dist = (entry_hash.hash as isize - probe as isize).abs() as usize;

                        if their_dist < dist {
                            break;
                        }
                    } else {
                        self.indices[probe] = Some(Pos { index, hash });
                        continue 'outer;
                    }

                    dist += 1;
                    probe = (probe + 1) & self.mask;
                }
            }
        }
    }

    let entries: Vec<Entry> = Vec::new();
    let mask = 1;
    let danger = 0;

    let mut map = Map::new(entries, mask, danger);
    map.rebuild();

    assert!(map.indices.iter().all(|x| x.is_none())); // Ensure indices remain empty
}

