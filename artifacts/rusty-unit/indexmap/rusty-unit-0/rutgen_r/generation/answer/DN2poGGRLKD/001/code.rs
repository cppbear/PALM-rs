// Answer 0

#[derive(Debug)]
struct IndexMap {
    entries: Vec<Entry>,
    indices: Vec<usize>,
}

#[derive(Debug)]
struct Entry {
    hash: usize,
}

impl IndexMap {
    fn new(entries: Vec<Entry>, indices: Vec<usize>) -> Self {
        IndexMap { entries, indices }
    }

    fn increment_indices(&mut self, start: usize, end: usize) {
        // Use a heuristic between a full sweep vs. a `find()` for every shifted item.
        let shifted_entries = &self.entries[start..end];
        if shifted_entries.len() > self.indices.capacity() / 2 {
            // Shift all indices in range.
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

fn update_index(indices: &mut Vec<usize>, hash: usize, old_index: usize, new_index: usize) {
    if let Some(pos) = indices.iter().position(|x| *x == old_index) {
        indices[pos] = new_index;
    }
}

#[test]
fn test_increment_indices_full_sweep() {
    let entries = vec![Entry { hash: 0 }, Entry { hash: 1 }, Entry { hash: 2 }];
    let indices = vec![0, 1, 2, 3];
    let mut index_map = IndexMap::new(entries, indices);

    index_map.increment_indices(0, 3);

    assert_eq!(index_map.indices, vec![1, 2, 3, 4]);
}

#[test]
fn test_increment_indices_partial_sweep() {
    let entries = vec![Entry { hash: 0 }, Entry { hash: 1 }, Entry { hash: 2 }, Entry { hash: 3 }];
    let indices = vec![0, 1, 2, 4];
    let mut index_map = IndexMap::new(entries, indices);

    index_map.increment_indices(1, 3);

    assert_eq!(index_map.indices, vec![0, 2, 3, 4]);
}

#[test]
#[should_panic]
fn test_increment_indices_out_of_bounds() {
    let entries = vec![Entry { hash: 0 }, Entry { hash: 1 }, Entry { hash: 2 }];
    let indices = vec![0, 1, 2];
    let mut index_map = IndexMap::new(entries, indices);

    index_map.increment_indices(2, 5); // This should panic due to out of bounds access
}

