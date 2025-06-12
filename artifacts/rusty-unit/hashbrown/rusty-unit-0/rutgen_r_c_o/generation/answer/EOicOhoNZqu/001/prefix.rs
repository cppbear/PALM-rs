// Answer 0

#[test]
fn test_capacity_zero_items_zero_growth() {
    let table = RawTable::<u8>::new_in(Global);
    let result = table.capacity();
}

#[test]
fn test_capacity_non_zero_items_zero_growth() {
    let mut table = RawTable::<u8>::with_capacity_in(5, Global);
    // Simulating adding items
    let _ = table.insert(1, 42, |v| *v);
    let result = table.capacity();
}

#[test]
fn test_capacity_zero_items_non_zero_growth() {
    let mut table = RawTable::<u8>::with_capacity_in(5, Global);
    // Simulating growth
    table.table.growth_left = 10;
    let result = table.capacity();
}

#[test]
fn test_capacity_non_zero_items_non_zero_growth() {
    let mut table = RawTable::<u8>::with_capacity_in(5, Global);
    // Simulating adding items
    let _ = table.insert(1, 42, |v| *v);
    table.table.growth_left = 10;
    let result = table.capacity();
}

#[test]
fn test_capacity_large_items_large_growth() {
    let mut table = RawTable::<u8>::with_capacity_in(u32::MAX as usize, Global);
    // Simulating adding maximum items
    for i in 0..u32::MAX {
        let _ = table.insert(i as u64, i, |v| *v);
    }
    table.table.growth_left = u32::MAX as usize;
    let result = table.capacity();
}

