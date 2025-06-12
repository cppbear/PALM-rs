// Answer 0

#[derive(Debug)]
struct IndexMap {
    entries: Vec<i32>,
    indices: Vec<usize>,
}

impl IndexMap {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
            indices: Vec::new(),
        }
    }

    fn insert(&mut self, entry: i32) {
        self.entries.push(entry);
        self.indices.push(self.entries.len() - 1);
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
                self.update_index(entry.hash(), i, i - 1);
            }
        }
    }

    fn update_index(&mut self, hash: usize, old_index: usize, new_index: usize) {
        // Dummy implementation for update_index
        if let Some(pos) = self.indices.iter().position(|&x| x == old_index) {
            self.indices[pos] = new_index;
        }
    }
}

#[test]
fn test_decrement_indices_full_shift() {
    let mut map = IndexMap::new();
    map.insert(1);
    map.insert(2);
    map.insert(3);
    map.insert(4);

    map.decrement_indices(1, 3);
    assert_eq!(map.indices, vec![0, 1, 2, 3]);
}

#[test]
fn test_decrement_indices_partial_shift() {
    let mut map = IndexMap::new();
    map.insert(1);
    map.insert(2);
    map.insert(3);
    map.insert(4);
    map.indices = vec![0, 1, 2, 3];

    map.decrement_indices(1, 2);
    assert_eq!(map.indices, vec![0, 0, 2, 3]);
}

#[test]
fn test_decrement_indices_boundary() {
    let mut map = IndexMap::new();
    map.insert(1);
    map.insert(2);
    map.insert(3);

    map.decrement_indices(0, 1);
    assert_eq!(map.indices, vec![0, 1, 2]);
}

#[test]
#[should_panic]
fn test_decrement_indices_invalid_range() {
    let mut map = IndexMap::new();
    map.insert(1);
    map.insert(2);
    
    // This should panic due to an invalid range
    map.decrement_indices(2, 1);
}

