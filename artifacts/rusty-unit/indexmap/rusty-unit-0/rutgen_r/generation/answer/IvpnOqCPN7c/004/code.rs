// Answer 0

#[derive(Debug, Clone)]
struct HashEntry {
    hash: u64,
    value: String,
}

struct IndexMap {
    entries: Vec<HashEntry>,
    indices: Vec<Option<usize>>,
}

impl IndexMap {
    fn new(entries: Vec<HashEntry>, indices: Vec<Option<usize>>) -> Self {
        IndexMap { entries, indices }
    }

    fn move_index(&mut self, from: usize, to: usize) {
        let from_hash = self.entries[from].hash;
        let _ = self.entries[to]; // explicit bounds check
        if from != to {
            update_index(self.indices, from_hash, from, usize::MAX);
            if from < to {
                self.decrement_indices(from + 1, to + 1);
                self.entries[from..=to].rotate_left(1);
            } else if to < from {
                self.increment_indices(to, from);
                self.entries[to..=from].rotate_right(1);
            }
            update_index(self.indices, from_hash, usize::MAX, to);
        }
    }
    
    fn decrement_indices(&mut self, start: usize, end: usize) {
        for i in start..end {
            if let Some(index) = &mut self.indices[i] {
                *index -= 1;
            }
        }
    }
    
    fn increment_indices(&mut self, start: usize, end: usize) {
        for i in start..end {
            if let Some(index) = &mut self.indices[i] {
                *index += 1;
            }
        }
    }
}

fn update_index(indices: &mut Vec<Option<usize>>, from_hash: u64, from: usize, to: usize) {
    // simulate updating indices, no actual implementation needed for tests
}

#[test]
fn test_move_index_same_position() {
    let mut map = IndexMap::new(
        vec![HashEntry { hash: 1, value: "a".into() }],
        vec![Some(0)],
    );

    // Moving from and to the same index should not panic
    map.move_index(0, 0);
}

#[test]
#[should_panic]
fn test_move_index_from_out_of_bounds() {
    let mut map = IndexMap::new(
        vec![HashEntry { hash: 1, value: "a".into() }],
        vec![Some(0)],
    );

    // This should panic as 'from' index is out of bounds
    map.move_index(1, 0);
}

#[test]
#[should_panic]
fn test_move_index_to_out_of_bounds() {
    let mut map = IndexMap::new(
        vec![HashEntry { hash: 1, value: "a".into() }],
        vec![Some(0)],
    );

    // This should panic as 'to' index is out of bounds
    map.move_index(0, 1);
}

#[test]
fn test_move_index_valid_case() {
    let mut map = IndexMap::new(
        vec![
            HashEntry { hash: 1, value: "a".into() },
            HashEntry { hash: 2, value: "b".into() },
            HashEntry { hash: 3, value: "c".into() },
        ],
        vec![Some(0), Some(1), Some(2)],
    );

    // Move the first entry to the second position
    map.move_index(0, 1);
    assert_eq!(map.entries[0].value, "b");
    assert_eq!(map.entries[1].value, "a");
    assert_eq!(map.entries[2].value, "c");
}

