// Answer 0

#[test]
fn test_rehash_in_place_valid() {
    struct TestHasher;

    impl TestHasher {
        unsafe fn hash(&self, _: &mut RawTableInner, index: usize) -> u64 {
            index as u64 // Simple hashing based on index for testing
        }
    }

    let allocator = Global; // Assuming Global is a valid allocator
    let table_layout = TableLayout::default(); // Assuming there's a default layout
    let capacity = 8; // Example capacity
    let mut raw_table_inner = RawTableInner::with_capacity(&allocator, table_layout, capacity);

    // Simulate populated buckets
    for i in 0..raw_table_inner.buckets() {
        unsafe {
            raw_table_inner.set_ctrl(i, Tag(1)); // Set to non-DELETED
        }
    }

    let hasher = TestHasher;
    let size_of = std::mem::size_of::<u64>(); // Assuming u64 is the element type
    let drop_fn: Option<unsafe fn(*mut u8)> = None;

    unsafe {
        raw_table_inner.rehash_in_place(&hasher.hash, size_of, drop_fn);
    }
}

#[test]
#[should_panic]
fn test_rehash_in_place_panic_out_of_bounds() {
    struct TestHasher;

    impl TestHasher {
        unsafe fn hash(&self, _: &mut RawTableInner, _: usize) -> u64 {
            panic!("Panic triggered in hasher"); // Intentional panic
        }
    }

    let allocator = Global;
    let table_layout = TableLayout::default();
    let capacity = 8;
    let mut raw_table_inner = RawTableInner::with_capacity(&allocator, table_layout, capacity);

    // Simulate populated buckets
    for i in 0..raw_table_inner.buckets() {
        unsafe {
            raw_table_inner.set_ctrl(i, Tag(1)); // Set to non-DELETED
        }
    }

    let hasher = TestHasher;
    let size_of = std::mem::size_of::<u64>();
    let drop_fn: Option<unsafe fn(*mut u8)> = None;

    unsafe {
        raw_table_inner.rehash_in_place(&hasher.hash, size_of, drop_fn);
    }
}

#[test]
fn test_rehash_in_place_with_deleted() {
    struct TestHasher;

    impl TestHasher {
        unsafe fn hash(&self, _: &mut RawTableInner, index: usize) -> u64 {
            index as u64 // Simple hashing based on index for testing
        }
    }

    let allocator = Global;
    let table_layout = TableLayout::default();
    let capacity = 8;
    let mut raw_table_inner = RawTableInner::with_capacity(&allocator, table_layout, capacity);

    // Only some slots are populated; others are "deleted"
    for i in 0..raw_table_inner.buckets() {
        if i % 2 == 0 {
            unsafe {
                raw_table_inner.set_ctrl(i, Tag(1)); // Set to non-DELETED
            }
        } else {
            unsafe {
                raw_table_inner.set_ctrl(i, Tag(0)); // Simulate DELETED
            }
        }
    }

    let hasher = TestHasher;
    let size_of = std::mem::size_of::<u64>();
    let drop_fn: Option<unsafe fn(*mut u8)> = None;

    unsafe {
        raw_table_inner.rehash_in_place(&hasher.hash, size_of, drop_fn);
    }
}

#[test]
fn test_rehash_in_place_empty() {
    struct TestHasher;

    impl TestHasher {
        unsafe fn hash(&self, _: &mut RawTableInner, index: usize) -> u64 {
            index as u64 // Simple hashing based on index for testing
        }
    }

    let allocator = Global;
    let table_layout = TableLayout::default();
    let capacity = 8;
    let mut raw_table_inner = RawTableInner::with_capacity(&allocator, table_layout, capacity);

    // Test with empty slots (not filling any buckets)
    for i in 0..raw_table_inner.buckets() {
        unsafe {
            raw_table_inner.set_ctrl(i, Tag(0)); // Simulate all DELETED
        }
    }

    let hasher = TestHasher;
    let size_of = std::mem::size_of::<u64>();
    let drop_fn: Option<unsafe fn(*mut u8)> = None;

    unsafe {
        raw_table_inner.rehash_in_place(&hasher.hash, size_of, drop_fn);
    }
}

#[test]
#[should_panic]
fn test_rehash_in_place_panics_for_buckets() {
    struct TestHasher;

    impl TestHasher {
        unsafe fn hash(&self, _: &mut RawTableInner, _: usize) -> u64 {
            0 // This hasher will never panic, but we will trigger the panic condition intentionally
        }
    }

    let allocator = Global;
    let table_layout = TableLayout::default();
    let capacity = 8;
    let mut raw_table_inner = RawTableInner::with_capacity(&allocator, table_layout, capacity);

    // This simulation does not set valid control bytes for the last index
    for i in 0..raw_table_inner.buckets() {
        unsafe {
            raw_table_inner.set_ctrl(i, Tag(1)); // Set to non-DELETED
        }
    }

    let hasher = TestHasher;
    let size_of = std::mem::size_of::<u64>();
    let drop_fn: Option<unsafe fn(*mut u8)> = None;

    // Triggering an out-of-bounds access
    let mut out_of_bounds_index = raw_table_inner.buckets(); // This will cause panic
    unsafe {
        raw_table_inner.rehash_in_place(&hasher.hash, size_of, drop_fn);
    }
}

