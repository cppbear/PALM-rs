// Answer 0

#[test]
fn test_replace_existing_value() {
    use crate::{HashSet, DefaultHashBuilder};
    use std::hash::BuildHasher;

    struct MyAllocator;

    impl Allocator for MyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {
            // No-op for testing
        }
    }

    let mut set: HashSet<Vec<i32>, DefaultHashBuilder, MyAllocator> = HashSet::new();
    set.insert(Vec::<i32>::new());

    assert_eq!(set.get(&[][..]).unwrap().capacity(), 0);
    let replaced = set.replace(Vec::with_capacity(10));
    
    assert_eq!(replaced.map(|v| v.capacity()), Some(0));
    assert_eq!(set.get(&[][..]).unwrap().capacity(), 10);
}

#[test]
fn test_replace_new_value() {
    use crate::{HashSet, DefaultHashBuilder};
    use std::hash::BuildHasher;

    struct MyAllocator;

    impl Allocator for MyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {
            // No-op for testing
        }
    }

    let mut set: HashSet<Vec<i32>, DefaultHashBuilder, MyAllocator> = HashSet::new();
    let replaced = set.replace(Vec::with_capacity(5));

    assert!(replaced.is_none());
    assert_eq!(set.get(&[][..]).unwrap().capacity(), 5);
}

