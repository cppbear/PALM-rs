// Answer 0

fn test_search_vacant_entry() {
    use crate::hash_map::DefaultHashBuilder;
    use crate::raw::{Global, RawTable};
    use crate::RawEntryBuilderMut;
    use crate::HashMap;

    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, i32, DefaultHashBuilder, DummyAllocator> = HashMap::new();
    
    let builder = RawEntryBuilderMut { map: &mut map };
    
    let hash: u64 = 12345; // Arbitrary hash value not present in the map

    let result = builder.search(hash, |k| *k == "nonexistent_key");

    match result {
        RawEntryMut::Vacant(_) => {}
        RawEntryMut::Occupied(_) => panic!("Expected a Vacant entry"),
    }
}

