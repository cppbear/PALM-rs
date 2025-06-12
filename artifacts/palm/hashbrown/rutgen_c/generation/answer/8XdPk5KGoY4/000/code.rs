// Answer 0

#[test]
fn test_with_capacity_in() {
    use bumpalo::Bump;
    
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            todo!()
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            todo!()
        }
    }

    let bump = Bump::new();
    let mut map = HashMap::with_capacity_in(5, bump);

    // The created HashMap holds no elements
    assert_eq!(map.len(), 0);
    // But it can hold at least 5 elements without reallocating
    let empty_map_capacity = map.capacity();
    assert!(empty_map_capacity >= 5);

    // Now we insert some 5 elements inside created HashMap
    map.insert("One", 1);
    map.insert("Two", 2);
    map.insert("Three", 3);
    map.insert("Four", 4);
    map.insert("Five", 5);

    // We can see that the HashMap holds 5 elements
    assert_eq!(map.len(), 5);
    // But its capacity isn't changed
    assert_eq!(map.capacity(), empty_map_capacity);
}

#[test]
fn test_with_capacity_in_zero_capacity() {
    use bumpalo::Bump;
    
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            todo!()
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            todo!()
        }
    }

    let bump = Bump::new();
    let map = HashMap::with_capacity_in(0, bump);
    
    // The created HashMap should hold no elements
    assert_eq!(map.len(), 0);
    // Capacity should also be 0 since we did not allocate
    assert_eq!(map.capacity(), 0);
}

