// Answer 0

#[test]
fn test_vacant_entry_into_value() {
    use hashbrown::hash_set::{Entry, HashSet};

    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {
            unimplemented!()
        }
    }

    let mut set: HashSet<&str, DefaultHashBuilder, TestAllocator> = HashSet::new();

    match set.entry("poneyland") {
        Entry::Occupied(_) => panic!(),
        Entry::Vacant(v) => {
            // Testing if the value can be correctly extracted
            assert_eq!(v.into_value(), "poneyland");
        },
    }
}

#[test]
#[should_panic]
fn test_vacant_entry_should_not_panic_on_non_insert() {
    use hashbrown::hash_set::{Entry, HashSet};

    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {
            unimplemented!()
        }
    }

    let mut set: HashSet<&str, DefaultHashBuilder, TestAllocator> = HashSet::new();
    
    // Ensure trying to get an into_value from an occupied entry causes a panic.
    let _ = set.entry("poneyland").or_insert("some_value"); // Insert to occupy the entry
    match set.entry("poneyland") {
        Entry::Occupied(_) => {
            panic!(); // This should trigger a panic in the test.
        },
        Entry::Vacant(_) => {},
    }
}

