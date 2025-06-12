// Answer 0

#[test]
fn test_get_occupied_entry() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<&str, ()> = HashMap::new();
    map.insert("poneyland", ());
    
    let entry = map.entry("poneyland").or_insert();
    match entry {
        Entry::Occupied(entry) => {
            assert_eq!(entry.get(), &"poneyland");
        },
        Entry::Vacant(_) => panic!(),
    }
}

#[test]
fn test_get_vacant_entry() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<&str, ()> = HashMap::new();
    
    match map.entry("nonexistent") {
        Entry::Vacant(_) => {},
        Entry::Occupied(_) => panic!(),
    }
}

