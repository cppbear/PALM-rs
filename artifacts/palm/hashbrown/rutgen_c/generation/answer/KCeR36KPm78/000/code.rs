// Answer 0

#[test]
fn test_iter_mut() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder, TestAllocator);
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    // Update all values
    for (_, val) in map.iter_mut() {
        *val *= 2;
    }

    assert_eq!(map.len(), 3);
    let mut vec: Vec<(&str, i32)> = Vec::new();

    for (key, val) in map.iter() {
        vec.push((*key, *val));
    }

    // Sort the items to test against a sorted array
    vec.sort_unstable();
    assert_eq!(vec, [("a", 2), ("b", 4), ("c", 6)]);
}

