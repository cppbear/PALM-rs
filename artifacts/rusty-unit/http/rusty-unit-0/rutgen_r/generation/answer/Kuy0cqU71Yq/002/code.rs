// Answer 0

#[test]
fn test_reinsert_entry_in_order_with_non_empty_bucket() {
    struct Pos {
        resolved: Option<(i32, i32)>,
    }

    impl Pos {
        fn resolve(&self) -> Option<(i32, i32)> {
            self.resolved
        }
    }

    struct TestMap {
        mask: usize,
        indices: Vec<Pos>,
    }

    impl TestMap {
        fn new(mask: usize, size: usize) -> Self {
            Self {
                mask,
                indices: vec![Pos { resolved: None }; size],
            }
        }

        fn reinsert_entry_in_order(&mut self, pos: Pos) {
            if let Some((_, entry_hash)) = pos.resolve() {
                let mut probe = self.mask & entry_hash as usize;

                while probe < self.indices.len() {
                    if self.indices[probe].resolve().is_none() {
                        self.indices[probe] = pos;
                        return;
                    }
                    probe += 1; // Move to next probe
                }
            }
        }
    }

    // Test Case Setup
    let mut test_map = TestMap::new(1, 5);
    let pos = Pos { resolved: Some((0, 1)) };
    test_map.indices[1] = Pos { resolved: Some((2, 1)) }; // Pre-filling an occupied bucket

    // Test the method
    test_map.reinsert_entry_in_order(pos);

    // Ensure the entry is added in the next empty bucket
    assert!(test_map.indices[0].resolve().is_none());
    assert!(test_map.indices[1].resolve().is_some());
    assert!(test_map.indices[2].resolve().is_some());
}

#[test]
fn test_reinsert_entry_in_order_with_empty_indices() {
    struct Pos {
        resolved: Option<(i32, i32)>,
    }

    impl Pos {
        fn resolve(&self) -> Option<(i32, i32)> {
            self.resolved
        }
    }

    struct TestMap {
        mask: usize,
        indices: Vec<Pos>,
    }

    impl TestMap {
        fn new(mask: usize, size: usize) -> Self {
            Self {
                mask,
                indices: vec![Pos { resolved: None }; size],
            }
        }

        fn reinsert_entry_in_order(&mut self, pos: Pos) {
            if let Some((_, entry_hash)) = pos.resolve() {
                let mut probe = self.mask & entry_hash as usize;

                while probe < self.indices.len() {
                    if self.indices[probe].resolve().is_none() {
                        self.indices[probe] = pos;
                        return;
                    }
                    probe += 1; // Move to next probe
                }
            }
        }
    }

    // Test Case Setup
    let mut test_map = TestMap::new(1, 5);
    let pos = Pos { resolved: Some((0, 2)) }; // Entry hash of 2 should fit probe

    // Test the method
    test_map.reinsert_entry_in_order(pos);

    // Ensure the entry is added in the first bucket
    assert!(test_map.indices[0].resolve().is_some());
    assert!(test_map.indices[1].resolve().is_none());
}

#[test]
#[should_panic]
fn test_reinsert_entry_in_order_with_invalid_pos() {
    struct Pos {
        resolved: Option<(i32, i32)>,
    }

    impl Pos {
        fn resolve(&self) -> Option<(i32, i32)> {
            self.resolved
        }
    }

    struct TestMap {
        mask: usize,
        indices: Vec<Pos>,
    }

    impl TestMap {
        fn new(mask: usize, size: usize) -> Self {
            Self {
                mask,
                indices: vec![Pos { resolved: None }; size],
            }
        }

        fn reinsert_entry_in_order(&mut self, pos: Pos) {
            if let Some((_, entry_hash)) = pos.resolve() {
                let mut probe = self.mask & entry_hash as usize;

                while probe < self.indices.len() {
                    if self.indices[probe].resolve().is_none() {
                        self.indices[probe] = pos;
                        return;
                    }
                    probe += 1; // Move to next probe
                }
            }
        }
    }

    // Test Case Setup
    let mut test_map = TestMap::new(1, 5);
    let pos = Pos { resolved: None }; // This should cause panic due to failed resolve condition

    // Attempt to call method with invalid pos
    test_map.reinsert_entry_in_order(pos);
}

