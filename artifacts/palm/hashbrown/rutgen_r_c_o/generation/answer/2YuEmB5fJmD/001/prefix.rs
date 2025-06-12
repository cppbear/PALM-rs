// Answer 0

#[test]
fn test_new_raw_table_inner() {
    let table = RawTableInner::new();
}

#[test]
fn test_new_raw_table_inner_empty() {
    let table = RawTableInner::new();
    let ctrl_ptr = table.ctrl.as_ptr();
    let bucket_mask = table.bucket_mask;
    let items = table.items;
    let growth_left = table.growth_left;
}

#[test]
fn test_new_raw_table_inner_properties() {
    let table = RawTableInner::new();
    assert_eq!(table.bucket_mask, 0);
    assert_eq!(table.items, 0);
    assert_eq!(table.growth_left, 0);
}

