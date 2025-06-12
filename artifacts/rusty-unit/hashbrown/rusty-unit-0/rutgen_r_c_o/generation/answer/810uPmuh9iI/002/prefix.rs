// Answer 0

#[test]
fn test_clear_non_empty_table_with_elements() {
    let mut table = RawTable::<i32>::with_capacity_in(10, Global);
    for i in 0..5 {
        unsafe {
            table.insert(i as u64, i, |x| x.clone() as u64);
        }
    }
    table.clear();
}

#[test]
fn test_clear_full_table() {
    let mut table = RawTable::<i32>::with_capacity_in(16, Global);
    for i in 0..16 {
        unsafe {
            table.insert(i as u64, i, |x| x.clone() as u64);
        }
    }
    table.clear();
}

#[test]
fn test_clear_partially_filled_table() {
    let mut table = RawTable::<i32>::with_capacity_in(20, Global);
    for i in 0..15 {
        unsafe {
            table.insert(i as u64, i, |x| x.clone() as u64);
        }
    }
    table.clear();
}

#[test]
fn test_clear_with_multiple_insertions() {
    let mut table = RawTable::<i32>::with_capacity_in(10, Global);
    for i in 0..8 {
        unsafe {
            table.insert(i as u64, i, |x| x.clone() as u64);
        }
    }
    table.clear();
    for i in 8..15 {
        unsafe {
            table.insert(i as u64, i, |x| x.clone() as u64);
        }
    }
    table.clear();
}

#[test]
fn test_clear_when_not_empty() {
    let mut table = RawTable::<i32>::with_capacity_in(5, Global);
    for i in [1, 2, 3, 4] {
        unsafe {
            table.insert(i as u64, i, |x| x.clone() as u64);
        }
    }
    table.clear();
}

#[test]
#[should_panic]
fn test_clear_after_panic_inducing_operation() {
    let mut table = RawTable::<i32>::with_capacity_in(5, Global);
    for i in 0..5 {
        unsafe {
            table.insert(i as u64, i, |x| x.clone() as u64);
        }
    }
    // Trigger a panic by executing an unguarded operation
    unsafe {
        let _ = table.remove(Bucket::<i32>::default());
    }
    table.clear();
}

