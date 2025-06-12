// Answer 0

#[test]
fn test_remove_entry() {
    struct MockAllocator;

    impl Allocator for MockAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<(&str, u32), MockAllocator> = RawTable {
        table: RawTableInner::default(),
        alloc: MockAllocator,
        marker: PhantomData,
    };

    let occupied_entry = RawOccupiedEntryMut {
        elem: Bucket {
            ptr: NonNull::new_unchecked(&mut ("a", 100)),
        },
        table: &mut table,
        hash_builder: &(),
    };

    let value = occupied_entry.remove();
    assert_eq!(value, 100);
}

