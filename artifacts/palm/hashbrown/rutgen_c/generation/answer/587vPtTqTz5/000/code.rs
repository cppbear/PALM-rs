// Answer 0

#[test]
fn test_get_inner_key_not_found() {
    struct SimpleAllocator;

    impl Allocator for SimpleAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {
            unimplemented!()
        }
    }

    let mut hashmap: HashMap<i32, String, DefaultHashBuilder, SimpleAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::default(),
        table: RawTable {
            table: RawTableInner::new_empty(),
            alloc: SimpleAllocator,
            marker: PhantomData,
        },
    };

    let key = 1;
    let result: Option<&(i32, String)> = hashmap.get_inner(&key);
    assert!(result.is_none());
}

#[test]
fn test_get_inner_key_found() {
    struct SimpleAllocator;

    impl Allocator for SimpleAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {
            unimplemented!()
        }
    }

    let mut hashmap: HashMap<i32, String, DefaultHashBuilder, SimpleAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::default(),
        table: RawTable {
            table: RawTableInner::new_empty(),
            alloc: SimpleAllocator,
            marker: PhantomData,
        },
    };

    hashmap.insert(1, String::from("one"));
    
    let key = 1;
    let result: Option<&(i32, String)> = hashmap.get_inner(&key);
    assert!(result.is_some());
    if let Some((k, v)) = result {
        assert_eq!(*k, 1);
        assert_eq!(v, "one");
    }
}

#[test]
fn test_get_inner_empty_table() {
    struct SimpleAllocator;

    impl Allocator for SimpleAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {
            unimplemented!()
        }
    }

    let hashmap: HashMap<i32, String, DefaultHashBuilder, SimpleAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::default(),
        table: RawTable {
            table: RawTableInner::new_empty(),
            alloc: SimpleAllocator,
            marker: PhantomData,
        },
    };

    let key = 1;
    let result: Option<&(i32, String)> = hashmap.get_inner(&key);
    assert!(result.is_none());
}

