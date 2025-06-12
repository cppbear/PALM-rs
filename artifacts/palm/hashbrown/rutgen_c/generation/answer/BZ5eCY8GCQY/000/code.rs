// Answer 0

#[test]
fn test_vacant_entry_debug_fmt() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::new_unchecked(core::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _: core::ptr::NonNull<u8>, _: core::alloc::Layout) {}
    }

    let mut table = HashTable::<u32, DummyAllocator> { raw: RawTable::new() };
    let insert_slot = InsertSlot { index: 0 };
    let vacant_entry = VacantEntry {
        hash: 0,
        insert_slot,
        table: &mut table,
    };

    let result = format!("{:?}", vacant_entry);
    assert_eq!(result, "VacantEntry");
}

