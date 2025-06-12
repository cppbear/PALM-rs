// Answer 0

#[test]
fn test_find_or_find_insert_slot_inner_valid_case() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let capacity = 8; // More than one bucket
    let mut raw_table_inner = RawTableInner::with_capacity(&alloc, table_layout, capacity);

    let hash = 42; // within valid hash range
    raw_table_inner.ctrl(0).write_bytes(Tag::full(hash).0, raw_table_inner.num_ctrl_bytes());

    let mut eq = |index| {
        index == 0 // testing with a condition that succeeds
    };

    let result = unsafe { raw_table_inner.find_or_find_insert_slot_inner(hash, &mut eq) };
}

#[test]
fn test_find_or_find_insert_slot_inner_multiple_buckets() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let capacity = 16; // ensure multiple buckets
    let mut raw_table_inner = RawTableInner::with_capacity(&alloc, table_layout, capacity);

    let hash = 99; // valid hash value
    raw_table_inner.ctrl(0).write_bytes(Tag::full(hash).0, raw_table_inner.num_ctrl_bytes());

    let mut eq = |index| {
        index == 1 // again, testing to ensure it always returns true for index 1
    };

    let result = unsafe { raw_table_inner.find_or_find_insert_slot_inner(hash, &mut eq) };
}

#[test]
fn test_find_or_find_insert_slot_inner_edge_case() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let capacity = 4; // using a smaller bucket
    let mut raw_table_inner = RawTableInner::with_capacity(&alloc, table_layout, capacity);

    let hash = (1u64 << 63) - 1; // maximum hash value

    raw_table_inner.ctrl(0).write_bytes(Tag::full(hash).0, raw_table_inner.num_ctrl_bytes());

    let mut eq = |index| {
        index <= 3 // testing edge of valid bucket index range
    };

    let result = unsafe { raw_table_inner.find_or_find_insert_slot_inner(hash, &mut eq) };
}

#[test]
fn test_find_or_find_insert_slot_inner_fully_filled() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let capacity = 8; // ensure multiple buckets
    let mut raw_table_inner = RawTableInner::with_capacity(&alloc, table_layout, capacity);

    let hash = 101; // valid hash value
    for i in 0..raw_table_inner.buckets() {
        raw_table_inner.ctrl(i).write_bytes(Tag::full(hash).0, raw_table_inner.num_ctrl_bytes());
    }

    let mut eq = |index| {
        false // no index should succeed as all are FULL
    };

    let result = unsafe { raw_table_inner.find_or_find_insert_slot_inner(hash, &mut eq) };
}

