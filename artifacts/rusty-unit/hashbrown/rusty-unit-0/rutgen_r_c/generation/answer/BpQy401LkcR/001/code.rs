// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap::new();
    let key = "test_key";

    let entry_ref = EntryRef::<&str, str, u32, DefaultHashBuilder, TestAllocator>::Vacant(VacantEntryRef {
        hash: 0,
        key,
        table: &mut map,
    });

    let value = entry_ref.or_insert_with(|| 42);
    assert_eq!(*value, 42);
    assert_eq!(map["test_key"], 42);
}

#[test]
fn test_or_insert_with_existing_entry() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap::new();
    map.insert("existing_key", 10);

    let entry_ref = EntryRef::<&str, str, u32, DefaultHashBuilder, TestAllocator>::Occupied(OccupiedEntry {
        hash: 0,
        bucket: Bucket::new(("existing_key", 10)),
        table: &mut map,
    });

    let value = entry_ref.or_insert_with(|| 20);
    assert_eq!(*value, 10);
    assert_eq!(map["existing_key"], 10);
}

