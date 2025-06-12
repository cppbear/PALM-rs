// Answer 0

#[derive(Default)]
struct TestIndices {
    capacity: usize,
    len: usize,
}

impl TestIndices {
    fn try_reserve(&mut self, additional: usize, _hash: usize) -> Result<(), ()> {
        if self.len + additional > self.capacity {
            Err(())
        } else {
            self.len += additional;
            Ok(())
        }
    }
}

#[derive(Default)]
struct TestEntries {
    capacity: usize,
    len: usize,
}

impl TestEntries {
    fn capacity(&self) -> usize {
        self.capacity
    }

    fn len(&self) -> usize {
        self.len
    }
}

struct TestMap {
    indices: TestIndices,
    entries: TestEntries,
}

impl TestMap {
    fn try_reserve_entries(&mut self, _additional: usize) -> Result<(), ()> {
        Ok(())
    }

    fn try_reserve(&mut self, additional: usize) -> Result<(), ()> {
        self.indices
            .try_reserve(additional, 0)
            .map_err(|_| ())?;
        if additional > self.entries.capacity() - self.entries.len() {
            self.try_reserve_entries(additional)
        } else {
            Ok(())
        }
    }
}

#[test]
fn test_try_reserve_success() {
    let mut map = TestMap {
        indices: TestIndices {
            capacity: 10,
            len: 5,
        },
        entries: TestEntries {
            capacity: 10,
            len: 5,
        },
    };
    let result = map.try_reserve(3);
    assert!(result.is_ok());
}

#[test]
fn test_try_reserve_indices_error() {
    let mut map = TestMap {
        indices: TestIndices {
            capacity: 5,
            len: 5,
        },
        entries: TestEntries {
            capacity: 10,
            len: 5,
        },
    };
    let result = map.try_reserve(3);
    assert!(result.is_err());
}

#[test]
fn test_try_reserve_entries_needed() {
    let mut map = TestMap {
        indices: TestIndices {
            capacity: 10,
            len: 5,
        },
        entries: TestEntries {
            capacity: 5,
            len: 5,
        },
    };
    let result = map.try_reserve(3);
    assert!(result.is_ok());
}

#[test]
fn test_try_reserve_boundary_capacity() {
    let mut map = TestMap {
        indices: TestIndices {
            capacity: 5,
            len: 5,
        },
        entries: TestEntries {
            capacity: 5,
            len: 5,
        },
    };
    let result = map.try_reserve(0);
    assert!(result.is_ok());
}

