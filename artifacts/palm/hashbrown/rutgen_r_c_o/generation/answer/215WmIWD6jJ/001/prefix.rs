// Answer 0

#[test]
fn test_raw_table_new() {
    let table: RawTable<u8, Global> = RawTable::new();
}

#[test]
fn test_raw_table_with_capacity_1() {
    let table: RawTable<u8, Global> = RawTable::with_capacity(1);
}

