// Answer 0

#[test]
fn iter_test() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: std::alloc::Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<&str, i32, DefaultHashBuilder, TestAllocator> =
        HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder, TestAllocator);
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    
    assert_eq!(map.len(), 3);
    let mut vec: Vec<(&str, i32)> = Vec::new();

    for (key, val) in map.iter() {
        vec.push((*key, *val));
    }

    vec.sort_unstable();
    assert_eq!(vec, [("a", 1), ("b", 2), ("c", 3)]);
    assert_eq!(map.len(), 3);
}

