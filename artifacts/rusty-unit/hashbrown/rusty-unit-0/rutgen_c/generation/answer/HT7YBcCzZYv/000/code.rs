// Answer 0

#[test]
fn test_find_or_find_insert_slot_found() {
    struct MyAllocator;

    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<u64, MyAllocator> = RawTable::with_capacity_in(4, MyAllocator);
    let hash = 42;

    let _ = table.insert(hash, 1, |x| *x);
    let result = table.find_or_find_insert_slot(hash, |&x| x == 1, |x| *x);

    assert!(result.is_ok());
}

#[test]
fn test_find_or_find_insert_slot_insert() {
    struct MyAllocator;

    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<u64, MyAllocator> = RawTable::with_capacity_in(4, MyAllocator);
    let hash = 42;

    let result = table.find_or_find_insert_slot(hash, |&x| x == 0, |x| *x);

    assert!(result.is_err());
}

#[test]
fn test_find_or_find_insert_slot_empty() {
    struct MyAllocator;

    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<u64, MyAllocator> = RawTable::with_capacity_in(4, MyAllocator);
    let hash = 99;

    let result = table.find_or_find_insert_slot(hash, |&x| x == 0, |x| *x);

    assert!(result.is_err());
}

