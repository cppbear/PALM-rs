// Answer 0

#[test]
fn test_rebuild_empty_map() {
    struct Pos {
        index: usize,
        hash: u64,
    }

    struct Entry {
        key: i32,
        hash: u64,
    }

    struct Map {
        entries: Vec<Entry>,
        indices: Vec<Option<Pos>>,
        mask: usize,
        danger: usize,
    }

    impl Map {
        fn new() -> Self {
            Map {
                entries: Vec::new(),
                indices: vec![None; 8],
                mask: 7,
                danger: 0,
            }
        }

        fn rebuild(&mut self) {
            // Loop over all entries and re-insert them into the map
            'outer: for (index, entry) in self.entries.iter_mut().enumerate() {
                let hash = entry.hash;  // Simplified for testing
                let mut probe = self.mask & hash as usize; // Simplified hash calculation
                let mut dist = 0;

                entry.hash = hash;

                while probe < self.indices.len() {
                    if let Some(existing) = &self.indices[probe] {
                        // if existing element probed less than us, swap
                        let their_dist = (existing.index as isize - probe as isize).abs() as usize;
                        if their_dist < dist {
                            break;
                        }
                    } else {
                        // Vacant slot
                        self.indices[probe] = Some(Pos { index, hash });
                        continue 'outer;
                    }

                    dist += 1;
                }
            }
        }
    }

    let mut map = Map::new();
    map.rebuild();
    assert!(map.indices.iter().all(|i| i.is_none()));
}

#[test]
fn test_rebuild_single_entry() {
    struct Pos {
        index: usize,
        hash: u64,
    }

    struct Entry {
        key: i32,
        hash: u64,
    }

    struct Map {
        entries: Vec<Entry>,
        indices: Vec<Option<Pos>>,
        mask: usize,
        danger: usize,
    }

    impl Map {
        fn new() -> Self {
            Map {
                entries: Vec::new(),
                indices: vec![None; 8],
                mask: 7,
                danger: 0,
            }
        }

        fn rebuild(&mut self) {
            'outer: for (index, entry) in self.entries.iter_mut().enumerate() {
                let hash = entry.hash; // Simplified for testing
                let mut probe = self.mask & hash as usize; // Simplified hash calculation
                let mut dist = 0;

                entry.hash = hash;

                while probe < self.indices.len() {
                    if let Some(existing) = &self.indices[probe] {
                        let their_dist = (existing.index as isize - probe as isize).abs() as usize;
                        if their_dist < dist {
                            break;
                        }
                    } else {
                        self.indices[probe] = Some(Pos { index, hash });
                        continue 'outer;
                    }

                    dist += 1;
                }
            }
        }
    }

    let mut map = Map::new();
    map.entries.push(Entry { key: 1, hash: 1 });
    map.rebuild();
    assert!(map.indices.iter().any(|i| i.is_some()));
}

#[test]
fn test_rebuild_multiple_entries() {
    struct Pos {
        index: usize,
        hash: u64,
    }

    struct Entry {
        key: i32,
        hash: u64,
    }

    struct Map {
        entries: Vec<Entry>,
        indices: Vec<Option<Pos>>,
        mask: usize,
        danger: usize,
    }

    impl Map {
        fn new() -> Self {
            Map {
                entries: Vec::new(),
                indices: vec![None; 8],
                mask: 7,
                danger: 0,
            }
        }

        fn rebuild(&mut self) {
            'outer: for (index, entry) in self.entries.iter_mut().enumerate() {
                let hash = entry.hash; // Simplified for testing
                let mut probe = self.mask & hash as usize; // Simplified hash calculation
                let mut dist = 0;

                entry.hash = hash;

                while probe < self.indices.len() {
                    if let Some(existing) = &self.indices[probe] {
                        let their_dist = (existing.index as isize - probe as isize).abs() as usize;
                        if their_dist < dist {
                            break;
                        }
                    } else {
                        self.indices[probe] = Some(Pos { index, hash });
                        continue 'outer;
                    }

                    dist += 1;
                }
            }
        }
    }

    let mut map = Map::new();
    map.entries.push(Entry { key: 1, hash: 1 });
    map.entries.push(Entry { key: 2, hash: 2 });
    map.rebuild();
    assert!(map.indices.iter().filter(|i| i.is_some()).count() == 2);
}

