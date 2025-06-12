// Answer 0

#[test]
fn test_clear_empty_table() {
    let alloc = Global;
    let mut table: RawTable<u32, _> = RawTable::new_in(alloc);
    table.clear();
}

#[test]
fn test_clear_already_empty_table() {
    let alloc = Global;
    let table: RawTable<u32, _> = RawTable::new_in(alloc);
    table.clear();
}

#[test]
fn test_clear_with_capacity() {
    let alloc = Global;
    let mut table: RawTable<u32, _> = RawTable::with_capacity_in(0, alloc);
    table.clear();
}

