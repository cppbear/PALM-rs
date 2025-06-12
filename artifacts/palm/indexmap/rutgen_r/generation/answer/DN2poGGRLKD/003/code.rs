// Answer 0

#[derive(Debug)]
struct TestStruct {
    indices: Vec<usize>,
    entries: Vec<Entry>,
}

struct Entry {
    hash: usize,
}

fn update_index(indices: &mut Vec<usize>, hash: usize, old_index: usize, new_index: usize) {
    // Mock implementation for test purposes.
    if let Some(pos) = indices.iter().position(|&x| x == old_index) {
        indices[pos] = new_index;
    }
}

impl TestStruct {
    fn increment_indices(&mut self, start: usize, end: usize) {
        let shifted_entries = &self.entries[start..end];
        if shifted_entries.len() > self.indices.capacity() / 2 {
            for i in &mut *self.indices {
                if start <= *i && *i < end {
                    *i += 1;
                }
            }
        } else {
            for (i, entry) in (start..end).zip(shifted_entries).rev() {
                update_index(self.indices, entry.hash, i, i + 1);
            }
        }
    }
}

#[test]
fn test_increment_indices_large_shift() {
    let mut test_struct = TestStruct {
        indices: vec![3, 4, 5, 6, 7, 8],
        entries: vec![
            Entry { hash: 0 },
            Entry { hash: 1 },
            Entry { hash: 2 },
            Entry { hash: 3 },
            Entry { hash: 4 },
            Entry { hash: 5 },
            Entry { hash: 6 },
            Entry { hash: 7 },
        ],
    };

    test_struct.increment_indices(0, 4);
    assert_eq!(test_struct.indices, vec![3, 4, 5, 7, 8, 9]);
}

#[test]
fn test_increment_indices_no_shift() {
    let mut test_struct = TestStruct {
        indices: vec![0, 1, 2, 3, 4],
        entries: vec![
            Entry { hash: 0 },
            Entry { hash: 1 },
            Entry { hash: 2 },
            Entry { hash: 3 },
            Entry { hash: 4 },
        ],
    };

    test_struct.increment_indices(3, 5);
    assert_eq!(test_struct.indices, vec![0, 1, 2, 3, 5]);
}

#[test]
#[should_panic]
fn test_increment_indices_panic_out_of_bounds() {
    let mut test_struct = TestStruct {
        indices: vec![0, 1, 2],
        entries: vec![
            Entry { hash: 0 },
            Entry { hash: 1 },
            Entry { hash: 2 },
        ],
    };

    test_struct.increment_indices(2, 5);
}

#[test]
fn test_increment_indices_with_unused_indices() {
    let mut test_struct = TestStruct {
        indices: vec![0, 5, 6, 7],
        entries: vec![
            Entry { hash: 0 },
            Entry { hash: 1 },
            Entry { hash: 2 },
            Entry { hash: 3 },
            Entry { hash: 4 },
            Entry { hash: 5 },
            Entry { hash: 6 },
            Entry { hash: 7 },
            Entry { hash: 8 },
            Entry { hash: 9 },
        ],
    };

    test_struct.increment_indices(0, 5);
    assert_eq!(test_struct.indices, vec![1, 6, 7, 8]);
}

