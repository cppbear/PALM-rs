// Answer 0

#[test]
fn test_with_capacity_and_hasher_in_non_zero_capacity() {
    struct MyAllocator;

    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            std::alloc::alloc(layout);
            Ok(std::ptr::NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }
    
    let alloc = MyAllocator;
    let hash_builder = DefaultHashBuilder::default();
    let capacity = 10;

    let map: HashMap<i32, i32, DefaultHashBuilder, MyAllocator> = HashMap::with_capacity_and_hasher_in(capacity, hash_builder, alloc);

    assert_eq!(map.capacity(), capacity);
}

#[test]
fn test_with_capacity_and_hasher_in_zero_capacity() {
    struct MyAllocator;

    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let alloc = MyAllocator;
    let hash_builder = DefaultHashBuilder::default();
    let capacity = 0;

    let map: HashMap<i32, i32, DefaultHashBuilder, MyAllocator> = HashMap::with_capacity_and_hasher_in(capacity, hash_builder, alloc);

    assert_eq!(map.capacity(), capacity);
}

