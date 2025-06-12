// Answer 0

#[test]
fn test_get_key_value_occupied_entry() {
    struct MyAllocator;
    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut table = RawTable {
        table: RawTableInner::default(),
        alloc: MyAllocator,
        marker: PhantomData,
    };

    let key = "key1";
    let value = 42;
    let bucket = Bucket {
        ptr: NonNull::from(&(key, value)),
    };

    let mut entry = RawOccupiedEntryMut {
        elem: bucket,
        table: &mut table,
        hash_builder: &(),
    };

    let (retrieved_key, retrieved_value) = entry.get_key_value();
    assert_eq!(retrieved_key, &key);
    assert_eq!(retrieved_value, &value);
}

#[test]
#[should_panic]
fn test_get_key_value_invalid_entry() {
    struct MyAllocator;
    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut table = RawTable {
        table: RawTableInner::default(),
        alloc: MyAllocator,
        marker: PhantomData,
    };

    // Create a bucket without valid pointer to trigger panic
    let bucket = Bucket {
        ptr: NonNull::dangling(),
    };

    let entry = RawOccupiedEntryMut {
        elem: bucket,
        table: &mut table,
        hash_builder: &(),
    };

    // Attempting to get key and value from an invalid entry
    let _ = entry.get_key_value();
}

