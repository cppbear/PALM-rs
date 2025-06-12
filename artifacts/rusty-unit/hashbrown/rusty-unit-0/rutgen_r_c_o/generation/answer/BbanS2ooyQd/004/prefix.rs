// Answer 0

#[test]
fn test_shrink_to_zero_capacity() {
    struct MockAllocator;
    impl Allocator for MockAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(core::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = MockAllocator;
    let mut table = RawTable::<u32, _>::new_in(alloc);
    let hasher = |x: &u32| *x as u64;

    table.shrink_to(0, hasher);
}

#[test]
fn test_shrink_to_non_zero_with_empty_table() {
    struct MockAllocator;
    impl Allocator for MockAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(core::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = MockAllocator;
    let mut table = RawTable::<u32, _>::with_capacity_in(16, alloc);
    let hasher = |x: &u32| *x as u64;

    table.shrink_to(4, hasher);
}

#[test]
fn test_shrink_to_at_least_min_size() {
    struct MockAllocator;
    impl Allocator for MockAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(core::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = MockAllocator;
    let mut table = RawTable::<u32, _>::with_capacity_in(16, alloc);
    let hasher = |x: &u32| *x as u64;

    table.shrink_to(5, hasher);
}

#[test]
fn test_shrink_to_exceeding_current_buckets() {
    struct MockAllocator;
    impl Allocator for MockAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(core::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = MockAllocator;
    let mut table = RawTable::<u32, _>::with_capacity_in(16, alloc);
    let hasher = |x: &u32| *x as u64;

    table.shrink_to(3, hasher);
}

#[test]
fn test_shrink_to_no_effect_with_empty_table() {
    struct MockAllocator;
    impl Allocator for MockAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(core::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = MockAllocator;
    let mut table = RawTable::<u32, _>::new_in(alloc);
    let hasher = |x: &u32| *x as u64;

    table.shrink_to(6, hasher);
}

