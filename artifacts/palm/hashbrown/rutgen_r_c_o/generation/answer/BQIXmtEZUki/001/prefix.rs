// Answer 0

#[test]
fn test_prepare_insert_slot_with_empty_bucket() {
    let mut table_inner = RawTableInner::with_capacity(&Global, TableLayout::default(), 8);
    let hash: u64 = 1;
    let (index, old_ctrl) = unsafe { table_inner.prepare_insert_slot(hash) };
}

#[test]
fn test_prepare_insert_slot_with_deleted_bucket() {
    let mut table_inner = RawTableInner::with_capacity(&Global, TableLayout::default(), 8);
    let hash: u64 = 2;

    unsafe {
        table_inner.set_ctrl(0, Tag(1)); // simulate a deleted bucket
    }

    let (index, old_ctrl) = unsafe { table_inner.prepare_insert_slot(hash) };
}

#[test]
#[should_panic]
fn test_prepare_insert_slot_infinite_loop() {
    let mut table_inner = RawTableInner::with_capacity(&Global, TableLayout::default(), 4);
    let hash: u64 = 3;

    unsafe {
        // Simulating all buckets full by using control bytes
        for i in 0..4 {
            table_inner.set_ctrl(i, Tag(1)); // not empty or deleted
        }
    }

    unsafe { table_inner.prepare_insert_slot(hash) };
}

#[test]
fn test_prepare_insert_slot_with_maximum_hash() {
    let mut table_inner = RawTableInner::with_capacity(&Global, TableLayout::default(), 16);
    let hash: u64 = u64::MAX;

    let (index, old_ctrl) = unsafe { table_inner.prepare_insert_slot(hash) };
}

#[test]
fn test_prepare_insert_slot_filled_and_empty_slots() {
    let mut table_inner = RawTableInner::with_capacity(&Global, TableLayout::default(), 8);
    let hash1: u64 = 5;
    let hash2: u64 = 6;

    unsafe {
        table_inner.set_ctrl(0, Tag(1)); // not empty
        table_inner.set_ctrl(1, Tag::EMPTY); // empty
        table_inner.set_ctrl(2, Tag(1)); // not empty
        table_inner.set_ctrl(3, Tag::DELETED); // deleted
    }

    let (index1, old_ctrl1) = unsafe { table_inner.prepare_insert_slot(hash1) };
    let (index2, old_ctrl2) = unsafe { table_inner.prepare_insert_slot(hash2) };
}

