// Answer 0

#[derive(Clone)]
struct Entry {
    hash: usize,
}

struct TestMap {
    entries: Vec<Entry>,
    indices: Vec<usize>,
}

impl TestMap {
    fn new(entries: Vec<Entry>, indices: Vec<usize>) -> Self {
        TestMap { entries, indices }
    }
    
    fn move_index(&mut self, from: usize, to: usize) {
        let from_hash = self.entries[from].hash;
        let _ = self.entries[to]; // explicit bounds check
        if from != to {
            // Use a sentinel index so other indices don't collide.
            update_index(&mut self.indices, from_hash, from, usize::MAX);

            // Update all other indices and rotate the entry positions.
            if from < to {
                self.decrement_indices(from + 1, to + 1);
                self.entries[from..=to].rotate_left(1);
            } else if to < from {
                self.increment_indices(to, from);
                self.entries[to..=from].rotate_right(1);
            }

            // Change the sentinel index to its final position.
            update_index(&mut self.indices, from_hash, usize::MAX, to);
        }
    }
    
    fn decrement_indices(&mut self, start: usize, end: usize) {
        for i in start..end {
            self.indices[i] -= 1;
        }
    }
    
    fn increment_indices(&mut self, start: usize, end: usize) {
        for i in start..end {
            self.indices[i] += 1;
        }
    }
}

fn update_index(indices: &mut Vec<usize>, hash: usize, old_index: usize, new_index: usize) {
    if old_index < indices.len() {
        indices[old_index] = new_index;
    }
}

#[test]
fn test_move_index_valid_case() {
    let mut map = TestMap::new(vec![Entry { hash: 1 }, Entry { hash: 2 }, Entry { hash: 3 }], vec![0, 1, 2]);
    map.move_index(1, 2);
    assert_eq!(map.entries[1].hash, 3);
    assert_eq!(map.entries[2].hash, 2);
}

#[test]
#[should_panic]
fn test_move_index_from_out_of_bounds() {
    let mut map = TestMap::new(vec![Entry { hash: 1 }, Entry { hash: 2 }], vec![0, 1]);
    map.move_index(2, 1); // 'from' index is out of bounds
}

#[test]
#[should_panic]
fn test_move_index_to_out_of_bounds() {
    let mut map = TestMap::new(vec![Entry { hash: 1 }, Entry { hash: 2 }], vec![0, 1]);
    map.move_index(0, 2); // 'to' index is out of bounds
}

#[test]
#[should_panic]
fn test_move_index_same_index() {
    let mut map = TestMap::new(vec![Entry { hash: 1 }, Entry { hash: 2 }], vec![0, 1]);
    map.move_index(1, 1); // 'from' and 'to' indices are the same
}

#[test]
fn test_move_index_boundary_case() {
    let mut map = TestMap::new(vec![Entry { hash: 1 }, Entry { hash: 2 }, Entry { hash: 3 }], vec![0, 1, 2]);
    map.move_index(0, 1);
    assert_eq!(map.entries[0].hash, 2);
    assert_eq!(map.entries[1].hash, 1);
}

