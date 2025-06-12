// Answer 0

#[test]
fn test_or_insert_with_key_vacant_entry() {
    struct MyHasher;

    impl BuildHasher for MyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    struct MyAllocator;

    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _: core::alloc::Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: core::alloc::Layout) {}
    }

    let mut map: HashMap<String, usize, MyHasher, MyAllocator> = HashMap::new();
    
    match map.entry_ref("new_key") {
        EntryRef::Vacant(vacant) => {
            let value = vacant.or_insert_with_key(|key| key.len());
            assert_eq!(*value, 8);
        }
        EntryRef::Occupied(_) => panic!("Expected Vacant entry"),
    }
}

#[test]
fn test_or_insert_with_key_occupied_entry() {
    struct MyHasher;

    impl BuildHasher for MyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    struct MyAllocator;

    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _: core::alloc::Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: core::alloc::Layout) {}
    }

    let mut map: HashMap<String, usize, MyHasher, MyAllocator> = HashMap::new();
    map.insert("existing_key".to_owned(), 5);
    
    match map.entry_ref("existing_key") {
        EntryRef::Occupied(occupied) => {
            *occupied.or_insert_with_key(|key| key.len()) *= 2;
            assert_eq!(map["existing_key"], 10);
        }
        EntryRef::Vacant(_) => panic!("Expected Occupied entry"),
    }
}

