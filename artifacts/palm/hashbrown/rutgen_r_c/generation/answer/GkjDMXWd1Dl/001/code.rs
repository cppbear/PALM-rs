// Answer 0

#[test]
fn test_from_hash_found() {
    struct SimpleAllocator;

    unsafe impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<&str, u32, std::collections::hash_map::DefaultHasher, SimpleAllocator> = HashMap::new();
    map.insert("a", 100);
    map.insert("b", 200);
    
    let key = "a";
    let hash = compute_hash(map.hasher(), &key);
    
    let entry_builder = RawEntryBuilder { map: &map };
    assert_eq!(entry_builder.from_hash(hash, |k| k == &key), Some((&"a", &100)));
}

#[test]
fn test_from_hash_not_found() {
    struct SimpleAllocator;

    unsafe impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<&str, u32, std::collections::hash_map::DefaultHasher, SimpleAllocator> = HashMap::new();
    map.insert("a", 100);
    map.insert("b", 200);

    let key = "c"; // Non-existent key
    let hash = compute_hash(map.hasher(), &key);
    
    let entry_builder = RawEntryBuilder { map: &map };
    assert_eq!(entry_builder.from_hash(hash, |k| k == &key), None);
}

#[test]
fn test_from_hash_empty_map() {
    struct SimpleAllocator;

    unsafe impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let map: HashMap<&str, u32, std::collections::hash_map::DefaultHasher, SimpleAllocator> = HashMap::new();
    
    let key = "a"; // Non-existent key
    let hash = compute_hash(map.hasher(), &key);
    
    let entry_builder = RawEntryBuilder { map: &map };
    assert_eq!(entry_builder.from_hash(hash, |k| k == &key), None);
}

