// Answer 0

#[test]
fn test_insert_unique_unchecked() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::default(),
    };

    unsafe {
        let (key1, value1) = map.insert_unique_unchecked(1, "value1");
        assert_eq!(*key1, 1);
        assert_eq!(*value1, "value1");

        let (key2, value2) = map.insert_unique_unchecked(2, "value2");
        assert_eq!(*key2, 2);
        assert_eq!(*value2, "value2");

        let (key3, value3) = map.insert_unique_unchecked(3, "value3");
        assert_eq!(*key3, 3);
        assert_eq!(*value3, "value3");

        // Attempt to insert a unique key, validating that we get back references to the correct items
        let (key4, value4) = map.insert_unique_unchecked(4, "value4");
        assert_eq!(*key4, 4);
        assert_eq!(*value4, "value4");

        // Ensure that we can still retrieve inserted values without panicking
        assert_eq!(map.get(&1), Some(&"value1"));
        assert_eq!(map.get(&2), Some(&"value2"));
        assert_eq!(map.get(&3), Some(&"value3"));
        assert_eq!(map.get(&4), Some(&"value4"));
    }
}

