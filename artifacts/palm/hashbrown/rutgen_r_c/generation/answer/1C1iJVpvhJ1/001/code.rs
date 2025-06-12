// Answer 0

#[test]
fn test_into_iter_iter() {
    use crate::raw::{RawIntoIter, RawIter};
    use std::ptr::NonNull;
    use std::alloc::Layout;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let test_allocator = TestAllocator;
    let raw_iter = RawIntoIter {
        iter: RawIter {
            iter: core::ops::Range { start: 0, end: 3 },
            items: 3,
        },
        allocation: None,
        marker: PhantomData,
    };
    
    let into_iter = IntoIter { inner: raw_iter };
    
    let iterator = into_iter.iter();
    
    assert_eq!(iterator.inner.items, 3);
}

#[test]
fn test_into_iter_empty() {
    use crate::raw::{RawIntoIter, RawIter};
    use std::ptr::NonNull;
    use std::alloc::Layout;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let test_allocator = TestAllocator;
    let raw_iter = RawIntoIter {
        iter: RawIter {
            iter: core::ops::Range { start: 0, end: 0 },
            items: 0,
        },
        allocation: None,
        marker: PhantomData,
    };

    let into_iter = IntoIter { inner: raw_iter };
    
    let iterator = into_iter.iter();
    
    assert_eq!(iterator.inner.items, 0);
}

