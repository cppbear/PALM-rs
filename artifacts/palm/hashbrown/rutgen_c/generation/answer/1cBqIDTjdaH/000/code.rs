// Answer 0

#[test]
fn test_into_values() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new(std::alloc::alloc(std::alloc::Layout::from_size_align(0, 1).unwrap())).unwrap())
        }
        
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {
            std::alloc::dealloc(_ptr.as_ptr(), std::alloc::Layout::from_size_align(0, 1).unwrap());
        }
    }
    
    let mut map: HashMap<&str, i32, DefaultHashBuilder, TestAllocator> = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::new(), TestAllocator);

    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    let mut vec: Vec<i32> = map.into_values().collect();
    
    vec.sort_unstable();
    assert_eq!(vec, [1, 2, 3]);
}

#[test]
fn test_into_values_empty_map() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new(std::alloc::alloc(std::alloc::Layout::from_size_align(0, 1).unwrap())).unwrap())
        }
        
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {
            std::alloc::dealloc(_ptr.as_ptr(), std::alloc::Layout::from_size_align(0, 1).unwrap());
        }
    }

    let map: HashMap<&str, i32, DefaultHashBuilder, TestAllocator> = HashMap::new();
    
    let vec: Vec<i32> = map.into_values().collect();
    
    assert!(vec.is_empty());
}

