// Answer 0

#[test]
fn test_allocator_with_default_allocator() {
    struct CustomAllocator;

    unsafe impl Allocator for CustomAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = CustomAllocator;
    let set: HashSet<i32, DefaultHashBuilder, CustomAllocator> =
        HashSet::with_hasher_in(DefaultHashBuilder::new(), allocator);
    let alloc_ref = set.allocator();
    assert_eq!(std::ptr::eq(alloc_ref as *const _, &allocator as *const _), true);
}

#[test]
fn test_allocator_with_custom_capacity_and_allocator() {
    struct CustomAllocator;

    unsafe impl Allocator for CustomAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let capacity = 10;
    let allocator = CustomAllocator;
    let set: HashSet<i32, DefaultHashBuilder, CustomAllocator> =
        HashSet::with_capacity_and_hasher_in(capacity, DefaultHashBuilder::new(), allocator);
    let alloc_ref = set.allocator();
    assert_eq!(std::ptr::eq(alloc_ref as *const _, &allocator as *const _), true);
}

