// Answer 0

#[test]
fn test_vacant_entry_ref_debug_fmt() {
    use core::fmt::Formatter;
    use core::marker::PhantomData;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let key = "test_key";
    let hash_builder = DefaultHashBuilder;
    let table = RawTable::<(&str, i32), TestAllocator>::new();

    let mut hash_map = HashMap {
        hash_builder,
        table,
    };

    let vacant_entry_ref = VacantEntryRef {
        hash: 0,
        key: &key,
        table: &mut hash_map,
    };

    let mut formatter = Formatter::new();

    assert!(vacant_entry_ref.fmt(&mut formatter).is_ok());
}

#[test]
fn test_vacant_entry_ref_debug_fmt_empty_key() {
    use core::fmt::Formatter;
    use core::marker::PhantomData;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let key: &str = "";
    let hash_builder = DefaultHashBuilder;
    let table = RawTable::<(&str, i32), TestAllocator>::new();

    let mut hash_map = HashMap {
        hash_builder,
        table,
    };

    let vacant_entry_ref = VacantEntryRef {
        hash: 0,
        key: &key,
        table: &mut hash_map,
    };

    let mut formatter = Formatter::new();

    assert!(vacant_entry_ref.fmt(&mut formatter).is_ok());
}

