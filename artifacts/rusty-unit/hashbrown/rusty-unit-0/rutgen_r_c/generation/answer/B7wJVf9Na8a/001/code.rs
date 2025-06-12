// Answer 0

#[test]
fn test_insert_entry() {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasherDefault, Hash};

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            // Mock allocation, always succeeding
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // No-op for deallocation
        }
    }

    let mut hash_map: HashMap<i32, String, BuildHasherDefault<RandomState>, TestAllocator> = HashMap::with_hasher(BuildHasherDefault::default());
    let hash_builder = &BuildHasherDefault::<RandomState>::default();

    let raw_table = RawTable {
        table: RawTableInner::default(), // Should be populated appropriately
        alloc: TestAllocator,
        marker: PhantomData::<(i32, String)>,
    };

    let raw_vacant_entry = RawVacantEntryMut {
        table: &mut raw_table,
        hash_builder,
    };

    let occupied_entry = raw_vacant_entry.insert_entry(10, "test".to_string());

    assert_eq!(occupied_entry.elem.key, 10);
    assert_eq!(occupied_entry.elem.value, "test");
}

#[test]
#[should_panic]
fn test_insert_entry_should_panic_on_duplicate_key() {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasherDefault, Hash};

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // No-op for deallocation
        }
    }

    let mut hash_map: HashMap<i32, String, BuildHasherDefault<RandomState>, TestAllocator> = HashMap::with_hasher(BuildHasherDefault::default());
    let hash_builder = &BuildHasherDefault::<RandomState>::default();

    let raw_table = RawTable {
        table: RawTableInner::default(), // Should be populated appropriately
        alloc: TestAllocator,
        marker: PhantomData::<(i32, String)>,
    };

    let raw_vacant_entry = RawVacantEntryMut {
        table: &mut raw_table,
        hash_builder,
    };

    raw_vacant_entry.insert_entry(10, "test1".to_string());
    // Inserting another entry with the same key should panic
    raw_vacant_entry.insert_entry(10, "test2".to_string());
}

