// Answer 0

#[derive(Default)]
struct TestStruct {
    indices: Vec<usize>,
    entries: Vec<usize>,
}

impl TestStruct {
    const MAX_ENTRIES_CAPACITY: usize = 100;

    fn try_reserve_entries(&mut self, additional: usize) -> Result<(), String> {
        let new_capacity = std::cmp::min(self.indices.capacity(), Self::MAX_ENTRIES_CAPACITY);
        let try_add = new_capacity - self.entries.len();
        if try_add > additional && self.entries.try_reserve_exact(try_add).is_ok() {
            return Ok(());
        }
        Err("Allocation error".to_string())
    }
}

#[test]
fn test_try_reserve_entries_success() {
    let mut test_struct = TestStruct::default();
    test_struct.indices.resize(100, 0); // Fill indices to max capacity
    test_struct.entries.resize(50, 0); // Fill entries to half capacity
    
    let additional = 30;
    let result = test_struct.try_reserve_entries(additional);
    
    assert!(result.is_ok());
}

#[test]
fn test_try_reserve_entries_boundary() {
    let mut test_struct = TestStruct::default();
    test_struct.indices.resize(100, 0); // Fill indices to max capacity
    test_struct.entries.resize(70, 0); // Fill entries to close to max capacity
    
    let additional = 30; // This should trigger the condition
    let result = test_struct.try_reserve_entries(additional);
    
    assert!(result.is_ok());
}

