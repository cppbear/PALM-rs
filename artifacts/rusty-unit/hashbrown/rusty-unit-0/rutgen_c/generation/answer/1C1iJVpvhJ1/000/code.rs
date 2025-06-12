// Answer 0

#[test]
fn test_into_iter_creates_iter() {
    use crate::raw::{Global, RawIntoIter};
    use core::alloc::Layout;
    use core::ptr::NonNull;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let raw_iter: RawIntoIter<(i32, i32), TestAllocator> = RawIntoIter {
        iter: RawIter { iter: RawIterRange { start: 0, end: 0 }, items: 0 },
        allocation: None,
        marker: PhantomData,
    };

    let into_iter = IntoIter { inner: raw_iter };
    let iter = into_iter.iter();
    
    assert_eq!(iter.items, 0);
}

#[test]
fn test_iter_with_items() {
    use crate::raw::{Global, RawIntoIter};
    use core::alloc::Layout;
    use core::ptr::NonNull;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let raw_iter: RawIntoIter<(i32, i32), TestAllocator> = RawIntoIter {
        iter: RawIter { iter: RawIterRange { start: 0, end: 10 }, items: 10 },
        allocation: None,
        marker: PhantomData,
    };

    let into_iter = IntoIter { inner: raw_iter };
    let iter = into_iter.iter();
    
    assert_eq!(iter.items, 10);
}

