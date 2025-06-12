// Answer 0

#[test]
fn test_raw_entry_builder_debug() {
    use core::fmt::Formatter;
    use core::ptr::NonNull;
    use std::alloc::Layout;

    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut hash_map: HashMap<i32, i32, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder,
        table: RawTable::new(),
    };

    let entry_builder = RawEntryBuilderMut {
        map: &mut hash_map,
    };

    let mut buffer = core::fmt::Formatter::new();
    let result = entry_builder.fmt(&mut buffer);
    assert!(result.is_ok());
}

