// Answer 0

#[derive(Debug)]
struct TestStruct {
    indices: Vec<usize>,
    entries: Vec<Entry>,
}

#[derive(Debug)]
struct Entry {
    hash: u64,
}

impl TestStruct {
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
                update_index(self.indices, entry.hash, i, i - 1);
            }
        }
    }
}

fn update_index(indices: &mut Vec<usize>, hash: u64, old_index: usize, new_index: usize) {
    // Dummy implementation for test purposes
    if old_index < indices.len() {
        indices[old_index] = new_index;
    }
}

#[test]
#[should_panic]
fn test_decrement_indices_panic_out_of_bounds() {
    let mut test_struct = TestStruct {
        indices: vec![0, 1, 2, 3],
        entries: vec![Entry { hash: 0 }, Entry { hash: 1 }],
    };
    
    // This will trigger a panic due to out of bounds access
    test_struct.decrement_indices(3, 5);
}

#[test]
fn test_decrement_indices_large_shift() {
    let mut test_struct = TestStruct {
        indices: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        entries: vec![Entry { hash: 0 }, Entry { hash: 1 }, Entry { hash: 2 }, Entry { hash: 3 }, Entry { hash: 4 }, Entry { hash: 5 }, Entry { hash: 6 }, Entry { hash: 7 }, Entry { hash: 8 }],
    };

    // capacity is 10, shifted_entries will be 5, which is > 10 / 2
    test_struct.decrement_indices(0, 5);
    
    assert_eq!(test_struct.indices, vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_decrement_indices_small_shift() {
    let mut test_struct = TestStruct {
        indices: vec![0, 1, 2, 3],
        entries: vec![Entry { hash: 0 }, Entry { hash: 1 }, Entry { hash: 2 }, Entry { hash: 3 }],
    };

    // capacity is 4, shifted_entries will be 3, which is not > 4 / 2
    test_struct.decrement_indices(1, 4);
    
    assert_eq!(test_struct.indices, vec![0, 0, 2, 3]);
}

