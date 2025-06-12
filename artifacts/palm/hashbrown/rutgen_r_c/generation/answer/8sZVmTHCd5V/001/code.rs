// Answer 0

#[test]
fn test_allocator() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            // Simplified allocation logic for testing
            NonNull::new_unchecked(0 as *mut u8).ok_or(())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // No operation
        }
    }

    let test_allocator = TestAllocator;
    let hashmap: HashMap<i32, i32, DefaultHashBuilder, TestAllocator> = HashMap::with_hasher_in(DefaultHashBuilder::default(), test_allocator);

    let allocator_ref = hashmap.allocator();
    assert_eq!(allocator_ref as *const _, &test_allocator as *const _);
}

#[test]
fn test_allocator_with_different_capacity() {
    struct SimpleAllocator;

    unsafe impl Allocator for SimpleAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            NonNull::new_unchecked(0 as *mut u8).ok_or(())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // No operation
        }
    }

    let simple_allocator = SimpleAllocator;
    let hashmap: HashMap<u32, u32, DefaultHashBuilder, SimpleAllocator> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder::default(), simple_allocator);

    let allocator_ref = hashmap.allocator();
    assert_eq!(allocator_ref as *const _, &simple_allocator as *const _);
}

