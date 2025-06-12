// Answer 0

#[test]
fn test_raw_occupied_entry_mut_key() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    struct DummyHashBuilder;

    let mut raw_table = RawTable {
        table: RawTableInner::default(),
        alloc: TestAllocator,
        marker: PhantomData,
    };

    let elem = Bucket {
        ptr: NonNull::from(&(&"a", &100)),
    };

    let entry = RawOccupiedEntryMut {
        elem,
        table: &mut raw_table,
        hash_builder: &DummyHashBuilder,
    };

    let key = entry.key();
    assert_eq!(key, &"a");
}

