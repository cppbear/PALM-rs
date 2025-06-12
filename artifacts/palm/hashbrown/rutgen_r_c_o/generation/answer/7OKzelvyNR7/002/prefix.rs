// Answer 0

#[test]
fn test_find_insert_slot_empty_bucket() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let capacity = 16; // Power of two
    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    // Assume we set all buckets to empty or deleted
    for i in 0..raw_table.buckets() {
        unsafe {
            raw_table.set_ctrl(i, Tag(EMPTY_TAG_VALUE));
        }
    }

    let hash = 42;
    let insert_slot = unsafe { raw_table.find_insert_slot(hash) };
    assert!(insert_slot.index < raw_table.buckets());
}

#[test]
fn test_find_insert_slot_full_buckets() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let capacity = 16; // Power of two
    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    
    // Fill all buckets to simulate a full table
    for i in 0..raw_table.buckets() {
        unsafe {
            raw_table.set_ctrl(i, Tag(FULL_TAG_VALUE)); // Assume FULL_TAG_VALUE indicates a full bucket
        }
    }

    let hash = 42;
    let insert_slot = unsafe { raw_table.find_insert_slot(hash) };
    assert!(insert_slot.index >= raw_table.buckets()); // Expect an invalid index due to full buckets
}

#[test]
fn test_find_insert_slot_mixed_state_buckets() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let capacity = 16; // Power of two
    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    
    // Set some buckets as empty and some as full
    for i in 0..8 {
        unsafe {
            raw_table.set_ctrl(i, Tag(FULL_TAG_VALUE)); // Full bucket
        }
    }
    for i in 8..raw_table.buckets() {
        unsafe {
            raw_table.set_ctrl(i, Tag(EMPTY_TAG_VALUE)); // Empty bucket
        }
    }

    let hash = 27;
    let insert_slot = unsafe { raw_table.find_insert_slot(hash) };
    assert!(insert_slot.index < raw_table.buckets()); // Expect to find an empty bucket
}

