// Answer 0

#[test]
fn test_occupied_entry_fmt_valid() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new(1 as *mut u8).unwrap())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let allocator = TestAllocator;
    let bucket = Bucket { ptr: NonNull::new(1 as *mut i32).unwrap() };
    let mut table = HashTable { raw: RawTable::new() };
    let entry = OccupiedEntry { hash: 1, bucket, table: &mut table };

    let mut formatter = fmt::Formatter::new();
    entry.fmt(&mut formatter);
}

#[test]
fn test_occupied_entry_fmt_edge_case_hash() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new(1 as *mut u8).unwrap())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let allocator = TestAllocator;
    let bucket = Bucket { ptr: NonNull::new(2 as *mut i32).unwrap() };
    let mut table = HashTable { raw: RawTable::new() };
    let entry = OccupiedEntry { hash: 2u64.pow(64) - 1, bucket, table: &mut table };

    let mut formatter = fmt::Formatter::new();
    entry.fmt(&mut formatter);
}

#[test]
fn test_occupied_entry_fmt_non_null_ptr() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new(1 as *mut u8).unwrap())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let allocator = TestAllocator;
    let bucket = Bucket { ptr: NonNull::new(3 as *mut i32).unwrap() };
    let mut table = HashTable { raw: RawTable::new() };
    let entry = OccupiedEntry { hash: 12345, bucket, table: &mut table };

    let mut formatter = fmt::Formatter::new();
    entry.fmt(&mut formatter);
}

#[test]
#[should_panic]
fn test_occupied_entry_fmt_null_table() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new(1 as *mut u8).unwrap())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let allocator = TestAllocator;
    let bucket = Bucket { ptr: NonNull::new(1 as *mut i32).unwrap() };
    let entry = OccupiedEntry { hash: 12345, bucket, table: std::ptr::null_mut() };

    let mut formatter = fmt::Formatter::new();
    entry.fmt(&mut formatter);
}

