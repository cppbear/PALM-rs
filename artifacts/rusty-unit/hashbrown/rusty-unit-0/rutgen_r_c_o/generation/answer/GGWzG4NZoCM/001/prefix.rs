// Answer 0

#[test]
fn test_clear_no_drop_empty_table() {
    let table: RawTable<u32> = RawTable::new_in(Global);
    table.clear_no_drop();
}

#[test]
fn test_clear_no_drop_single_item() {
    let mut table: RawTable<u32> = RawTable::with_capacity_in(1, Global);
    unsafe {
        table.insert(1, 42, |x| *x);
    }
    table.clear_no_drop();
}

#[test]
fn test_clear_no_drop_full_buckets() {
    let mut table: RawTable<u32> = RawTable::with_capacity_in(4, Global);
    for i in 0..4 {
        unsafe {
            table.insert(i as u64, i * 10, |x| *x);
        }
    }
    table.clear_no_drop();
}

#[test]
fn test_clear_no_drop_edge_case_growth() {
    let mut table: RawTable<u32> = RawTable::with_capacity_in(8, Global);
    for i in 0..4 {
        unsafe {
            table.insert(i as u64, i * 20, |x| *x);
        }
    }
    table.insert(4, 80, |x| *x); // Trigger a growth
    table.clear_no_drop();
}

#[test]
fn test_clear_no_drop_large_growth_left() {
    let mut table: RawTable<u32> = RawTable::with_capacity_in(16, Global);
    let growth_left = 15;
    for i in 0..7 {
        unsafe {
            table.insert(i as u64, i * 30, |x| *x);
        }
    }
    table.table.growth_left = growth_left; // Simulate state
    table.clear_no_drop();
}

