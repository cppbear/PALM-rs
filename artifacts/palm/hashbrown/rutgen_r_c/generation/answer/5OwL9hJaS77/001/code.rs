// Answer 0

#[test]
fn test_or_insert_vacant_entry() {
    use hashbrown::{hash_map::RawEntryMut, HashMap};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasherDefault;

    // Define a struct to implement the Allocator trait
    struct AllocatorMock;

    impl Allocator for AllocatorMock {
        unsafe fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            unimplemented!() // Mock implementation
        }
        
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {
            unimplemented!() // Mock implementation
        }
    }

    // Create a HashMap with appropriate types
    let mut map: HashMap<&str, u32, BuildHasherDefault<DefaultHasher>, AllocatorMock> = HashMap::new();
    
    // Check the initial state, map should be empty
    assert!(map.is_empty());

    // Call or_insert on a vacant entry
    let (key, value) = map.raw_entry_mut().from_key("poneyland").or_insert("poneyland", 3);
    
    // Assert that values are correctly inserted
    assert_eq!(*key, "poneyland");
    assert_eq!(*value, 3);    
    assert_eq!(map["poneyland"], 3);

    // Modify the value via the mutable reference
    *value *= 2;

    // Assert the changes are reflected in the map
    assert_eq!(map["poneyland"], 6);
}

#[test]
fn test_or_insert_with_empty_map() {
    use hashbrown::{hash_map::RawEntryMut, HashMap};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasherDefault;

    struct AllocatorMock;

    impl Allocator for AllocatorMock {
        unsafe fn allocate(&self, _: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            unimplemented!() // Mock implementation
        }

        unsafe fn deallocate(&self, _: std::ptr::NonNull<u8>, _: std::alloc::Layout) {
            unimplemented!() // Mock implementation
        }
    }

    let mut map: HashMap<&str, u32, BuildHasherDefault<DefaultHasher>, AllocatorMock> = HashMap::new();
    
    assert!(
        map.raw_entry_mut().from_key("test").is_vacant()
    );

    let (key, value) = map.raw_entry_mut().from_key("test").or_insert("test", 42);

    assert_eq!(*key, "test");
    assert_eq!(*value, 42);
    assert_eq!(map["test"], 42);
}

#[test]
fn test_or_insert_multiple_entries() {
    use hashbrown::{hash_map::RawEntryMut, HashMap};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasherDefault;

    struct AllocatorMock;

    impl Allocator for AllocatorMock {
        unsafe fn allocate(&self, _: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            unimplemented!() // Mock implementation
        }

        unsafe fn deallocate(&self, _: std::ptr::NonNull<u8>, _: std::alloc::Layout) {
            unimplemented!() // Mock implementation
        }
    }

    let mut map: HashMap<&str, u32, BuildHasherDefault<DefaultHasher>, AllocatorMock> = HashMap::new();

    let (key1, value1) = map.raw_entry_mut().from_key("first").or_insert("first", 1);
    let (key2, value2) = map.raw_entry_mut().from_key("second").or_insert("second", 2);

    assert_eq!(*key1, "first");
    assert_eq!(*value1, 1);
    assert_eq!(map["first"], 1);

    assert_eq!(*key2, "second");
    assert_eq!(*value2, 2);
    assert_eq!(map["second"], 2);
}

