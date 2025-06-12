// Answer 0

#[test]
fn test_search_occupied_entry() {
    use crate::hash_map::DefaultHashBuilder;
    use crate::hash_map::RawEntryBuilderMut;

    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<&str, i32, DefaultHashBuilder, MockAllocator> = HashMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);
    
    let hash = make_hash::<&str, DefaultHashBuilder>("key1");
    
    let raw_entry_builder = RawEntryBuilderMut { map: &mut map };
    
    let result = raw_entry_builder.search(hash, |k| *k == "key1");

    if let RawEntryMut::Occupied(occupied) = result {
        assert_eq!(occupied.elem.ptr.as_ref(), &("key1", 1));
    } else {
        panic!("Expected an occupied entry but got vacant.");
    }
}

#[test]
fn test_search_vacant_entry() {
    use crate::hash_map::DefaultHashBuilder;
    use crate::hash_map::RawEntryBuilderMut;

    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<&str, i32, DefaultHashBuilder, MockAllocator> = HashMap::new();
    
    let hash = make_hash::<&str, DefaultHashBuilder>("missing_key");
    
    let raw_entry_builder = RawEntryBuilderMut { map: &mut map };

    let result = raw_entry_builder.search(hash, |k| *k == "missing_key");

    if let RawEntryMut::Vacant(_) = result {
        // Expected behavior
    } else {
        panic!("Expected a vacant entry but got occupied.");
    }
}

