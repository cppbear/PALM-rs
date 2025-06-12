// Answer 0

#[test]
fn test_values_mut_single_entry() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, i32, DefaultHashBuilder, TestAllocator> =
        HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::new(), TestAllocator);

    map.insert("a", 1);

    for val in map.values_mut() {
        *val += 10;
    }

    assert_eq!(map.len(), 1);
    assert_eq!(map.get("a"), Some(&11));
}

#[test]
fn test_values_mut_multiple_entries() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, i32, DefaultHashBuilder, TestAllocator> =
        HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::new(), TestAllocator);

    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    for val in map.values_mut() {
        *val += 10;
    }

    assert_eq!(map.len(), 3);
    let mut vec: Vec<i32> = Vec::new();

    for val in map.values() {
        vec.push(*val);
    }

    vec.sort_unstable();
    assert_eq!(vec, [11, 12, 13]);
}

#[test]
fn test_values_mut_empty_map() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, i32, DefaultHashBuilder, TestAllocator> =
        HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), TestAllocator);

    assert_eq!(map.len(), 0);
    let values_mut = map.values_mut();
    assert!(values_mut.inner.is_empty());
}

#[test]
fn test_values_mut_modify_non_existent() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, i32, DefaultHashBuilder, TestAllocator> =
        HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::new(), TestAllocator);

    for _ in 0..10 {
        map.insert("key", 5);
    }

    for val in map.values_mut() {
        *val += 5;
    }

    assert_eq!(map.len(), 1);
    assert_eq!(map.get("key"), Some(&10));
}

