// Answer 0

#[test]
fn test_record_item_insert_at_index_zero() {
    let mut table_inner = RawTableInner::with_capacity(&Global, TableLayout::new(), 4);
    let old_ctrl = Tag(0);
    let hash = 0;
    unsafe {
        table_inner.record_item_insert_at(0, old_ctrl, hash);
    }
}

#[test]
fn test_record_item_insert_at_max_index() {
    let mut table_inner = RawTableInner::with_capacity(&Global, TableLayout::new(), 4);
    let old_ctrl = Tag(255);
    let hash = u64::MAX;
    let max_index = usize::MAX;
    unsafe {
        table_inner.record_item_insert_at(max_index, old_ctrl, hash);
    }
}

#[test]
fn test_record_item_insert_at_middle_index() {
    let mut table_inner = RawTableInner::with_capacity(&Global, TableLayout::new(), 8);
    let old_ctrl = Tag(128);
    let hash = 123456;
    let middle_index = 4;
    unsafe {
        table_inner.record_item_insert_at(middle_index, old_ctrl, hash);
    }
}

#[test]
fn test_record_item_insert_at_special_empty_old_ctrl() {
    let mut table_inner = RawTableInner::with_capacity(&Global, TableLayout::new(), 4);
    let old_ctrl = Tag::EMPTY;
    let hash = 42;
    let index = 1;
    unsafe {
        table_inner.record_item_insert_at(index, old_ctrl, hash);
    }
}

#[test]
fn test_record_item_insert_at_special_deleted_old_ctrl() {
    let mut table_inner = RawTableInner::with_capacity(&Global, TableLayout::new(), 4);
    let old_ctrl = Tag::DELETED;
    let hash = 99;
    let index = 2;
    unsafe {
        table_inner.record_item_insert_at(index, old_ctrl, hash);
    }
}

