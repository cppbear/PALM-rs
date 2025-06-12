// Answer 0

#[derive(Debug)]
struct HeaderMap {
    entries: Vec<usize>,
    indices: Box<[usize]>,
    mask: usize,
}

#[derive(Debug)]
struct MaxSizeReached;

impl MaxSizeReached {
    fn new() -> Self {
        MaxSizeReached
    }
}

impl HeaderMap {
    fn new() -> Self {
        HeaderMap {
            entries: Vec::new(),
            indices: Box::new([]),
            mask: 0,
        }
    }

    fn try_reserve(&mut self, additional: usize) -> Result<(), MaxSizeReached> {
        let cap = self.entries.len().checked_add(additional).ok_or_else(MaxSizeReached::new)?;

        let raw_cap = to_raw_capacity(cap);

        if raw_cap > self.indices.len() {
            let raw_cap = raw_cap.checked_next_power_of_two().ok_or_else(MaxSizeReached::new)?;
            if raw_cap > MAX_SIZE {
                return Err(MaxSizeReached::new());
            }

            if self.entries.is_empty() {
                self.mask = raw_cap - 1;
                self.indices = vec![0; raw_cap].into_boxed_slice();
                self.entries = Vec::with_capacity(usable_capacity(raw_cap));
            } else {
                self.try_grow(raw_cap)?;
            }
        }

        Ok(())
    }
}

fn to_raw_capacity(cap: usize) -> usize {
    cap // Simplification for example purposes
}

fn usable_capacity(raw_cap: usize) -> usize {
    raw_cap // Simplification for example purposes
}

fn MAX_SIZE() -> usize {
    100 // Arbitrary maximum size for example purposes
}

impl HeaderMap {
    fn try_grow(&mut self, _raw_cap: usize) -> Result<(), MaxSizeReached> {
        Ok(()) // Simplification for example purposes
    }
}

#[test]
fn test_try_reserve_success() {
    let mut map = HeaderMap::new();
    assert!(map.try_reserve(10).is_ok());
}

#[test]
fn test_try_reserve_exceeding_max_size() {
    let mut map = HeaderMap::new();
    map.entries.resize(usize::MAX, 0); // Fill entries to likely exceed max size
    assert!(map.try_reserve(1).is_err());
}

#[test]
fn test_try_reserve_when_full() {
    let mut map = HeaderMap::new();
    map.try_reserve(50).unwrap(); // Assume it can grow initially
    map.entries.resize(50, 0);
    assert!(map.try_reserve(100).is_ok()); // This should succeed as it can grow
}

