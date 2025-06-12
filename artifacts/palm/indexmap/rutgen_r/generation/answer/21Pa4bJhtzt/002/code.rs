// Answer 0

#[derive(Default)]
struct Indices {
    capacity: usize,
    len: usize,
}

impl Indices {
    fn reserve(&mut self, additional: usize, _: fn()) {
        self.capacity += additional;
    }
}

#[derive(Default)]
struct Entries {
    capacity: usize,
    len: usize,
}

impl Entries {
    fn capacity(&self) -> usize {
        self.capacity
    }

    fn len(&self) -> usize {
        self.len
    }

    fn reserve(&mut self, additional: usize) {
        self.capacity += additional;
    }
}

struct IndexMap {
    indices: Indices,
    entries: Entries,
}

impl IndexMap {
    fn borrow_mut(&mut self) -> &mut Entries {
        &mut self.entries
    }

    pub(crate) fn reserve(&mut self, additional: usize) {
        self.indices.reserve(additional, || {});
        if additional > self.entries.capacity() - self.entries.len() {
            self.borrow_mut().reserve(additional);
        }
    }
}

#[test]
fn test_reserve_no_growth_needed() {
    let mut index_map = IndexMap {
        indices: Indices { capacity: 10, len: 5 },
        entries: Entries { capacity: 10, len: 5 },
    };
    index_map.reserve(5);
    assert_eq!(index_map.indices.capacity, 15);
    assert_eq!(index_map.entries.capacity, 10);
}

#[test]
fn test_reserve_growth_needed() {
    let mut index_map = IndexMap {
        indices: Indices { capacity: 10, len: 5 },
        entries: Entries { capacity: 10, len: 9 },
    };
    index_map.reserve(1);
    assert_eq!(index_map.indices.capacity, 11);
    assert_eq!(index_map.entries.capacity, 11);
}

