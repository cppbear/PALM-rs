// Answer 0

#[test]
fn test_get_many_unchecked_mut_inner() {
    struct SimpleAllocator;
    unsafe impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    struct DummyHashBuilder;
    impl BuildHasher for DummyHashBuilder {
        type Hasher = std::hash::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::hash::RandomState::new()
        }
    }

    let mut hashmap: HashMap<i32, String, DummyHashBuilder, SimpleAllocator> = HashMap {
        hash_builder: DummyHashBuilder,
        table: RawTable {
            table: RawTableInner, // Properly initialize RawTableInner as needed
            alloc: SimpleAllocator,
            marker: PhantomData,
        },
    };

    hashmap.insert(1, "one".to_string());
    hashmap.insert(2, "two".to_string());

    let keys: [&i32; 2] = [&1, &2];
    let mut results: [Option<&'_ mut (i32, String)>; 2] = unsafe { hashmap.get_many_unchecked_mut_inner(keys) };

    assert!(results[0].is_some());
    assert!(results[1].is_some());

    if let Some((k, v)) = results[0].as_mut() {
        assert_eq!(*k, 1);
        *v = "uno".to_string();
    }

    if let Some((k, v)) = results[1].as_mut() {
        assert_eq!(*k, 2);
        *v = "dos".to_string();
    }

    assert_eq!(hashmap.get(&1), Some(&"uno".to_string()));
    assert_eq!(hashmap.get(&2), Some(&"dos".to_string()));
}

