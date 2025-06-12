// Answer 0

#[test]
fn test_insert_valid_entry() {
    use std::collections::hash_map::RandomState;

    struct MyAllocator;

    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {
            unimplemented!()
        }
    }

    let mut table = RawTable::<(&str, u32), MyAllocator> {
        table: RawTableInner::default(),
        alloc: MyAllocator,
        marker: PhantomData,
    };

    let hasher = RandomState::new();
    let mut vacant_entry = RawVacantEntryMut {
        table: &mut table,
        hash_builder: &hasher,
    };

    let (key, value) = vacant_entry.insert("c", 300);
    assert_eq!(*key, "c");
    assert_eq!(*value, 300);
}

#[test]
#[should_panic]
fn test_insert_panic_on_occupied_entry() {
    use std::collections::hash_map::RandomState;

    struct MyAllocator;

    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {
            unimplemented!()
        }
    }

    let mut table = RawTable::<(&str, u32), MyAllocator> {
        table: RawTableInner::default(),
        alloc: MyAllocator,
        marker: PhantomData,
    };

    let hasher = RandomState::new();
    let mut vacant_entry = RawVacantEntryMut {
        table: &mut table,
        hash_builder: &hasher,
    };

    // Assuming that inserting "c" at this point, causes an occupied entry
    vacant_entry.insert("c", 300);

    // Attempting to insert again should panic if we consider it an occupied entry scenario
    vacant_entry.insert("c", 400);
}

#[test]
fn test_insert_with_different_key() {
    use std::collections::hash_map::RandomState;

    struct MyAllocator;

    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {
            unimplemented!()
        }
    }

    let mut table = RawTable::<(&str, u32), MyAllocator> {
        table: RawTableInner::default(),
        alloc: MyAllocator,
        marker: PhantomData,
    };

    let hasher = RandomState::new();
    let mut vacant_entry = RawVacantEntryMut {
        table: &mut table,
        hash_builder: &hasher,
    };

    let (key1, value1) = vacant_entry.insert("c", 300);
    assert_eq!(*key1, "c");
    assert_eq!(*value1, 300);

    let (key2, value2) = vacant_entry.insert("d", 400);
    assert_eq!(*key2, "d");
    assert_eq!(*value2, 400);
}

