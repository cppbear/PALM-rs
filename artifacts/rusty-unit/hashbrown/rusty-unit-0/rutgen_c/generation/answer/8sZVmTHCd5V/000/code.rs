// Answer 0

#[test]
fn test_allocator() {
    // Define a simple allocator struct for testing
    struct TestAllocator;

    // Implement the Allocator trait for the TestAllocator
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::new_unchecked(core::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    // Create a HashMap with a TestAllocator
    let alloc = TestAllocator;
    let map: HashMap<i32, i32, DefaultHashBuilder, TestAllocator> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder::default(), alloc);

    // Test that the allocator function returns the correct allocator
    assert!(std::ptr::eq(map.allocator(), &alloc));
}

#[test]
fn test_allocator_default() {
    // Use the default allocator from the HashMap
    let map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder::default(), Global);
    
    // Test that the allocator function returns a reference to the default global allocator
    assert!(std::ptr::eq(map.allocator(), &Global));
}

