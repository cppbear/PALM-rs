// Answer 0

#[test]
fn test_allocator_with_default_allocator() {
    let hashmap: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::new(), Global);
    let allocator = hashmap.allocator();
}

#[test]
fn test_allocator_with_custom_allocator() {
    struct CustomAllocator;

    unsafe impl Allocator for CustomAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            if layout.size() <= 4096 {
                Ok(NonNull::dangling())
            } else {
                Err(())
            }
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let hashmap: HashMap<i32, i32, DefaultHashBuilder, CustomAllocator> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder::new(), CustomAllocator);
    let allocator = hashmap.allocator();
}

#[test]
fn test_allocator_with_large_capacity() {
    let hashmap: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(1000000, DefaultHashBuilder::new(), Global);
    let allocator = hashmap.allocator();
}

#[test]
#[should_panic]
fn test_allocator_with_zero_capacity() {
    let hashmap: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), Global);
    let _allocator = hashmap.allocator();
}

#[test]
fn test_allocator_with_boundary_capacity() {
    let hashmap: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(4096, DefaultHashBuilder::new(), Global);
    let allocator = hashmap.allocator();
}

