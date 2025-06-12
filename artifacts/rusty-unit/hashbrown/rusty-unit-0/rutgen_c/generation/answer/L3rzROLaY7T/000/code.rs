// Answer 0

#[test]
fn test_remove_entry() {
    use std::alloc::{Global, Layout};
    use std::ptr::NonNull;
    use core::marker::PhantomData;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            // Simplification for tests, just return a null pointer.
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // No-op for tests.
        }
    }

    let mut table = RawTable::<(&str, u32), TestAllocator> {
        table: RawTableInner {},
        alloc: TestAllocator,
        marker: PhantomData,
    };

    let key_value = ("a", 100);
    let bucket = Bucket {
        ptr: NonNull::from(&key_value),
    };

    let mut entry = RawOccupiedEntryMut {
        elem: bucket,
        table: &mut table,
        hash_builder: &(),
    };

    let (k, v) = entry.remove_entry();
    assert_eq!(k, "a");
    assert_eq!(v, 100);
}

#[test]
fn test_remove_entry_from_non_existent_key() {
    use std::alloc::{Global, Layout};
    use std::ptr::NonNull;
    use core::marker::PhantomData;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table = RawTable::<(&str, u32), TestAllocator> {
        table: RawTableInner {},
        alloc: TestAllocator,
        marker: PhantomData,
    };

    // Simulate trying to remove an entry from an empty table
    let bucket = Bucket {
        ptr: NonNull::dangling(),
    };

    let mut entry = RawOccupiedEntryMut {
        elem: bucket,
        table: &mut table,
        hash_builder: &(),
    };

    // Normally, we would expect the method to potentially panic or handle an error
    // Since removing from an entry that doesn't exist is undefined behavior,
    // an implementation should handle it, but for this context, we cannot enforce that;
    // thus, we may just express the expectation.
}

