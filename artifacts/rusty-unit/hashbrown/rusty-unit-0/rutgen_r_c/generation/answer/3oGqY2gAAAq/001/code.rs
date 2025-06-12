// Answer 0

#[test]
fn test_symmetric_difference_debug_empty() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    use std::collections::HashSet;
    use std::hash::BuildHasherDefault;

    let empty_set: HashSet<i32, BuildHasherDefault<fnv::FnvHasher>, DummyAllocator> = HashSet::new();
    let symmetric_difference = SymmetricDifference {
        iter: empty_set.difference(&empty_set).chain(empty_set.difference(&empty_set)),
    };

    let output = format!("{:?}", symmetric_difference);
    assert_eq!(output, "[]");
}

#[test]
fn test_symmetric_difference_debug_non_empty() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    use std::collections::HashSet;
    use std::hash::BuildHasherDefault;

    let set_a: HashSet<i32, BuildHasherDefault<fnv::FnvHasher>, DummyAllocator> = 
        [(1, ()), (2, ()), (3, ())].iter().cloned().collect();
    let set_b: HashSet<i32, BuildHasherDefault<fnv::FnvHasher>, DummyAllocator> = 
        [(2, ()), (3, ()), (4, ())].iter().cloned().collect();

    let symmetric_difference = SymmetricDifference {
        iter: set_a.difference(&set_b).chain(set_b.difference(&set_a)),
    };

    let output = format!("{:?}", symmetric_difference);
    assert_eq!(output, "[1, 4]");
}

#[test]
#[should_panic]
fn test_symmetric_difference_debug_panic() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    use std::collections::HashSet;
    use std::hash::BuildHasherDefault;

    let set_a: HashSet<i32, BuildHasherDefault<fnv::FnvHasher>, DummyAllocator> = 
        [(1, ()), (2, ()), (3, ())].iter().cloned().collect();
    let set_b: HashSet<i32, BuildHasherDefault<fnv::FnvHasher>, DummyAllocator> = 
        [(2, ()), (3, ()), (4, ())].iter().cloned().collect();

    // This will cause a panic due to invalid reference assuming the context had some invalid logic
    let symmetric_difference = SymmetricDifference {
        iter: set_a
            .difference(&set_b)
            .chain(set_b.difference(&set_a).take(0).map(|_| panic!("Panic!"))),
    };

    let _output = format!("{:?}", symmetric_difference);
}

