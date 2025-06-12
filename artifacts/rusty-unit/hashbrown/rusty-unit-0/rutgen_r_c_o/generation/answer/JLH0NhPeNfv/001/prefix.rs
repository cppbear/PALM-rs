// Answer 0

#[test]
fn test_reserve_rehash_valid() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    struct TestType {
        data: i32,
    }

    impl Drop for TestType {
        fn drop(&mut self) {}
    }

    let allocator = TestAllocator;
    let mut table = RawTable::<TestType, _>::new_in(allocator);
    let additional = 1;
    let fallibility = Fallibility::Fallible;
    
    // Passes a hasher that computes a simple hash of TestType
    let hasher = |item: &TestType| item.data as u64;

    unsafe {
        let _ = table.reserve_rehash(additional, hasher, fallibility);
    }
}

#[test]
#[should_panic]
fn test_reserve_rehash_zero_additional() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    struct TestType {
        data: i32,
    }

    impl Drop for TestType {
        fn drop(&mut self) {}
    }

    let allocator = TestAllocator;
    let mut table = RawTable::<TestType, _>::new_in(allocator);
    let additional = 0; // This will trigger a panic.
    let fallibility = Fallibility::Fallible;
    let hasher = |item: &TestType| item.data as u64;

    unsafe {
        let _ = table.reserve_rehash(additional, hasher, fallibility);
    }
}

#[test]
#[should_panic]
fn test_reserve_rehash_exceeds_max() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    struct TestType {
        data: i32,
    }

    impl Drop for TestType {
        fn drop(&mut self) {}
    }

    let allocator = TestAllocator;
    let mut table = RawTable::<TestType, _>::new_in(allocator);
    let additional = usize::MAX; // This may cause panic or overflow.
    let fallibility = Fallibility::Fallible;
    let hasher = |item: &TestType| item.data as u64;

    unsafe {
        let _ = table.reserve_rehash(additional, hasher, fallibility);
    }
}

