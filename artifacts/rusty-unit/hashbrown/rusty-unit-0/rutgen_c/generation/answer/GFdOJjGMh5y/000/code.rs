// Answer 0

#[test]
fn test_get_key_value_mut_existing_key() {
    struct SimpleAllocator;

    impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, SimpleAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: SimpleAllocator,
            marker: PhantomData,
        },
    };

    map.insert(1, "a");
    let (k, v) = map.get_key_value_mut(&1).unwrap();
    assert_eq!(k, &1);
    assert_eq!(*v, "a");
    *v = "b";
    assert_eq!(map.get_key_value_mut(&1).unwrap().1, &mut "b");
}

#[test]
fn test_get_key_value_mut_non_existing_key() {
    struct SimpleAllocator;

    impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, SimpleAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: SimpleAllocator,
            marker: PhantomData,
        },
    };

    map.insert(1, "a");
    assert_eq!(map.get_key_value_mut(&2), None);
}

