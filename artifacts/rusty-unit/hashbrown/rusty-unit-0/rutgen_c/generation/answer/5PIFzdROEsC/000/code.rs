// Answer 0

#[test]
fn test_absent_entry_debug_fmt() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let table: &mut HashTable<i32, TestAllocator> = &mut HashTable {
        raw: RawTable::default(),
    };
    let absent_entry = AbsentEntry { table };

    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", absent_entry);
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "AbsentEntry");
}

