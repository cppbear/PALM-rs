// Answer 0

#[test]
fn test_or_default_with_occupied_entry() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<&str, Option<u32, TestAllocator>> = HashMap::new();
    map.insert("key1", Some(42));

    let entry = Entry::Occupied(OccupiedEntry {
        hash: 0,
        elem: Bucket::new((std::ptr::NonNull::new(&mut map["key1"]).unwrap(),)),
        table: &mut map,
    });

    assert_eq!(entry.or_default(), &mut Some(42));
}

#[test]
fn test_or_default_with_vacant_entry() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<&str, Option<u32, TestAllocator>> = HashMap::new();
    let hash = 0; // Simulate a hash for the vacant entry

    let entry = Entry::Vacant(VacantEntry {
        hash,
        key: "new_key",
        table: &mut map,
    });

    assert_eq!(entry.or_default(), &mut None);
}

