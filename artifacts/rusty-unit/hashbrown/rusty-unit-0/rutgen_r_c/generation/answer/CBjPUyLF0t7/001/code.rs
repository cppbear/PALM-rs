// Answer 0

#[test]
fn test_buckets_non_empty() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new(unsafe { std::mem::transmute::<*mut u8, *mut u8>(1 as *mut _) }).unwrap())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = TestAllocator;
    let table = RawTable::<i32, TestAllocator>::with_capacity_in(8, alloc);
    assert_eq!(table.buckets(), 1); // bucket_mask starts as 0 when initialized with capacity more than 0
}

#[test]
fn test_buckets_empty() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new(unsafe { std::mem::transmute::<*mut u8, *mut u8>(1 as *mut _) }).unwrap())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = TestAllocator;
    let table = RawTable::<i32, TestAllocator>::new_in(alloc);
    assert_eq!(table.buckets(), 1); // bucket_mask starts as 0 when initialized
}

