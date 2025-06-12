// Answer 0

#[test]
fn test_hash_set_new_in_with_global_allocator() {
    use crate::HashSet;
    use crate::Global;

    let allocator = Global;
    let set: HashSet<i32, _, Global> = HashSet::new_in(allocator);
    assert_eq!(set.map.table.len(), 0);
}

#[test]
fn test_hash_set_new_in_with_custom_allocator() {
    use crate::HashSet;
    use crate::Global;

    // Define a simple custom allocator struct for testing
    struct CustomAllocator;

    unsafe impl Allocator for CustomAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            // Mock allocation logic
            Ok(std::ptr::NonNull::new_unchecked(std::ptr::null_mut()))
        }
        
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: core::alloc::Layout) {
            // Mock deallocation logic
        }
    }

    let allocator = CustomAllocator;
    let set: HashSet<i32, _, CustomAllocator> = HashSet::new_in(allocator);
    assert_eq!(set.map.table.len(), 0);
}

