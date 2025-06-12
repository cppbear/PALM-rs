// Answer 0

#[derive(Default)]
struct Indices {
    reserved: usize,
}

impl Indices {
    fn try_reserve(&mut self, additional: usize, _hash: usize) -> Result<(), &'static str> {
        if self.reserved + additional > 100 {
            return Err("Exceeded maximum capacity");
        }
        self.reserved += additional;
        Ok(())
    }
}

#[derive(Default)]
struct Entries {
    reserved: usize,
}

impl Entries {
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), &'static str> {
        if self.reserved + additional > 100 {
            return Err("Exceeded maximum capacity");
        }
        self.reserved += additional;
        Ok(())
    }
}

#[derive(Default)]
struct MyMap {
    indices: Indices,
    entries: Entries,
}

struct TryReserveError;

impl TryReserveError {
    fn from_hashbrown(_: &'static str) -> Self {
        TryReserveError
    }

    fn from_alloc(_: &'static str) -> Self {
        TryReserveError
    }
}

fn get_hash(_entries: &Entries) -> usize {
    0
}

impl MyMap {
    pub(crate) fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.indices
            .try_reserve(additional, get_hash(&self.entries))
            .map_err(TryReserveError::from_hashbrown)?;
        self.entries
            .try_reserve_exact(additional)
            .map_err(TryReserveError::from_alloc)
    }
}

#[test]
fn test_try_reserve_exact_success() {
    let mut map = MyMap::default();
    assert!(map.try_reserve_exact(10).is_ok());
}

#[test]
fn test_try_reserve_exact_exceed_indices_capacity() {
    let mut map = MyMap::default();
    map.indices.reserved = 95; // Set initial reserved to near the limit
    assert_eq!(map.try_reserve_exact(10).is_err(), true);
}

#[test]
fn test_try_reserve_exact_exceed_entries_capacity() {
    let mut map = MyMap::default();
    map.entries.reserved = 95; // Set initial reserved to near the limit
    assert_eq!(map.try_reserve_exact(10).is_err(), true);
}

