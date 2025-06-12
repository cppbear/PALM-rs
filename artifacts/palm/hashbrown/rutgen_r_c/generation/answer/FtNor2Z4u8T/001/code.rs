// Answer 0

#[test]
fn test_get_occupied_entry() {
    use crate::raw::Global;
    use crate::HashMap;
    use crate::raw::Bucket;
    use std::ptr::NonNull;

    // Create a HashMap and insert a key-value pair.
    let mut map: HashMap<&str, u32, DefaultHashBuilder, Global> = HashMap::new();
    map.insert("poneyland", 12);

    // Create a suitable bucket to simulate an occupied entry.
    let bucket = Bucket {
        ptr: NonNull::new(map.get("poneyland").unwrap() as *const _ as *mut _).unwrap(),
    };

    // Create an occupied entry.
    let occupied_entry = OccupiedEntry {
        hash: 0,  // Hash would normally be calculated based on the key
        elem: bucket,
        table: &mut map,
    };

    // Verify that getting the value from the occupied entry works correctly.
    assert_eq!(occupied_entry.get(), &12);
}

#[test]
#[should_panic]
fn test_get_on_vacant_entry() {
    use crate::raw::Global;
    use crate::HashMap;
    use crate::raw::Bucket;
    use std::ptr::NonNull;

    // Create a HashMap with no entries.
    let mut map: HashMap<&str, u32, DefaultHashBuilder, Global> = HashMap::new();

    // Attempting to create a non-existing occupied entry (simulating a vacant one).
    let bucket = Bucket {
        ptr: NonNull::dangling(), // Invalid Bucket for a non-existing entry
    };

    let occupied_entry = OccupiedEntry {
        hash: 0,
        elem: bucket,
        table: &mut map,
    };

    // This should panic as we are attempting to get a value from a non-existent entry.
    // Note: In a real scenario, the function `get()` should handle the panic case more gracefully.
    let _ = occupied_entry.get();
}

