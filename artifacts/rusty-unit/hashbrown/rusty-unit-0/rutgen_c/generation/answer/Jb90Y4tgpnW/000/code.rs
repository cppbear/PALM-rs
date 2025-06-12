// Answer 0

#[test]
fn test_get_many_unchecked_mut_non_overlapping_keys() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: std::alloc::Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<String, i32, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::default(),
        table: RawTable::default(),
    };

    map.insert("Key1".to_string(), 1);
    map.insert("Key2".to_string(), 2);
    
    let result: [Option<&mut i32>; 2] = unsafe {
        map.get_many_unchecked_mut(["Key1", "Key2"])
    };
    
    assert_eq!(result, [Some(&mut 1), Some(&mut 2)]);
}

#[test]
fn test_get_many_unchecked_mut_with_missing_key() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: std::alloc::Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<String, i32, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::default(),
        table: RawTable::default(),
    };

    map.insert("Key1".to_string(), 1);
    
    let result: [Option<&mut i32>; 2] = unsafe {
        map.get_many_unchecked_mut(["Key1", "Key2"])
    };

    assert_eq!(result, [Some(&mut 1), None]);
} 

#[test]
#[should_panic]
fn test_get_many_unchecked_mut_with_overlapping_keys() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: std::alloc::Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<String, i32, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::default(),
        table: RawTable::default(),
    };

    map.insert("Key1".to_string(), 1);
    
    let _: [Option<&mut i32>; 2] = unsafe {
        map.get_many_unchecked_mut(["Key1", "Key1"])
    };  // This should cause a panic due to overlapping keys
}

