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
    fn new(indices: Vec<usize>, entries: Vec<Entry>) -> Self {
        TestStruct { indices, entries }
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
                update_index(self.indices, entry.hash, i, i - 1);
            }
        }
    }
}

fn update_index(indices: &mut Vec<usize>, hash: u64, original_index: usize, new_index: usize) {
    // Mock implementation for testing purposes.
    if let Some(pos) = indices.iter().position(|&x| x == original_index) {
        indices[pos] = new_index;
    }
}

#[test]
fn test_decrement_indices_half_capacity() {
    let mut map = TestStruct::new(vec![0, 1, 2, 3, 4], vec![Entry { hash: 1 }, Entry { hash: 2 }, Entry { hash: 3 }, Entry { hash: 4 }]);
    map.decrement_indices(1, 3);
    assert_eq!(map.indices, vec![0, 0, 1, 2, 4]);
}

#[test]
#[should_panic]
fn test_decrement_indices_out_of_bounds() {
    let mut map = TestStruct::new(vec![0, 1, 2, 3, 4], vec![Entry { hash: 1 }, Entry { hash: 2 }]);
    map.decrement_indices(0, 3); // This will panic because shift_entries will try to access out of bounds.
}

#[test]
fn test_decrement_indices_no_effect() {
    let mut map = TestStruct::new(vec![0, 1, 2, 3, 4], vec![Entry { hash: 1 }, Entry { hash: 2 }, Entry { hash: 3 }]);
    map.decrement_indices(0, 0);
    assert_eq!(map.indices, vec![0, 1, 2, 3, 4]); // No effect
}

