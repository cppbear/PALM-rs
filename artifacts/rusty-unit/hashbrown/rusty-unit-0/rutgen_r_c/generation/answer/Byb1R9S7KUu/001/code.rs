// Answer 0

#[test]
fn test_insert_entry() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            NonNull::new(Box::into_raw(Box::new(0u8))).ok_or(())
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
            Box::from_raw(ptr.as_ptr());
        }
    }

    let alloc = DummyAllocator;
    let mut table: RawTable<u8, DummyAllocator> = RawTable::new_in(alloc);

    let hasher = |&num: &u8| num as u64; // Simple hasher function

    // Inserting element with hash
    let hash = 42;
    let value = 10;
    let reference = table.insert_entry(hash, value, hasher);

    assert_eq!(*reference, 10);
}

#[test]
#[should_panic]
fn test_insert_entry_out_of_bounds() {
    struct PanicAllocator;

    unsafe impl Allocator for PanicAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            // Simulate allocation failure
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = PanicAllocator;
    let mut table: RawTable<u8, PanicAllocator> = RawTable::new_in(alloc);

    let hasher = |&num: &u8| num as u64; // Simple hasher function

    // Attempting to insert when allocation fails should panic
    let hash = 100;
    let value = 20;
    let _reference = table.insert_entry(hash, value, hasher);
}

#[test]
fn test_insert_entry_multiple_items() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            NonNull::new(Box::into_raw(Box::new(0u8))).ok_or(())
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
            Box::from_raw(ptr.as_ptr());
        }
    }

    let alloc = DummyAllocator;
    let mut table: RawTable<u8, DummyAllocator> = RawTable::new_in(alloc);

    let hasher = |&num: &u8| num as u64; // Simple hasher function

    // Inserting multiple items
    for i in 0..5 {
        let hash = i;
        let value = i * 10;
        let reference = table.insert_entry(hash, value, hasher);
        assert_eq!(*reference, value);
    }
}

