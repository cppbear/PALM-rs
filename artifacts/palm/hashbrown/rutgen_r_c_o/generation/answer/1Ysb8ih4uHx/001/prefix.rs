// Answer 0

#[test]
fn test_drain_empty_table() {
    let table: RawTable<i32> = RawTable::with_capacity_in(0, Global);
    let mut drain = table.drain();
}

#[test]
fn test_drain_single_element() {
    let mut table: RawTable<i32> = RawTable::with_capacity_in(1, Global);
    unsafe {
        table.insert(0, 42, |x| *x);
    }
    let mut drain = table.drain();
}

#[test]
fn test_drain_multiple_elements() {
    let mut table: RawTable<i32> = RawTable::with_capacity_in(4, Global);
    unsafe {
        table.insert(0, 10, |x| *x);
        table.insert(1, 20, |x| *x);
        table.insert(2, 30, |x| *x);
    }
    let mut drain = table.drain();
}

#[test]
fn test_drain_large_table() {
    let mut table: RawTable<i32> = RawTable::with_capacity_in(1 << 20, Global);
    for i in 0..(1 << 20) {
        unsafe {
            table.insert(i as u64, i as i32, |x| *x);
        }
    }
    let mut drain = table.drain();
}

#[test]
fn test_drain_filled_table() {
    let mut table: RawTable<i32> = RawTable::with_capacity_in(8, Global);
    for i in 0..8 {
        unsafe {
            table.insert(i as u64, i, |x| *x);
        }
    }
    let mut drain = table.drain();
}

#[test]
fn test_drain_sequential_inserts_and_removes() {
    let mut table: RawTable<i32> = RawTable::with_capacity_in(4, Global);
    for i in 0..4 {
        unsafe {
            table.insert(i as u64, i * 10, |x| *x);
        }
    }
    let mut drain = table.drain();
}

