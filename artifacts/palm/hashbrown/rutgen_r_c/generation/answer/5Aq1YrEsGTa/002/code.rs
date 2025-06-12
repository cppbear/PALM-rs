// Answer 0

#[test]
fn test_retain_removes_elements_based_on_predicate() {
    struct MyAllocator;

    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, i32, DefaultHashBuilder, MyAllocator> =
        HashMap::with_capacity_and_hasher_in(8, DefaultHashBuilder::new(), MyAllocator);

    for x in 0..8 {
        map.insert(x, x * 10);
    }

    assert_eq!(map.len(), 8);

    map.retain(|&k, _| k % 2 == 0);

    assert_eq!(map.len(), 4);

    let vec: Vec<(i32, i32)> = map.iter().map(|(&k, &v)| (k, v)).collect();
    let mut expected_vec = vec![(0, 0), (2, 20), (4, 40), (6, 60)];
    expected_vec.sort_unstable();  
    assert_eq!(vec, expected_vec);
}

#[test]
fn test_retain_removes_nothing_if_all_elements_match_predicate() {
    struct MyAllocator;

    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, i32, DefaultHashBuilder, MyAllocator> =
        HashMap::with_capacity_and_hasher_in(4, DefaultHashBuilder::new(), MyAllocator);

    for x in 0..4 {
        map.insert(x, x * 10);
    }

    assert_eq!(map.len(), 4);

    map.retain(|&k, _| k < 4);

    assert_eq!(map.len(), 4);
}

#[test]
fn test_retain_removes_all_elements_if_no_element_matches() {
    struct MyAllocator;

    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, i32, DefaultHashBuilder, MyAllocator> =
        HashMap::with_capacity_and_hasher_in(4, DefaultHashBuilder::new(), MyAllocator);

    for x in 0..4 {
        map.insert(x, x * 10);
    }

    assert_eq!(map.len(), 4);

    map.retain(|&k, _| k > 4); // No keys are greater than 4.

    assert_eq!(map.len(), 0);
}

