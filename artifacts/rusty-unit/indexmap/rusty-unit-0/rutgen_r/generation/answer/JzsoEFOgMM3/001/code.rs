// Answer 0

fn test_decrement_indices_large_shift() {
    struct TestStruct {
        entries: Vec<u32>,
        indices: Vec<usize>,
    }

    impl TestStruct {
        fn new(entries: Vec<u32>, indices: Vec<usize>) -> Self {
            TestStruct { entries, indices }
        }

        fn decrement_indices(&mut self, start: usize, end: usize) {
            let shifted_entries = &self.entries[start..end];
            if shifted_entries.len() > self.indices.capacity() / 2 {
                for i in &mut *self.indices {
                    if start <= *i && *i < end {
                        *i -= 1;
                    }
                }
            } else {
                for (i, entry) in (start..end).zip(shifted_entries) {
                    // Assuming update_index is defined elsewhere in the actual code.
                    // Here we mock its logic for demonstration purposes.
                    // update_index(self.indices, entry.hash, i, i - 1);
                    self.indices[i] -= 1; // Mocking index update
                }
            }
        }
    }

    let mut test_struct = TestStruct::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![0, 1, 2, 3, 4, 5, 6, 7]);
    let original_indices = test_struct.indices.clone();
    
    test_struct.decrement_indices(2, 8);

    assert_eq!(test_struct.indices, vec![0, 1, 1, 2, 3, 4, 5, 6]);
    assert!(test_struct.indices != original_indices); // Ensure indices changed.
}

fn test_decrement_indices_with_no_shift() {
    struct TestStruct {
        entries: Vec<u32>,
        indices: Vec<usize>,
    }

    impl TestStruct {
        fn new(entries: Vec<u32>, indices: Vec<usize>) -> Self {
            TestStruct { entries, indices }
        }

        fn decrement_indices(&mut self, start: usize, end: usize) {
            let shifted_entries = &self.entries[start..end];
            if shifted_entries.len() > self.indices.capacity() / 2 {
                for i in &mut *self.indices {
                    if start <= *i && *i < end {
                        *i -= 1;
                    }
                }
            } else {
                for (i, entry) in (start..end).zip(shifted_entries) {
                    self.indices[i] -= 1; // Mocking index update
                }
            }
        }
    }

    let mut test_struct = TestStruct::new(vec![1, 2, 3, 4, 5], vec![0, 1, 2, 3]);
    
    test_struct.decrement_indices(0, 2);

    assert_eq!(test_struct.indices, vec![0, 0, 2, 3]); // Ensure only first two decremented
}

#[test]
fn run_tests() {
    test_decrement_indices_large_shift();
    test_decrement_indices_with_no_shift();
}

