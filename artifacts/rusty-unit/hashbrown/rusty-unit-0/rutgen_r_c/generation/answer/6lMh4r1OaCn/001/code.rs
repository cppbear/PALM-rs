// Answer 0

#[test]
fn test_insert_unique_unchecked() {
    use core::hash::BuildHasherDefault;
    use std::collections::HashSet;

    struct MyAllocator;

    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut set: HashSet<i32, BuildHasherDefault<fnv::FnvHasher>, MyAllocator> = HashSet {
        map: HashMap {
            hash_builder: BuildHasherDefault::default(),
            table: RawTable::new(),
        },
    };

    unsafe {
        let value = 10;
        let result = set.insert_unique_unchecked(value);
        assert_eq!(*result, value);
    }

    unsafe {
        let value_same = 10;
        // If we run this again, it may panic due to the same value being inserted.
        // This is expected behavior, so we will just validate we are not catching existing values.
        let result_same = set.insert_unique_unchecked(value_same);
        assert_eq!(*result_same, value_same);
    }

    unsafe {
        let new_value = 20;
        let result_new = set.insert_unique_unchecked(new_value);
        assert_eq!(*result_new, new_value);
    }
}

#[should_panic]
#[test]
fn test_panic_on_duplicate_insert() {
    use core::hash::BuildHasherDefault;
    use std::collections::HashSet;

    struct MyAllocator;

    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut set: HashSet<i32, BuildHasherDefault<fnv::FnvHasher>, MyAllocator> = HashSet {
        map: HashMap {
            hash_builder: BuildHasherDefault::default(),
            table: RawTable::new(),
        },
    };

    unsafe {
        set.insert_unique_unchecked(42);
        set.insert_unique_unchecked(42); // This should trigger a panic
    }
}

