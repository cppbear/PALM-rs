// Answer 0

#[test]
fn test_key_vacant_entry() {
    use hashbrown::hash_map::{Entry, HashMap};

    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, DummyAllocator> = HashMap::new();
    let entry = map.entry("horseland");

    match entry {
        Entry::Vacant(ref vacant_entry) => {
            assert_eq!(vacant_entry.key(), &"horseland");
        },
        _ => panic!("Expected a Vacant entry, but got an Occupied entry."),
    }
}

#[test]
fn test_key_occupied_entry() {
    use hashbrown::hash_map::{Entry, HashMap};

    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, DummyAllocator> = HashMap::new();
    map.insert("poneyland", 3);
    let entry = map.entry("poneyland");

    match entry {
        Entry::Occupied(ref occupied_entry) => {
            assert_eq!(occupied_entry.key(), &"poneyland");
        },
        _ => panic!("Expected an Occupied entry, but got a Vacant entry."),
    }
}

