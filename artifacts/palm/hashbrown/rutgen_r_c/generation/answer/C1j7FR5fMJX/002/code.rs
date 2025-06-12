// Answer 0

#[test]
fn test_bucket_valid() {
    use crate::alloc::alloc::{Allocator, Global};
    use core::alloc::Layout;
    use core::ptr::NonNull;

    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            // Simulating allocation
            Ok(NonNull::new_unchecked(std::alloc::alloc(layout)))
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            // Simulating deallocation
            std::alloc::dealloc(ptr.as_ptr(), layout);
        }
    }

    let allocator = DummyAllocator;
    let mut table: RawTable<u32, DummyAllocator> = RawTable::with_capacity_in(8, allocator);

    // Ensure the bucket count is correct after initialization
    assert_eq!(table.buckets(), 8);

    // Get valid bucket index 3
    let bucket = unsafe { table.bucket(3) };
    assert!(!bucket.ptr.as_ptr().is_null());
}

#[test]
#[should_panic(expected = "assertion failed: index < self.buckets()")]
fn test_bucket_out_of_bounds() {
    use crate::alloc::alloc::{Allocator, Global};
    use core::alloc::Layout;
    use core::ptr::NonNull;

    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::alloc::alloc(layout)))
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            std::alloc::dealloc(ptr.as_ptr(), layout);
        }
    }

    let allocator = DummyAllocator;
    let mut table: RawTable<u32, DummyAllocator> = RawTable::with_capacity_in(8, allocator);

    // Attempt to access bucket at an out-of-bounds index 10
    let _ = unsafe { table.bucket(10) };
}

#[test]
#[should_panic(expected = "assertion failed: self.table.bucket_mask != 0")]
fn test_bucket_empty_table() {
    use crate::alloc::alloc::{Allocator, Global};
    use core::alloc::Layout;
    use core::ptr::NonNull;

    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::alloc::alloc(layout)))
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            std::alloc::dealloc(ptr.as_ptr(), layout);
        }
    }

    let allocator = DummyAllocator;
    let table: RawTable<u32, DummyAllocator> = RawTable::new_in(allocator);

    // Try to access any bucket when the table is uninitialized (empty)
    let _ = unsafe { table.bucket(0) };
}

