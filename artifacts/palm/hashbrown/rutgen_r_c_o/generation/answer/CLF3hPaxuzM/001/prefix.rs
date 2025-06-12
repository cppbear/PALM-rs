// Answer 0

#[test]
fn test_search_occupied_entry_found() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<u64, &str, DefaultHashBuilder, TestAllocator> = HashMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    
    let raw_entry_builder = RawEntryBuilderMut { map: &mut map };
    
    let hash = 1; // Hash corresponding to key 1
    let is_match = |key: &u64| *key == 1;

    let result = raw_entry_builder.search(hash, is_match);
}

#[test]
fn test_search_occupied_entry_different_hash() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<u64, &str, DefaultHashBuilder, TestAllocator> = HashMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    
    let raw_entry_builder = RawEntryBuilderMut { map: &mut map };
    
    let hash = 1; // Hash corresponding to key 1
    let is_match = |key: &u64| *key == 2; // Looking for a different key

    let result = raw_entry_builder.search(hash, is_match);
}

