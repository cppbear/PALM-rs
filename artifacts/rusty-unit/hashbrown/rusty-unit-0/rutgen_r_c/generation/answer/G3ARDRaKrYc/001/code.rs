// Answer 0

#[test]
fn test_remove_entry_success() {
    use core::alloc::Layout;

    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut table: RawTable<i32, DummyAllocator> = RawTable::new_in(DummyAllocator);
    
    // Insert a value into the table to configure the state as needed
    let hash = 42;
    let value = 99;
    table.insert(hash, value, |&v| v as u64); // Insert an entry with hash
    
    let result = table.remove_entry(hash, |&v| v == value); // Try to remove it

    assert_eq!(result, Some(value)); // Confirm we received the correct value
}

#[test]
fn test_remove_entry_not_found() {
    use core::alloc::Layout;

    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut table: RawTable<i32, DummyAllocator> = RawTable::new_in(DummyAllocator);

    let hash = 99; // A hash that doesn't exist in the table

    // Attempt to remove an entry that is not present
    let result = table.remove_entry(hash, |&_v| false);

    assert_eq!(result, None); // Confirm that the result is None
}

