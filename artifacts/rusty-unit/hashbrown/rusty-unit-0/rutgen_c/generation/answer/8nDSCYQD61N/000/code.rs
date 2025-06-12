// Answer 0

#[test]
fn test_and_modify_with_occupied_entry() {
    struct SimpleAllocator;

    impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<String, u32, DefaultHashBuilder, SimpleAllocator> = HashMap::new();
    map.insert(String::from("test"), 10);

    match map.entry_ref("test") {
        EntryRef::Occupied(entry) => {
            entry.and_modify(|value| *value += 5);
            assert_eq!(entry.get(), &15);
        },
        EntryRef::Vacant(_) => panic!("Expected occupied entry"),
    }
}

#[test]
fn test_and_modify_with_vacant_entry() {
    struct SimpleAllocator;

    impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<String, u32, DefaultHashBuilder, SimpleAllocator> = HashMap::new();

    match map.entry_ref("non_existent") {
        EntryRef::Occupied(_) => panic!("Expected vacant entry"),
        EntryRef::Vacant(entry) => {
            entry.or_insert(20);
            assert_eq!(map["non_existent"], 20);
        },
    }
}

