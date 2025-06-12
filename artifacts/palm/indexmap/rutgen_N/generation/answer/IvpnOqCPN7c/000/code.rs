// Answer 0

#[derive(Clone)]
struct Entry {
    hash: usize,
    // other fields...
}

struct TestMap {
    entries: Vec<Entry>,
    indices: Vec<usize>,
}

impl TestMap {
    fn new() -> Self {
        TestMap {
            entries: Vec::new(),
            indices: Vec::new(),
        }
    }
    
    fn move_index(&mut self, from: usize, to: usize) {
        let from_hash = self.entries[from].hash;
        let _ = self.entries[to]; // explicit bounds check
        if from != to {
            update_index(&mut self.indices, from_hash, from, usize::MAX);

            if from < to {
                self.decrement_indices(from + 1, to + 1);
                self.entries[from..=to].rotate_left(1);
            } else if to < from {
                self.increment_indices(to, from);
                self.entries[to..=from].rotate_right(1);
            }

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
    // Imagine we update the index mapping here.
}

#[test]
fn test_move_index() {
    let mut map = TestMap::new();
    map.entries.push(Entry { hash: 1 });
    map.entries.push(Entry { hash: 2 });
    map.entries.push(Entry { hash: 3 });

    map.move_index(1, 2);
    assert_eq!(map.entries[1].hash, 3);
    assert_eq!(map.entries[2].hash, 2);
}

#[test]
fn test_move_index_same_position() {
    let mut map = TestMap::new();
    map.entries.push(Entry { hash: 1 });
    map.entries.push(Entry { hash: 2 });

    map.move_index(0, 0);
    assert_eq!(map.entries[0].hash, 1);
}

#[test]
fn test_move_index_boundary() {
    let mut map = TestMap::new();
    map.entries.push(Entry { hash: 1 });
    map.entries.push(Entry { hash: 2 });
    map.entries.push(Entry { hash: 3 });

    map.move_index(0, 2);
    assert_eq!(map.entries[0].hash, 2);
    assert_eq!(map.entries[1].hash, 3);
    assert_eq!(map.entries[2].hash, 1);
}

