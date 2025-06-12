// Answer 0

#[test]
fn test_hashmap_new_in_empty() {
    struct Bump;

    impl Allocator for Bump {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let bump = Bump;
    let map: HashMap<&str, i32, DefaultHashBuilder, Bump> = HashMap::new_in(bump);

    assert_eq!(map.len(), 0);
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_hashmap_new_in_insert() {
    struct Bump;

    impl Allocator for Bump {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let bump = Bump;
    let mut map: HashMap<&str, i32, DefaultHashBuilder, Bump> = HashMap::new_in(bump);

    map.insert("One", 1);
    assert_eq!(map.len(), 1);
    assert!(map.capacity() > 0);  // Note: Actual condition check depends on internal allocator logic
}

#[test]
fn test_hashmap_new_in_capacity_no_alloc() {
    struct Bump;

    impl Allocator for Bump {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let bump = Bump;
    let map: HashMap<&str, i32, DefaultHashBuilder, Bump> = HashMap::new_in(bump);

    assert_eq!(map.len(), 0);
    assert_eq!(map.capacity(), 0);
}

