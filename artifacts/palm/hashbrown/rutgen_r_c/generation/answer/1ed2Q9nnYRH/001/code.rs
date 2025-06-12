// Answer 0

#[test]
fn test_entry_debug_vacant() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {
            // No-op for tests
        }
    }

    let mut map: HashMap<i32, i32, DefaultHashBuilder, TestAllocator> = HashMap::new();
    
    let key = 42;
    let vacant_entry = VacantEntry {
        hash: 0,
        key,
        table: &mut map,
    };

    let entry = Entry::Vacant(vacant_entry);
    
    let mut buffer = vec![];
    let result = entry.fmt(&mut std::fmt::Formatter::new(&mut buffer));
    
    assert!(result.is_ok());
    let formatted_str = String::from_utf8(buffer).unwrap();
    // Check if it contains "Entry"
    assert!(formatted_str.contains("Entry"));
} 

#[test]
fn test_entry_debug_occupied() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {
            // No-op for tests
        }
    }

    let mut map: HashMap<i32, i32, DefaultHashBuilder, TestAllocator> = HashMap::new();
    map.insert(42, 100); // Insert value to have an occupied entry

    let occupied_entry = OccupiedEntry {
        hash: 0,
        elem: Bucket::new((42, 100)), // Assume Bucket has a new method for this
        table: &mut map,
    };

    let entry = Entry::Occupied(occupied_entry);
    
    let mut buffer = vec![];
    let result = entry.fmt(&mut std::fmt::Formatter::new(&mut buffer));
    
    assert!(result.is_ok());
    let formatted_str = String::from_utf8(buffer).unwrap();
    // Check if it contains "Entry"
    assert!(formatted_str.contains("Entry"));
}

