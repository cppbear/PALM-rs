// Answer 0

#[test]
fn test_get_inner_non_empty_table() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new(std::ptr::null_mut()).unwrap())
        }
        
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut hashmap: HashMap<i32, String, DefaultHashBuilder, MockAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: MockAllocator,
            marker: PhantomData,
        },
    };

    for i in 1..=10 {
        hashmap.insert(i, format!("value {}", i));
    }

    let key_to_lookup = &5;
    let result = hashmap.get_inner(key_to_lookup);
}

#[test]
fn test_get_inner_non_empty_table_with_string_keys() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new(std::ptr::null_mut()).unwrap())
        }
        
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut hashmap: HashMap<String, i32, DefaultHashBuilder, MockAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: MockAllocator,
            marker: PhantomData,
        },
    };

    hashmap.insert("key1".to_string(), 1);
    hashmap.insert("key2".to_string(), 2);
    hashmap.insert("key3".to_string(), 3);

    let key_to_lookup = &"key2".to_string();
    let result = hashmap.get_inner(key_to_lookup);
}

#[test]
fn test_get_inner_with_negative_keys_in_non_empty_table() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new(std::ptr::null_mut()).unwrap())
        }
        
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut hashmap: HashMap<i32, String, DefaultHashBuilder, MockAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: MockAllocator,
            marker: PhantomData,
        },
    };

    hashmap.insert(-1, "negative value".to_string());
    hashmap.insert(-2, "another negative value".to_string());

    let key_to_lookup = &-1;
    let result = hashmap.get_inner(key_to_lookup);
}

#[test]
fn test_get_inner_large_capacity() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new(std::ptr::null_mut()).unwrap())
        }
        
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut hashmap: HashMap<u64, usize, DefaultHashBuilder, MockAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: MockAllocator,
            marker: PhantomData,
        },
    };

    for i in 1..=1000 {
        hashmap.insert(i * 10, i);
    }

    let key_to_lookup = &500;
    let result = hashmap.get_inner(key_to_lookup);
}

