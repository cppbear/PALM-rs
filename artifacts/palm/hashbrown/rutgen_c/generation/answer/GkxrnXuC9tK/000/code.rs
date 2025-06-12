// Answer 0

#[test]
fn test_insert_unique_unchecked() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(),
    };

    unsafe {
        let (key, value) = map.insert_unique_unchecked(1, "a");
        assert_eq!(*key, 1);
        assert_eq!(*value, "a");

        let (key, value) = map.insert_unique_unchecked(2, "b");
        assert_eq!(*key, 2);
        assert_eq!(*value, "b");

        let (key, value) = map.insert_unique_unchecked(3, "c");
        assert_eq!(*key, 3);
        assert_eq!(*value, "c");
    }
}

#[test]
#[should_panic]
fn test_insert_unique_unchecked_with_existing_key() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(),
    };

    unsafe {
        let (key1, value1) = map.insert_unique_unchecked(1, "a");
        assert_eq!(*key1, 1);
        assert_eq!(*value1, "a");

        // Insertion of the same key to test panic
        let _ = map.insert_unique_unchecked(1, "b");
    }
}

