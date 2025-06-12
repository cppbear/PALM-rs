// Answer 0

#[test]
fn test_insert_with_hasher_basic() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table = RawTable::<(&str, u32), DummyAllocator> {
        table: RawTableInner::new(),
        alloc: DummyAllocator,
        marker: PhantomData,
    };

    let key = "a";
    let value = 100;
    let hash = 1;
    let hasher = |x: &str| x.len() as u64;

    let entry_mut = RawVacantEntryMut {
        table: &mut table,
        hash_builder: &hasher,
    };

    let (k, v) = entry_mut.insert_with_hasher(hash, key, value, hasher);
    assert_eq!(*k, "a");
    assert_eq!(*v, 100);
}

#[test]
#[should_panic]
fn test_insert_with_hasher_panic_on_occupied() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table = RawTable::<(&str, u32), DummyAllocator> {
        table: RawTableInner::new(),
        alloc: DummyAllocator,
        marker: PhantomData,
    };

    let key = "a";
    let value = 100;
    let hash = 1;
    let hasher = |x: &str| x.len() as u64;

    let entry_mut = RawVacantEntryMut {
        table: &mut table,
        hash_builder: &hasher,
    };

    // Simulate occupied entry by pre-inserting the entry directly
    table.insert_entry(hash, (key, value), |x| hasher(&x.0)); // Assuming insert_entry exists

    entry_mut.insert_with_hasher(hash, key, 200, hasher); // This should panic
}

