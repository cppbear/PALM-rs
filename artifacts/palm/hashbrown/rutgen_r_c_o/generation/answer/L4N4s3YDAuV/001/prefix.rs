// Answer 0

#[test]
fn test_get_valid_hash() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut table: RawTable<i32, TestAllocator> = RawTable::new_in(TestAllocator);
    let hash = 12345;
    let value = 42;
    table.insert(hash, value, |v| *v as u64); // insert value to ensure presence

    let eq_function = |&v: &i32| v == value;
    let result = table.get(hash, eq_function);
}

#[test]
fn test_get_another_valid_hash() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut table: RawTable<i32, TestAllocator> = RawTable::new_in(TestAllocator);
    let hash = 54321;
    let value = 100;
    table.insert(hash, value, |v| *v as u64); // insert value to ensure presence

    let eq_function = |&v: &i32| v == value;
    let result = table.get(hash, eq_function);
}

#[test]
fn test_get_hash_at_upper_limit() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut table: RawTable<i32, TestAllocator> = RawTable::new_in(TestAllocator);
    let hash = u64::MAX; // test upper limit
    let value = 200;
    table.insert(hash, value, |v| *v as u64); // insert value to ensure presence

    let eq_function = |&v: &i32| v == value;
    let result = table.get(hash, eq_function);
}

