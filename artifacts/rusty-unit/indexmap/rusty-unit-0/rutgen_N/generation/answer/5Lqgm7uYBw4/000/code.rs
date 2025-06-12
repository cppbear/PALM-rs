// Answer 0

#[test]
fn test_swap_indices_equal_indices() {
    struct Entry {
        hash: std::cell::Cell<usize>,
    }

    struct IndexMap {
        entries: Vec<Entry>,
        indices: std::collections::HashMap<usize, usize>,
    }

    impl IndexMap {
        fn new() -> Self {
            IndexMap {
                entries: Vec::new(),
                indices: std::collections::HashMap::new(),
            }
        }
        
        fn insert(&mut self, entry: Entry) {
            let index = self.entries.len();
            self.indices.insert(entry.hash.get(), index);
            self.entries.push(entry);
        }
        
        fn swap_indices(&mut self, a: usize, b: usize) {
            if a == b && a < self.entries.len() {
                return;
            }

            match self.indices.get_many_mut(
                [self.entries[a].hash.get(), self.entries[b].hash.get()],
                move |i, &x| if i == 0 { x == a } else { x == b },
            ) {
                [Some(ref_a), Some(ref_b)] => {
                    std::mem::swap(ref_a, ref_b);
                    self.entries.swap(a, b);
                }
                _ => panic!("indices not found"),
            }
        }
    }

    let mut map = IndexMap::new();
    map.insert(Entry { hash: std::cell::Cell::new(1) });
    map.insert(Entry { hash: std::cell::Cell::new(2) });
    map.swap_indices(0, 0);
    assert_eq!(map.entries[0].hash.get(), 1);
    assert_eq!(map.entries[1].hash.get(), 2);
}

#[test]
fn test_swap_indices_different_indices() {
    struct Entry {
        hash: std::cell::Cell<usize>,
    }

    struct IndexMap {
        entries: Vec<Entry>,
        indices: std::collections::HashMap<usize, usize>,
    }

    impl IndexMap {
        fn new() -> Self {
            IndexMap {
                entries: Vec::new(),
                indices: std::collections::HashMap::new(),
            }
        }

        fn insert(&mut self, entry: Entry) {
            let index = self.entries.len();
            self.indices.insert(entry.hash.get(), index);
            self.entries.push(entry);
        }

        fn swap_indices(&mut self, a: usize, b: usize) {
            if a == b && a < self.entries.len() {
                return;
            }

            match self.indices.get_many_mut(
                [self.entries[a].hash.get(), self.entries[b].hash.get()],
                move |i, &x| if i == 0 { x == a } else { x == b },
            ) {
                [Some(ref_a), Some(ref_b)] => {
                    std::mem::swap(ref_a, ref_b);
                    self.entries.swap(a, b);
                }
                _ => panic!("indices not found"),
            }
        }
    }

    let mut map = IndexMap::new();
    map.insert(Entry { hash: std::cell::Cell::new(1) });
    map.insert(Entry { hash: std::cell::Cell::new(2) });
    map.swap_indices(0, 1);
    assert_eq!(map.entries[0].hash.get(), 2);
    assert_eq!(map.entries[1].hash.get(), 1);
}

#[test]
#[should_panic(expected = "indices not found")]
fn test_swap_indices_out_of_bounds() {
    struct Entry {
        hash: std::cell::Cell<usize>,
    }

    struct IndexMap {
        entries: Vec<Entry>,
        indices: std::collections::HashMap<usize, usize>,
    }

    impl IndexMap {
        fn new() -> Self {
            IndexMap {
                entries: Vec::new(),
                indices: std::collections::HashMap::new(),
            }
        }

        fn insert(&mut self, entry: Entry) {
            let index = self.entries.len();
            self.indices.insert(entry.hash.get(), index);
            self.entries.push(entry);
        }

        fn swap_indices(&mut self, a: usize, b: usize) {
            if a == b && a < self.entries.len() {
                return;
            }

            match self.indices.get_many_mut(
                [self.entries[a].hash.get(), self.entries[b].hash.get()],
                move |i, &x| if i == 0 { x == a } else { x == b },
            ) {
                [Some(ref_a), Some(ref_b)] => {
                    std::mem::swap(ref_a, ref_b);
                    self.entries.swap(a, b);
                }
                _ => panic!("indices not found"),
            }
        }
    }

    let mut map = IndexMap::new();
    map.insert(Entry { hash: std::cell::Cell::new(1) });
    map.insert(Entry { hash: std::cell::Cell::new(2) });
    map.swap_indices(0, 2);
}

