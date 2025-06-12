// Answer 0

#[test]
fn test_erase_indices_no_erased_elements() {
    let mut indices = vec![0, 1, 2, 3, 4, 5]; // Initial indices
    let entries = vec![10, 20, 30, 40, 50, 60]; // Corresponding entries

    let mut map = TestMap {
        indices,
        entries,
    };

    // Start and end such that erased == 0
    let start = 3;
    let end = 3;

    // Before erase_indices
    assert_eq!(map.indices.len(), 6);

    // Call the function
    map.erase_indices(start, end);

    // After erase_indices
    assert_eq!(map.indices.len(), 6);
    assert_eq!(map.indices, vec![0, 1, 2, 3, 4, 5]); // Indices should remain unchanged
}

#[test]
fn test_erase_indices_half_capacity_reached() {
    let mut indices = vec![0, 1, 2, 3, 4, 5]; // Initial indices
    let entries = vec![10, 20, 30, 40, 50, 60]; // Corresponding entries

    let mut map = TestMap {
        indices,
        entries,
    };

    // Start and end such that start + shifted == half_capacity
    let start = 2;
    let end = 4;

    // Before erase_indices
    assert_eq!(map.indices.len(), 6);

    // Call the function
    map.erase_indices(start, end);

    // After erase_indices
    assert_eq!(map.indices.len(), 5);
    assert_eq!(map.indices, vec![0, 1, 4, 5]); // Adjusted indices
}

#[test]
fn test_erase_indices_full_sweep_required() {
    let mut indices = vec![0, 1, 2, 3, 4, 5, 6, 7]; // Initial indices
    let entries = vec![10, 20, 30, 40, 50, 60, 70, 80]; // Corresponding entries

    let mut map = TestMap {
        indices,
        entries,
    };

    // Start and end such that erased + shifted == half_capacity
    let start = 0;
    let end = 4;

    // Before erase_indices
    assert_eq!(map.indices.len(), 8);

    // Call the function
    map.erase_indices(start, end);

    // After erase_indices
    assert_eq!(map.indices.len(), 4);
    assert_eq!(map.indices, vec![0, 4, 5, 6, 7]); // Indices should be adjusted
}

// Helper struct to encapsulate the state for tests
struct TestMap {
    indices: Vec<usize>,
    entries: Vec<usize>,
}

impl TestMap {
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
            for (i, _entry) in (start..).zip(erased_entries.iter()) {
                self.indices.remove(i);
            }
            for ((new, old), _entry) in (start..).zip(end..).zip(shifted_entries.iter()) {
                self.indices[new] = old;
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

