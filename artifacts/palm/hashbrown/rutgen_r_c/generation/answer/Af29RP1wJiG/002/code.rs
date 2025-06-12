// Answer 0

#[test]
fn test_search_none() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let empty_map: HashMap<i32, i32, std::collections::hash_map::RandomState, DummyAllocator> =
        HashMap {
            hash_builder: std::collections::hash_map::RandomState::new(),
            table: RawTable {
                table: RawTableInner::default(),
                alloc: DummyAllocator,
                marker: PhantomData,
            },
        };

    let builder = RawEntryBuilder { map: &empty_map };
    let result = builder.search(0, |_| true);
    assert_eq!(result, None);
}

#[test]
fn test_search_none_with_non_matching_predicate() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let map_with_one_entry: HashMap<i32, i32, std::collections::hash_map::RandomState, DummyAllocator> =
        HashMap {
            hash_builder: std::collections::hash_map::RandomState::new(),
            table: RawTable {
                table: RawTableInner::new_with_entry((1, 10)), // Pretend this initializes the table with one entry (1, 10)
                alloc: DummyAllocator,
                marker: PhantomData,
            },
        };

    let builder = RawEntryBuilder { map: &map_with_one_entry };
    let result = builder.search(1, |&key| key != 1);
    assert_eq!(result, None);
}

