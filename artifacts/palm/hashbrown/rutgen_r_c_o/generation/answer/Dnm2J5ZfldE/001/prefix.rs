// Answer 0

#[test]
fn test_allocator_with_default_capacity() {
    let set: HashSet<i32> = HashSet::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), Global);
    let allocator = set.allocator();
}

#[test]
fn test_allocator_with_small_capacity() {
    let set: HashSet<i32> = HashSet::with_capacity_and_hasher_in(1, DefaultHashBuilder::new(), Global);
    let allocator = set.allocator();
}

#[test]
fn test_allocator_with_large_capacity() {
    let set: HashSet<i32> = HashSet::with_capacity_and_hasher_in(1000, DefaultHashBuilder::new(), Global);
    let allocator = set.allocator();
}

#[test]
fn test_allocator_with_maximum_capacity() {
    let set: HashSet<i32> = HashSet::with_capacity_and_hasher_in(u32::MAX as usize, DefaultHashBuilder::new(), Global);
    let allocator = set.allocator();
}

#[test]
fn test_allocator_with_custom_allocator() {
    struct CustomAllocator;
    unsafe impl Allocator for CustomAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            NonNull::new(0 as *mut u8).ok_or(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let set: HashSet<i32, DefaultHashBuilder, CustomAllocator> = HashSet::with_capacity_and_hasher_in(10, DefaultHashBuilder::new(), CustomAllocator);
    let allocator = set.allocator();
}

