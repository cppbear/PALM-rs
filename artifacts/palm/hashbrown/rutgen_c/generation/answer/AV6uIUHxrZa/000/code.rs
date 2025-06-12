// Answer 0

#[test]
fn test_remove_existing_value() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct SimpleAllocator;

    impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut set: HashSet<i32, DefaultHasher, SimpleAllocator> = HashSet {
        map: HashMap {
            hash_builder: DefaultHasher::new(),
            table: RawTable::new(),
        },
    };

    set.insert(2);
    assert_eq!(set.remove(&2), true);
    assert_eq!(set.remove(&2), false);
}

#[test]
fn test_remove_nonexistent_value() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct SimpleAllocator;

    impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut set: HashSet<i32, DefaultHasher, SimpleAllocator> = HashSet {
        map: HashMap {
            hash_builder: DefaultHasher::new(),
            table: RawTable::new(),
        },
    };

    assert_eq!(set.remove(&3), false);
}

#[test]
fn test_remove_when_empty() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct SimpleAllocator;

    impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut set: HashSet<i32, DefaultHasher, SimpleAllocator> = HashSet {
        map: HashMap {
            hash_builder: DefaultHasher::new(),
            table: RawTable::new(),
        },
    };

    assert_eq!(set.remove(&1), false);
}

