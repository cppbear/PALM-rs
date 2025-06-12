// Answer 0

#[test]
fn test_swap_indices_equal_and_in_bounds() {
    struct HashWrapper {
        value: usize,
    }

    struct Entry {
        hash: HashWrapper,
    }

    struct IndexMap {
        entries: Vec<Entry>,
        indices: std::collections::HashMap<usize, usize>,
    }

    impl IndexMap {
        fn new() -> Self {
            IndexMap {
                entries: vec![
                    Entry { hash: HashWrapper { value: 0 } },
                    Entry { hash: HashWrapper { value: 1 } },
                ],
                indices: vec![(0, 0), (1, 1)].into_iter().collect(),
            }
        }

        fn swap_indices(&mut self, a: usize, b: usize) {
            if a == b && a < self.entries.len() {
                return;
            }

            match self.indices.get_many_mut(
                [self.entries[a].hash.value, self.entries[b].hash.value],
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
    map.swap_indices(0, 0); // Test equal indices, should not panic
}

#[test]
#[should_panic]
fn test_swap_indices_equal_and_out_of_bounds() {
    struct HashWrapper {
        value: usize,
    }

    struct Entry {
        hash: HashWrapper,
    }

    struct IndexMap {
        entries: Vec<Entry>,
        indices: std::collections::HashMap<usize, usize>,
    }

    impl IndexMap {
        fn new() -> Self {
            IndexMap {
                entries: vec![
                    Entry { hash: HashWrapper { value: 0 } },
                    Entry { hash: HashWrapper { value: 1 } },
                ],
                indices: vec![(0, 0), (1, 1)].into_iter().collect(),
            }
        }

        fn swap_indices(&mut self, a: usize, b: usize) {
            if a == b && a < self.entries.len() {
                return;
            }

            match self.indices.get_many_mut(
                [self.entries[a].hash.value, self.entries[b].hash.value],
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
    map.swap_indices(2, 2); // Test equal indices with out-of-bounds access
}

#[test]
#[should_panic]
fn test_swap_indices_out_of_bound() {
    struct HashWrapper {
        value: usize,
    }

    struct Entry {
        hash: HashWrapper,
    }

    struct IndexMap {
        entries: Vec<Entry>,
        indices: std::collections::HashMap<usize, usize>,
    }

    impl IndexMap {
        fn new() -> Self {
            IndexMap {
                entries: vec![],
                indices: std::collections::HashMap::new(),
            }
        }

        fn swap_indices(&mut self, a: usize, b: usize) {
            if a == b && a < self.entries.len() {
                return;
            }

            match self.indices.get_many_mut(
                [self.entries[a].hash.value, self.entries[b].hash.value],
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
    map.swap_indices(0, 1); // Test out-of-bounds indices
}

