// Answer 0

#[test]
fn test_bucket_valid_index_non_zero_sized() {
    use crate::alloc::alloc::{Layout, Global};
    use std::ptr::NonNull;

    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            // Mock allocation, return a valid pointer
            Ok(NonNull::new_unchecked(std::alloc::alloc(layout)))
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            std::alloc::dealloc(ptr.as_ptr(), layout);
        }
    }

    let alloc = MockAllocator;
    let mut table = RawTable::with_capacity_in(4, alloc);
    let index = table.buckets(); // This should be equal to the number of buckets.

    unsafe {
        // This should panic due to index being equal to the number of buckets.
        let _bucket = table.bucket(index);
    }
}

#[test]
#[should_panic]
fn test_bucket_panic_on_invalid_index_non_zero_sized() {
    use crate::alloc::alloc::{Layout, Global};
    use std::ptr::NonNull;

    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::alloc::alloc(layout)))
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            std::alloc::dealloc(ptr.as_ptr(), layout);
        }
    }

    let alloc = MockAllocator;
    let mut table = RawTable::with_capacity_in(4, alloc);
    let index = table.buckets(); // This should be equal to the number of buckets.

    unsafe {
        // This should panic because the index is out of bounds.
        let _bucket = table.bucket(index);
    }
}

#[test]
fn test_bucket_zero_sized() {
    struct ZeroSized;

    let alloc = Global;
    let mut table = RawTable::<ZeroSized, _>::with_capacity_in(4, alloc);
    let index = 0;

    unsafe {
        let bucket = table.bucket(index);
        // Ensure the bucket is created successfully for a zero-sized type
        assert!(!bucket.ptr.as_ptr().is_null());
    }
}

