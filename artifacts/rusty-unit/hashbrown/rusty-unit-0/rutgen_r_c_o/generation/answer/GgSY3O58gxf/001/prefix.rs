// Answer 0

#[test]
fn test_raw_table_is_empty_with_zero_items() {
    let table: RawTable<i32> = RawTable::new_in(Global);
    let result = table.is_empty();
}

#[test]
fn test_raw_table_is_empty_after_clearing() {
    let mut table: RawTable<i32> = RawTable::with_capacity_in(10, Global);
    unsafe {
        table.clear();
    }
    let result = table.is_empty();
}

#[test]
fn test_raw_table_is_empty_after_insertion_and_removal() {
    let mut table: RawTable<i32> = RawTable::new_in(Global);
    unsafe {
        table.insert(1, 42, |v| *v);
        table.remove(1);
    }
    let result = table.is_empty();
}

