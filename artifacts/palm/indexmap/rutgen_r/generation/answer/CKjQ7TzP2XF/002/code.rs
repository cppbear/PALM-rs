// Answer 0

#[derive(Default)]
struct CustomMap {
    indices: Vec<usize>,
    entries: Vec<usize>,
}

impl CustomMap {
    pub(crate) fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.indices
            .try_reserve(additional, get_hash(&self.entries))
            .map_err(TryReserveError::from_hashbrown)?;
        self.entries
            .try_reserve_exact(additional)
            .map_err(TryReserveError::from_alloc)
    }
}

// Dummy function to simulate get_hash behavior
fn get_hash(entries: &Vec<usize>) -> usize {
    entries.len() // Simplistic hash function
}

// Dummy error types to simulate original context
pub struct TryReserveError;

impl TryReserveError {
    pub fn from_hashbrown(_: ()) -> Self {
        TryReserveError
    }

    pub fn from_alloc(_: ()) -> Self {
        TryReserveError
    }
}

#[test]
fn test_try_reserve_exact_success() {
    let mut custom_map = CustomMap::default();
    
    // Initially empty map
    assert_eq!(custom_map.try_reserve_exact(5), Ok(()));
    
    // After reserving, ensure indices and entries can still be reserved
    assert_eq!(custom_map.indices.len(), 0);
    assert_eq!(custom_map.entries.len(), 0);
    
    // Try adding some initial entries
    custom_map.entries.push(1);
    custom_map.entries.push(2);
    
    // Now we should be able to reserve
    assert_eq!(custom_map.try_reserve_exact(3), Ok(()));
    assert_eq!(custom_map.indices.len(), 5); // assuming try_reserve updates length based on additional
    assert_eq!(custom_map.entries.len(), 2); // entries should still be the same
}

#[test]
#[should_panic]
fn test_try_reserve_exact_fail_on_over_reserve() {
    let mut custom_map = CustomMap::default();
    
    // Simulate a condition where indices cannot accommodate additional
    // Let's push to the indices directly to simulate over-reserving without handling the real allocation
    custom_map.indices.push(1);
    custom_map.indices.push(2);
    
    // Attempt to reserve exceeding current capacity
    custom_map.try_reserve_exact(10).unwrap();
}

#[test]
fn test_try_reserve_exact_when_empty() {
    let mut custom_map = CustomMap::default();
    
    assert_eq!(custom_map.try_reserve_exact(0), Ok(())); // Reserving zero should succeed
}

#[test]
fn test_try_reserve_exact_with_large_request() {
    let mut custom_map = CustomMap::default();
    
    // Test with a large number that should succeed
    assert_eq!(custom_map.try_reserve_exact(1000), Ok(()));
}

