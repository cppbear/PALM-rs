// Answer 0

#[derive(Debug)]
struct IndexMap {
    indices: Vec<usize>,
    entries: Vec<Entry>,
}

#[derive(Debug)]
struct Entry {
    hash: usize,
}

impl IndexMap {
    fn new() -> Self {
        IndexMap {
            indices: Vec::new(),
            entries: Vec::new(),
        }
    }

    fn push_entry(&mut self, hash: usize) {
        self.entries.push(Entry { hash });
        self.indices.push(self.entries.len() - 1);
    }

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

fn update_index(indices: &mut Vec<usize>, hash: usize, old_index: usize, new_index: usize) {
    if let Some(pos) = indices.iter().position(|&x| x == old_index) {
        indices[pos] = new_index;
    }
}

#[test]
fn test_increment_indices_no_shift() {
    let mut map = IndexMap::new();
    map.push_entry(1);
    map.push_entry(2);
    map.push_entry(3);
    
    map.increment_indices(1, 2);
    
    assert_eq!(map.indices, vec![0, 2, 1]);
}

#[test]
fn test_increment_indices_shift_full() {
    let mut map = IndexMap::new();
    map.push_entry(1);
    map.push_entry(2);
    map.push_entry(3);
    map.push_entry(4);
    
    map.increment_indices(0, 2);
    
    assert_eq!(map.indices, vec![2, 0, 1, 3]);
}

#[test]
fn test_increment_indices_with_empty_range() {
    let mut map = IndexMap::new();
    map.push_entry(1);
    map.push_entry(2);
    
    map.increment_indices(1, 1);
    
    assert_eq!(map.indices, vec![0, 1]);
}

#[test]
fn test_increment_indices_end_beyond_capacity() {
    let mut map = IndexMap::new();
    map.push_entry(1);
    map.push_entry(2);
    map.push_entry(3);
    
    map.increment_indices(0, 4);
    
    assert_eq!(map.indices, vec![1, 2, 3]);
}

