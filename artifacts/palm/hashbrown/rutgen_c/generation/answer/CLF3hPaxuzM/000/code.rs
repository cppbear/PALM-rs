// Answer 0

#[test]
fn test_search_occupied_entry() {
    struct DummyAllocator;
    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map = HashMap::<&str, i32, DefaultHashBuilder, DummyAllocator>::new();
    map.insert("key", 42);

    let hash = make_hash("key");
    let builder = RawEntryBuilderMut { map: &mut map };
    match builder.search(hash, |k| *k == "key") {
        RawEntryMut::Occupied(_) => {}
        _ => panic!("Expected an occupied entry"),
    }
}

#[test]
fn test_search_vacant_entry() {
    struct DummyAllocator;
    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map = HashMap::<&str, i32, DefaultHashBuilder, DummyAllocator>::new();

    let hash = make_hash("nonexistent");
    let builder = RawEntryBuilderMut { map: &mut map };
    match builder.search(hash, |k| *k == "nonexistent") {
        RawEntryMut::Vacant(_) => {}
        _ => panic!("Expected a vacant entry"),
    }
}

