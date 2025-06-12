// Answer 0

#[test]
fn test_replace_entry_with_vacant() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::ptr::NonNull;

    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _: std::alloc::Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        
        unsafe fn deallocate(&self, _: NonNull<u8>, _: std::alloc::Layout) {}
    }

    let mut map: HashMap<&str, u32, DummyAllocator> = HashMap::new();
    map.insert("a", 100);

    // Attempt to replace an entry that will not still be occupied
    let raw_entry = match map.raw_entry_mut().from_key(&"b") {
        RawEntryMut::Occupied(_) => panic!(),
        RawEntryMut::Vacant(o) => o.replace_entry_with(|_k, _v| None),
    };

    match raw_entry {
        RawEntryMut::Vacant(_) => { /* expected */ },
        RawEntryMut::Occupied(_) => panic!(),
    }
}

#[test]
fn test_replace_entry_with_no_change() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::ptr::NonNull;

    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _: std::alloc::Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        
        unsafe fn deallocate(&self, _: NonNull<u8>, _: std::alloc::Layout) {}
    }

    let mut map: HashMap<&str, u32, DummyAllocator> = HashMap::new();
    map.insert("a", 100);

    // Attempt to change an entry but return None to indicate it should be vacant
    let raw_entry = match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Occupied(o) => o.replace_entry_with(|k, v| {
            assert_eq!(k, &"a");
            assert_eq!(v, 100);
            None
        }),
        RawEntryMut::Vacant(_) => panic!(),
    };

    match raw_entry {
        RawEntryMut::Vacant(_) => { /* expected */ },
        RawEntryMut::Occupied(_) => panic!(),
    }
    assert_eq!(map.get(&"a"), None);
}

