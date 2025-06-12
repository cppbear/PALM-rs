// Answer 0

#[test]
fn test_with_hasher_in() {
    use hashbrown::HashSet;
    use hashbrown::DefaultHashBuilder;
    use std::alloc::System;

    struct Alloc;

    impl hashbrown::hash_map::Alloc for Alloc {
        fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
            unsafe { System.alloc(layout) }
        }

        fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
            unsafe { System.dealloc(ptr, layout) }
        }
    }

    let hasher = DefaultHashBuilder::default();
    let alloc = Alloc;
    let mut set: HashSet<i32, _, Alloc> = HashSet::with_hasher_in(hasher, alloc);

    assert!(set.is_empty());
    set.insert(5);
    assert_eq!(set.len(), 1);
    set.insert(10);
    assert_eq!(set.len(), 2);
    assert!(set.contains(&5));
    assert!(set.contains(&10));
}

#[test]
fn test_with_hasher_in_empty_set() {
    use hashbrown::HashSet;
    use hashbrown::DefaultHashBuilder;
    use std::alloc::System;

    struct Alloc;

    impl hashbrown::hash_map::Alloc for Alloc {
        fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
            unsafe { System.alloc(layout) }
        }

        fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
            unsafe { System.dealloc(ptr, layout) }
        }
    }

    let hasher = DefaultHashBuilder::default();
    let alloc = Alloc;
    let set: HashSet<i32, _, Alloc> = HashSet::with_hasher_in(hasher, alloc);

    assert!(set.is_empty());
}

#[test]
#[should_panic]
fn test_with_hasher_in_panic_alloc() {
    use hashbrown::HashSet;
    use hashbrown::DefaultHashBuilder;
    
    struct FaultyAlloc;

    impl hashbrown::hash_map::Alloc for FaultyAlloc {
        fn alloc(&self, _layout: std::alloc::Layout) -> *mut u8 {
            panic!("Allocation failure");
        }

        fn dealloc(&self, _ptr: *mut u8, _layout: std::alloc::Layout) {
            panic!("Deallocation failure");
        }
    }

    let hasher = DefaultHashBuilder::default();
    let alloc = FaultyAlloc;
    let mut set: HashSet<i32, _, FaultyAlloc> = HashSet::with_hasher_in(hasher, alloc);
    set.insert(1);
}

