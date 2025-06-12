// Answer 0

#[test]
fn test_shrink_to_min_size_zero() {
    let mut table = RawTable::new_in(Global);
    
    // Initial conditions
    for i in 0..1 {
        let _bucket = table.insert(i as u64, i, |x| *x);
    }

    unsafe {
        table.shrink_to(0, |x| *x);
    }
}

#[test]
fn test_shrink_to_min_size_one() {
    let mut table = RawTable::with_capacity_in(1, Global);
    
    // Initial conditions
    let _bucket = table.insert(1, 0, |x| *x);

    unsafe {
        table.shrink_to(1, |x| *x);
    }
}

#[test]
fn test_shrink_to_boundary_case() {
    let mut table = RawTable::with_capacity_in(2, Global);
    
    // Initial conditions
    let _bucket_1 = table.insert(1, 0, |x| *x);
    let _bucket_2 = table.insert(2, 1, |x| *x);

    unsafe {
        table.shrink_to(1, |x| *x);
    }
}

#[test]
fn test_shrink_to_exceeding_capacity() {
    let mut table = RawTable::with_capacity_in(2, Global);
    
    // Initial conditions
    let _bucket_1 = table.insert(1, 0, |x| *x);
    let _bucket_2 = table.insert(2, 1, |x| *x);

    unsafe {
        table.shrink_to(2, |x| *x);
    }
}

#[test]
fn test_shrink_to_min_buckets() {
    let mut table = RawTable::new_in(Global);
    
    // Initial conditions
    for i in 0..2 {
        let _bucket = table.insert(i as u64, i, |x| *x);
    }

    unsafe {
        table.shrink_to(1, |x| *x);
    }
}

