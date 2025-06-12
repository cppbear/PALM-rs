// Answer 0

#[test]
fn test_shrink_to_non_empty() {
    let mut table = RawTable::new_in(Global);
    // Simulate adding some elements to the table
    for i in 0..10 {
        let _bucket = table.insert(i as u64, i, |x| *x);
    }
    unsafe {
        table.shrink_to(5, |x| *x);
    }
}

#[test]
fn test_shrink_to_zero_elements() {
    let mut table = RawTable::new_in(Global);
    // No elements to shrink
    unsafe {
        table.shrink_to(0, |x| *x);
    }
}

#[test]
fn test_shrink_to_exceed_current_size() {
    let mut table = RawTable::new_in(Global);
    // Simulate adding some elements to the table
    for i in 0..5 {
        let _bucket = table.insert(i as u64, i, |x| *x);
    }
    unsafe {
        table.shrink_to(10, |x| *x);
    }
}

#[test]
fn test_shrink_to_exact_size() {
    let mut table = RawTable::new_in(Global);
    // Simulate adding some elements to the table
    for i in 0..8 {
        let _bucket = table.insert(i as u64, i, |x| *x);
    }
    unsafe {
        table.shrink_to(8, |x| *x);
    }
}

#[test]
fn test_shrink_to_min_size() {
    let mut table = RawTable::new_in(Global);
    // Simulate adding some elements to the table
    for i in 0..4 {
        let _bucket = table.insert(i as u64, i, |x| *x);
    }
    unsafe {
        table.shrink_to(2, |x| *x);
    }
}

