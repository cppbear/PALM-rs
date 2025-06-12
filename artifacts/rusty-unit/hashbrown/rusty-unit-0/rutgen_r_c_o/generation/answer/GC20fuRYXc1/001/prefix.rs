// Answer 0

#[test]
fn test_len_empty_table() {
    let table: RawTable<u32> = RawTable::new_in(Global);
    let result = table.len();
}

#[test]
fn test_len_single_element() {
    let mut table: RawTable<u32> = RawTable::with_capacity_in(1, Global);
    unsafe {
        table.insert(1, 42, |x| *x);
    }
    let result = table.len();
}

#[test]
fn test_len_multiple_elements() {
    let mut table: RawTable<u32> = RawTable::with_capacity_in(5, Global);
    unsafe {
        table.insert(1, 10, |x| *x);
        table.insert(2, 20, |x| *x);
        table.insert(3, 30, |x| *x);
    }
    let result = table.len();
}

#[test]
fn test_len_capacity_exceeded() {
    let mut table: RawTable<u32> = RawTable::with_capacity_in(2, Global);
    unsafe {
        table.insert(1, 5, |x| *x);
        table.insert(2, 10, |x| *x);
        table.insert(3, 15, |x| *x);
    }
    let result = table.len();
}

#[test]
fn test_len_after_clear() {
    let mut table: RawTable<u32> = RawTable::new_in(Global);
    unsafe {
        table.insert(1, 100, |x| *x);
        table.clear();
    }
    let result = table.len();
}

