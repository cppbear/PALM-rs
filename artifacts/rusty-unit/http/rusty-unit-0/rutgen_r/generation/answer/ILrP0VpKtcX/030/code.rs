// Answer 0

#[test]
fn test_remove_found_valid_entry() {
    struct Pos {
        index: usize,
        hash: usize,
    }
    impl Pos {
        fn none() -> Self {
            Pos { index: usize::MAX, hash: 0 }
        }
        fn new(index: usize, hash: usize) -> Self {
            Pos { index, hash }
        }
        fn resolve(&self) -> Option<(usize, usize)> {
            if self.index == usize::MAX {
                None
            } else {
                Some((self.index, self.hash))
            }
        }
    }

    struct Link {
        next: usize,
        prev: usize,
    }
    
    struct Entry {
        hash: usize,
        links: Option<Link>,
    }
    
    struct Bucket<T> {
        entries: Vec<Entry>,
        indices: Vec<Pos>,
        extra_values: Vec<Entry>,
        mask: usize,
    }

    impl Bucket<Entry> {
        fn remove_found(&mut self, probe: usize, found: usize) -> Entry {
            // Copy of the implementation here
            self.indices[probe] = Pos::none();
            let entry = self.entries.swap_remove(found);

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

                if let Some(links) = entry.links {
                    self.extra_values[links.next].prev = Link { next: found, prev: 0 }; // updating with dummy values
                    self.extra_values[links.prev].next = Link { next: found, prev: 0 }; // updating with dummy values
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

    let mut bucket = Bucket {
        entries: vec![Entry { hash: 1, links: Some(Link { next: 1, prev: 0 }) }],
        indices: vec![Pos::new(0, 1)],
        extra_values: vec![Entry { hash: 0, links: None }],
        mask: 1,
    };

    let entry = bucket.remove_found(0, 0);
    assert_eq!(entry.hash, 1);
}

#[test]
#[should_panic]
fn test_remove_found_panics_on_invalid_found() {
    struct Pos {
        index: usize,
        hash: usize,
    }
    impl Pos {
        fn none() -> Self {
            Pos { index: usize::MAX, hash: 0 }
        }
        fn resolve(&self) -> Option<(usize, usize)> {
            if self.index == usize::MAX {
                None
            } else {
                Some((self.index, self.hash))
            }
        }
    }

    struct Entry {
        hash: usize,
        links: Option<Link>,
    }

    struct Bucket<T> {
        entries: Vec<Entry>,
        indices: Vec<Pos>,
    }

    impl Bucket<Entry> {
        fn remove_found(&mut self, probe: usize, found: usize) -> Entry {
            self.indices[probe] = Pos::none();
            let entry = self.entries.swap_remove(found);
            entry
        }
    }

    // This will panic since `found` is out of bounds
    let mut bucket = Bucket {
        entries: vec![Entry { hash: 1, links: None }],
        indices: vec![Pos::new(0, 1)],
    };

    bucket.remove_found(0, 1);
}

