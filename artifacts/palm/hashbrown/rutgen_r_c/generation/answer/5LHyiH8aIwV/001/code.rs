// Answer 0

#[test]
fn test_get_many_key_value_unchecked_mut_all_present() {
    use crate::raw::Global;

    struct MyAllocator;

    impl Allocator for MyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<String, i32, DefaultHashBuilder, MyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(),
    };
    map.insert("apple".to_string(), 1);
    map.insert("banana".to_string(), 2);

    let keys = ["apple", "banana"];
    let result = unsafe { map.get_many_key_value_unchecked_mut(&keys) };

    assert_eq!(result, [
        Some((&"apple".to_string(), &mut 1)),
        Some((&"banana".to_string(), &mut 2)),
    ]);
}

#[test]
fn test_get_many_key_value_unchecked_mut_some_missing() {
    use crate::raw::Global;

    struct MyAllocator;

    impl Allocator for MyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<String, i32, DefaultHashBuilder, MyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(),
    };
    map.insert("apple".to_string(), 1);
    
    let keys = ["apple", "banana"];
    let result = unsafe { map.get_many_key_value_unchecked_mut(&keys) };

    assert_eq!(result, [
        Some((&"apple".to_string(), &mut 1)),
        None,
    ]);
}

#[test]
#[should_panic]
fn test_get_many_key_value_unchecked_mut_overlapping_keys() {
    use crate::raw::Global;

    struct MyAllocator;

    impl Allocator for MyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<String, i32, DefaultHashBuilder, MyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(),
    };
    map.insert("apple".to_string(), 1);
    let keys = ["apple", "apple"];
    let _ = unsafe { map.get_many_key_value_unchecked_mut(&keys) };
}

