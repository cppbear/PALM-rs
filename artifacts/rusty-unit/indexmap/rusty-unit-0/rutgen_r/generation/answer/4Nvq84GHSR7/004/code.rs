// Answer 0

#[test]
fn test_erase_indices_no_erasure() {
    #[derive(Default)]
    struct TestStruct {
        entries: Vec<u32>,
        indices: Vec<usize>,
    }

    impl TestStruct {
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

                insert_bulk_no_grow(&mut self.indices, start_entries);
                insert_bulk_no_grow(&mut self.indices, shifted_entries);
            } else if erased + shifted < half_capacity {
                for (i, entry) in (start..).zip(erased_entries) {
                    erase_index(&mut self.indices, entry, i);
                }

                for ((new, old), entry) in (start..).zip(end..).zip(shifted_entries) {
                    update_index(&mut self.indices, entry, old, new);
                }
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

    fn insert_bulk_no_grow(indices: &mut Vec<usize>, entries: &[u32]) {
        for entry in entries {
            indices.push(*entry as usize);
        }
    }

    fn erase_index(indices: &mut Vec<usize>, hash: u32, index: usize) {
        // Simulate erasing an index
        if let Some(pos) = indices.iter().position(|&i| i == hash as usize) {
            indices.remove(pos);
        }
    }

    fn update_index(indices: &mut Vec<usize>, hash: u32, old: usize, new: usize) {
        // Simulate updating an index
        if let Some(pos) = indices.iter().position(|&i| i == old) {
            indices[pos] = new;
        }
    }

    let mut test_struct = TestStruct::default();
    test_struct.entries = vec![1, 2, 3, 4, 5];
    test_struct.indices = vec![0, 1, 2, 3, 4];

    // Set for conditions:
    // erased == 0 is true means no indices are between start and end
    // start + shifted < half_capacity is true (start=0, shifted remains below half capacity)
    // start < erased is not applicable since erased is 0

    test_struct.erase_indices(0, 0);
    
    assert_eq!(test_struct.indices.len(), 5);
}

#[test]
#[should_panic]
fn test_erase_indices_panic_cases() {
    #[derive(Default)]
    struct TestStruct {
        entries: Vec<u32>,
        indices: Vec<usize>,
    }

    impl TestStruct {
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

                insert_bulk_no_grow(&mut self.indices, start_entries);
                insert_bulk_no_grow(&mut self.indices, shifted_entries);
            } else if erased + shifted < half_capacity {
                for (i, entry) in (start..).zip(erased_entries) {
                    erase_index(&mut self.indices, entry, i);
                }

                for ((new, old), entry) in (start..).zip(end..).zip(shifted_entries) {
                    update_index(&mut self.indices, entry, old, new);
                }
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

    fn insert_bulk_no_grow(indices: &mut Vec<usize>, entries: &[u32]) {
        for entry in entries {
            indices.push(*entry as usize);
        }
    }

    fn erase_index(indices: &mut Vec<usize>, hash: u32, index: usize) {
        // Simulate erasing an index
        if let Some(pos) = indices.iter().position(|&i| i == hash as usize) {
            indices.remove(pos);
        }
    }

    fn update_index(indices: &mut Vec<usize>, hash: u32, old: usize, new: usize) {
        // Simulate updating an index
        if let Some(pos) = indices.iter().position(|&i| i == old) {
            indices[pos] = new;
        }
    }

    let mut test_struct = TestStruct::default();
    test_struct.entries = vec![1, 2, 3, 4, 5];

    // Trigger panic due to end being out of bounds of entries
    test_struct.erase_indices(0, 10);
}

