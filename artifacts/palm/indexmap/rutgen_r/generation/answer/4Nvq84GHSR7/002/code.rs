// Answer 0

#[derive(Debug)]
struct IndexMap {
    indices: Vec<usize>,
    entries: Vec<usize>,
}

impl IndexMap {
    fn new() -> Self {
        Self {
            indices: Vec::new(),
            entries: Vec::new(),
        }
    }

    fn insert(&mut self, entry: usize) {
        self.entries.push(entry);
        self.indices.push(self.indices.len());
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
            self.indices.extend_from_slice(start_entries);
            self.indices.extend_from_slice(shifted_entries);
        } else if erased + shifted < half_capacity {
            for (i, entry) in (start..).zip(erased_entries) {
                self.indices.retain(|&x| x != entry);
            }
            for ((new, old), entry) in (start..).zip(end..).zip(shifted_entries) {
                self.indices.retain(|&x| x != old);
                self.indices.push(new);
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

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_erase_indices_panic_out_of_bounds_start() {
    let mut map = IndexMap::new();
    map.insert(1);
    map.insert(2);
    map.insert(3);
    
    map.erase_indices(4, 5); // start index out of bounds
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_erase_indices_panic_out_of_bounds_end() {
    let mut map = IndexMap::new();
    map.insert(1);
    map.insert(2);
    
    map.erase_indices(1, 3); // end index out of bounds
}

#[test]
fn test_erase_indices_non_empty_erased() {
    let mut map = IndexMap::new();
    map.insert(1);
    map.insert(2);
    map.insert(3);
    map.insert(4);
    map.insert(5);
    map.insert(6);
    
    map.erase_indices(2, 4); // erase indices 2 to 4, should leave 1, 2 and shift 4, 5, 6 down

    assert_eq!(map.indices, vec![0, 1, 2, 3]); // After erase, remaining indices should be adjusted
}

#[test]
fn test_erase_indices_edge_case_start_equals_end() {
    let mut map = IndexMap::new();
    map.insert(1);
    map.insert(2);
    map.insert(3);
    
    map.erase_indices(2, 2); // No indices erased, should do nothing

    assert_eq!(map.indices, vec![0, 1]); // Indices should remain the same
}

#[test]
fn test_erase_indices_large_erased() {
    let mut map = IndexMap::new();
    for i in 0..10 {
        map.insert(i);
    }
    
    map.erase_indices(0, 5); // Erase indices 0 to 5

    assert_eq!(map.indices.len(), 5); // Should only have the last 5 entries left
}

