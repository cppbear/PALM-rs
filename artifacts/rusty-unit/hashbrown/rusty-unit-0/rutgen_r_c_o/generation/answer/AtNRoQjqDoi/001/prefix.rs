// Answer 0

#[test]
fn test_resize_with_min_capacity_and_items() {
    let mut table: RawTable<usize> = RawTable::new_in(Global);
    // Initialize table with some items
    for i in 0..1 {
        unsafe {
            table.insert(0, i, |x| *x);
        }
    }
    unsafe {
        let result = table.resize(1, |x| *x, Fallibility::Infallible);
    }
}

#[test]
fn test_resize_with_exact_capacity_and_items() {
    let mut table: RawTable<usize> = RawTable::new_in(Global);
    // Initialize table with the exact capacity
    for i in 0..2 {
        unsafe {
            table.insert(0, i, |x| *x);
        }
    }
    unsafe {
        let result = table.resize(2, |x| *x, Fallibility::Infallible);
    }
}

#[test]
fn test_resize_with_excess_capacity() {
    let mut table: RawTable<usize> = RawTable::new_in(Global);
    // Initialize table with some items
    for i in 0..3 {
        unsafe {
            table.insert(0, i, |x| *x);
        }
    }
    unsafe {
        let result = table.resize(4, |x| *x, Fallibility::Infallible);
    }
}

#[test]
fn test_resize_to_capacity_zero_with_items() {
    let mut table: RawTable<usize> = RawTable::new_in(Global);
    // Initialize table with items
    for i in 0..1 {
        unsafe {
            table.insert(0, i, |x| *x);
        }
    }
    // Ensure this panic occurs when resizing to capacity 0
    #[should_panic]
    unsafe {
        table.resize(0, |x| *x, Fallibility::Infallible);
    }
}

#[test]
fn test_resize_with_edge_bucket_conditions() {
    let mut table: RawTable<usize> = RawTable::new_in(Global);
    // Initialize table with items
    for i in 0..8 {
        unsafe {
            table.insert(0, i, |x| *x);
        }
    }
    unsafe {
        let result = table.resize(16, |x| *x, Fallibility::Infallible);
    }
}

#[test]
fn test_resize_with_excess_items() {
    let mut table: RawTable<usize> = RawTable::new_in(Global);
    // Try to insert more items than capacity allows
    let initial_capacity = 4;
    for i in 0..5 {
        unsafe {
            table.insert(0, i, |x| *x);
        }
    }
    // Ensure this does not panic but handles input correctly
    unsafe {
        let result = table.resize(initial_capacity, |x| *x, Fallibility::Infallible);
    }
}

