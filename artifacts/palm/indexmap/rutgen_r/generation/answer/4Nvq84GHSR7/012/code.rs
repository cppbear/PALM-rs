// Answer 0

#[test]
fn test_erase_indices_empty_entries() {
    struct TestStruct {
        entries: Vec<i32>,
        indices: Vec<usize>,
    }

    impl TestStruct {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
                indices: Vec::new(),
            }
        }

        fn erase_indices(&mut self, start: usize, end: usize) {
            let (init, shifted_entries) = self.entries.split_at(end);
            let (start_entries, erased_entries) = init.split_at(start);

            let erased = erased_entries.len();
            let shifted = shifted_entries.len();
            let half_capacity = self.indices.capacity() / 2;

            if erased == 0 {
                // Degenerate case, nothing to do
            } else if start + shifted < half_capacity && start < erased {
                self.indices.clear();
            } else if erased + shifted < half_capacity {
                for (i, entry) in (start..).zip(erased_entries) {}
                for ((new, old), entry) in (start..).zip(end..).zip(shifted_entries) {}
            } else {
                let offset = end - start;
                self.indices.retain(move |i| {
                    if *i >= end {
                        *i -= offset;
                        true
                    } else {
                        *i < start
                    }
                });
            }

            debug_assert_eq!(self.indices.len(), start + shifted);
        }
    }

    let mut test_struct = TestStruct::new();

    test_struct.erase_indices(0, 0); // No entries to erase
}

#[test]
fn test_erase_indices_single_entry() {
    struct TestStruct {
        entries: Vec<i32>,
        indices: Vec<usize>,
    }

    impl TestStruct {
        fn new(entries: Vec<i32>) -> Self {
            Self {
                entries,
                indices: vec![0],
            }
        }

        fn erase_indices(&mut self, start: usize, end: usize) {
            let (init, shifted_entries) = self.entries.split_at(end);
            let (start_entries, erased_entries) = init.split_at(start);

            let erased = erased_entries.len();
            let shifted = shifted_entries.len();
            let half_capacity = self.indices.capacity() / 2;

            if erased == 0 {
                // Degenerate case, nothing to do
            } else if start + shifted < half_capacity && start < erased {
                self.indices.clear();
            } else if erased + shifted < half_capacity {
                for (i, entry) in (start..).zip(erased_entries) {}
                for ((new, old), entry) in (start..).zip(end..).zip(shifted_entries) {}
            } else {
                let offset = end - start;
                self.indices.retain(move |i| {
                    if *i >= end {
                        *i -= offset;
                        true
                    } else {
                        *i < start
                    }
                });
            }

            debug_assert_eq!(self.indices.len(), start + shifted);
        }
    }

    let mut test_struct = TestStruct::new(vec![1]);

    test_struct.erase_indices(0, 1); // Complete erase on single entry
}

#[test]
fn test_erase_indices_multiple_entries() {
    struct TestStruct {
        entries: Vec<i32>,
        indices: Vec<usize>,
    }

    impl TestStruct {
        fn new(entries: Vec<i32>) -> Self {
            Self {
                entries,
                indices: (0..entries.len()).collect(),
            }
        }

        fn erase_indices(&mut self, start: usize, end: usize) {
            let (init, shifted_entries) = self.entries.split_at(end);
            let (start_entries, erased_entries) = init.split_at(start);

            let erased = erased_entries.len();
            let shifted = shifted_entries.len();
            let half_capacity = self.indices.capacity() / 2;

            if erased == 0 {
                // Degenerate case, nothing to do
            } else if start + shifted < half_capacity && start < erased {
                self.indices.clear();
            } else if erased + shifted < half_capacity {
                for (i, entry) in (start..).zip(erased_entries) {}
                for ((new, old), entry) in (start..).zip(end..).zip(shifted_entries) {}
            } else {
                let offset = end - start;
                self.indices.retain(move |i| {
                    if *i >= end {
                        *i -= offset;
                        true
                    } else {
                        *i < start
                    }
                });
            }

            debug_assert_eq!(self.indices.len(), start + shifted);
        }
    }

    let mut test_struct = TestStruct::new(vec![1, 2, 3, 4, 5]);

    test_struct.erase_indices(2, 4); // Erase middle elements
}

