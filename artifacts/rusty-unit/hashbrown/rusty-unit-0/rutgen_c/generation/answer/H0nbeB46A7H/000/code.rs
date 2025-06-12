// Answer 0

#[test]
fn test_insert() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table = RawTable {
        table: RawTableInner::default(),
        alloc: TestAllocator,
        marker: PhantomData,
    };

    let key = "a";
    let value = 100;
    let occupied_entry = RawOccupiedEntryMut {
        elem: Bucket { ptr: NonNull::new_unchecked(Box::into_raw(Box::new((key, value)))) },
        table: &mut table,
        hash_builder: &(),
    };
    
    let old_value = occupied_entry.insert(1000);
    assert_eq!(old_value, value);
}

#[test]
fn test_insert_multiple() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table = RawTable {
        table: RawTableInner::default(),
        alloc: TestAllocator,
        marker: PhantomData,
    };

    let key = "b";
    let value = 200;
    let occupied_entry = RawOccupiedEntryMut {
        elem: Bucket { ptr: NonNull::new_unchecked(Box::into_raw(Box::new((key, value)))) },
        table: &mut table,
        hash_builder: &(),
    };
    
    let old_value = occupied_entry.insert(300);
    assert_eq!(old_value, value);
}

#[test]
fn test_insert_boundary() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table = RawTable {
        table: RawTableInner::default(),
        alloc: TestAllocator,
        marker: PhantomData,
    };

    let key = "c";
    let value = i32::MAX;
    let occupied_entry = RawOccupiedEntryMut {
        elem: Bucket { ptr: NonNull::new_unchecked(Box::into_raw(Box::new((key, value)))) },
        table: &mut table,
        hash_builder: &(),
    };
    
    let old_value = occupied_entry.insert(i32::MIN);
    assert_eq!(old_value, value);
}

