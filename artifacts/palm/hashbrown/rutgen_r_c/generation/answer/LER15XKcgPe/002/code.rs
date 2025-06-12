// Answer 0

#[test]
fn test_prepare_rehash_in_place_full_buckets() {
    // Test for a case where all control bytes are full and we have enough buckets.
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        // Implement required allocator methods
    }

    let alloc = MockAllocator;
    let table_layout = TableLayout::default(); // Assuming default constructor exists
    let capacity = 16; // Using a power of two for buckets

    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    
    // Simulating all full control bytes
    for index in 0..raw_table.buckets() {
        unsafe {
            raw_table.set_ctrl(index, Tag(1)); // Assume Tag(1) represents a FULL byte
        }
    }

    unsafe {
        raw_table.prepare_rehash_in_place();
    }

    // Checking if all full control bytes are converted to DELETED
    for index in 0..raw_table.buckets() {
        unsafe {
            assert_eq!(raw_table.ctrl(index).read(), Tag(2)); // Assuming Tag(2) represents a DELETED byte
        }
    }
}

#[test]
fn test_prepare_rehash_in_place_empty_buckets() {
    // Test for a case where buckets are initialized as EMPTY and we have enough buckets.
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        // Implement required allocator methods
    }

    let alloc = MockAllocator;
    let table_layout = TableLayout::default(); // Assuming default constructor exists
    let capacity = 16; // Using a power of two for buckets

    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    
    // Simulating all empty control bytes
    for index in 0..raw_table.buckets() {
        unsafe {
            raw_table.set_ctrl(index, Tag(0)); // Assume Tag(0) represents an EMPTY byte
        }
    }

    unsafe {
        raw_table.prepare_rehash_in_place();
    }

    // Checking if all empty control bytes remain EMPTY
    for index in 0..raw_table.buckets() {
        unsafe {
            assert_eq!(raw_table.ctrl(index).read(), Tag(0)); // Confirming they stay as EMPTY
        }
    }
}

#[test]
fn test_prepare_rehash_in_place_with_deleted_buckets() {
    // Test for a case where some buckets are DELETED and some are FULL.
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        // Implement required allocator methods
    }

    let alloc = MockAllocator;
    let table_layout = TableLayout::default(); // Assuming default constructor exists
    let capacity = 16; // Using a power of two for buckets

    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    
    // Setting mixed control bytes: some FULL, some DELETED, some EMPTY
    unsafe {
        raw_table.set_ctrl(0, Tag(1)); // FULL
        raw_table.set_ctrl(1, Tag(2)); // DELETED
        raw_table.set_ctrl(2, Tag(0)); // EMPTY
        raw_table.set_ctrl(3, Tag(2)); // DELETED
    }

    unsafe {
        raw_table.prepare_rehash_in_place();
    }

    // Checking the expected control byte conversion
    unsafe {
        assert_eq!(raw_table.ctrl(0).read(), Tag(2)); // FULL to DELETED
        assert_eq!(raw_table.ctrl(1).read(), Tag(0)); // DELETED to EMPTY
        assert_eq!(raw_table.ctrl(2).read(), Tag(0)); // EMPTY remains EMPTY
        assert_eq!(raw_table.ctrl(3).read(), Tag(0)); // DELETED to EMPTY
    }
} 

#[test]
fn test_prepare_rehash_in_place_insufficient_buckets() {
    // Test for a case where there are insufficient buckets
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        // Implement required allocator methods
    }

    let alloc = MockAllocator;
    let table_layout = TableLayout::default(); // Assuming default constructor exists
    let capacity = 2; // Fewer buckets than Group::WIDTH for edge case

    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    
    // Simulating all control bytes as FULL
    for index in 0..raw_table.buckets() {
        unsafe {
            raw_table.set_ctrl(index, Tag(1)); // Assume FULL bytes
        }
    }

    // Prepare for potential panic, as we do not have enough buckets
    unsafe {
        raw_table.prepare_rehash_in_place(); // This should not panic since we handle this case internally
    }

    // Check that the conversion still behaves correctly under insufficient capacity
    for index in 0..raw_table.buckets() {
        unsafe {
            assert_eq!(raw_table.ctrl(index).read(), Tag(2)); // Expect FULL to DELETED conversion
        }
    }
}

