// Answer 0

#[test]
fn test_rebuild_empty_indices() {
    struct Pos {
        index: usize,
        hash: u64,
    }
    
    struct Entry {
        key: String,
        hash: u64,
    }
    
    struct Map {
        entries: Vec<Entry>,
        indices: Vec<Option<Pos>>,
        mask: usize,
        danger: bool,
    }

    impl Map {
        fn new() -> Self {
            Map {
                entries: Vec::new(),
                indices: Vec::new(),
                mask: 0,
                danger: false,
            }
        }

        fn rebuild(&mut self) {
            'outer: for (index, entry) in self.entries.iter_mut().enumerate() {
                let hash = hash_elem_using(&self.danger, &entry.key);
                let mut probe = desired_pos(self.mask, hash);
                let mut dist = 0;

                entry.hash = hash;

                probe_loop!(probe < self.indices.len(), {
                    if let Some((_, entry_hash)) = self.indices[probe].resolve() {
                        let their_dist = probe_distance(self.mask, entry_hash, probe);

                        if their_dist < dist {
                            break;
                        }
                    } else {
                        self.indices[probe] = Some(Pos { index, hash });
                        continue 'outer;
                    }

                    dist += 1;
                });

                do_insert_phase_two(&mut self.indices, probe, Pos { index, hash });
            }
        }
    }

    fn hash_elem_using(danger: &bool, key: &str) -> u64 {
        // This function would normally return a hash - simplified here.
        key.len() as u64
    }

    fn desired_pos(mask: usize, hash: u64) -> usize {
        // This function simulates desired position calculation.
        (hash as usize) & mask
    }

    fn probe_distance(mask: usize, entry_hash: u64, probe: usize) -> usize {
        // A dummy function that calculates probing distance.
        (entry_hash as usize).wrapping_sub(probe) & mask
    }
    
    fn do_insert_phase_two(indices: &mut Vec<Option<Pos>>, probe: usize, pos: Pos) {
        // Simulate a phase two insert operation.
        indices[probe] = Some(pos);
    }
    
    // Create a Map with empty entries and indices
    let mut map = Map::new();
    map.indices = Vec::new(); // Indices length is zero

    // Call rebuild, expecting no panics
    map.rebuild();
}

#[test]
fn test_rebuild_non_empty_entries() {
    struct Pos {
        index: usize,
        hash: u64,
    }
    
    struct Entry {
        key: String,
        hash: u64,
    }
    
    struct Map {
        entries: Vec<Entry>,
        indices: Vec<Option<Pos>>,
        mask: usize,
        danger: bool,
    }

    impl Map {
        fn new(entries: Vec<Entry>, mask: usize) -> Self {
            Map {
                entries,
                indices: Vec::new(),
                mask,
                danger: false,
            }
        }

        fn rebuild(&mut self) {
            'outer: for (index, entry) in self.entries.iter_mut().enumerate() {
                let hash = hash_elem_using(&self.danger, &entry.key);
                let mut probe = desired_pos(self.mask, hash);
                let mut dist = 0;
                
                entry.hash = hash;

                probe_loop!(probe < self.indices.len(), {
                    if let Some((_, entry_hash)) = self.indices[probe].resolve() {
                        let their_dist = probe_distance(self.mask, entry_hash, probe);

                        if their_dist < dist {
                            break;
                        }
                    } else {
                        self.indices[probe] = Some(Pos { index, hash });
                        continue 'outer;
                    }

                    dist += 1;
                });

                do_insert_phase_two(&mut self.indices, probe, Pos { index, hash });
            }
        }
    }

    fn hash_elem_using(danger: &bool, key: &str) -> u64 {
        // Dummy hash function
        key.len() as u64
    }

    fn desired_pos(mask: usize, hash: u64) -> usize {
        (hash as usize) & mask
    }

    fn probe_distance(mask: usize, entry_hash: u64, probe: usize) -> usize {
        (entry_hash as usize).wrapping_sub(probe) & mask
    }

    fn do_insert_phase_two(indices: &mut Vec<Option<Pos>>, probe: usize, pos: Pos) {
        if probe >= indices.len() {
            indices.resize_with(probe + 1, || None);
        }
        indices[probe] = Some(pos);
    }

    let entry1 = Entry { key: String::from("key1"), hash: 0 };
    let entry2 = Entry { key: String::from("key2"), hash: 0 };

    // Initialize Map with two entries but empty indices
    let mut map = Map::new(vec![entry1, entry2], 1);
    map.indices = Vec::new(); // Indices length is zero

    // Check that calling rebuild does not panic
    map.rebuild();
}

