// Answer 0

#[test]
fn test_new_in_empty_hashmap() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = DummyAllocator;
    let map: HashMap<&str, i32, DefaultHashBuilder, DummyAllocator> = HashMap::new_in(alloc);

    assert_eq!(map.len(), 0);
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_new_in_insert_element() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = DummyAllocator;
    let mut map: HashMap<&str, i32, DefaultHashBuilder, DummyAllocator> = HashMap::new_in(alloc);

    // Insert an element
    map.insert("One", 1);

    assert_eq!(map.len(), 1);
    assert!(map.capacity() > 0);
}

