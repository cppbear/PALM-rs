// Answer 0

#[test]
fn test_iter_with_non_empty_table() {
    // Initializing a RawTable with a certain capacity.
    let alloc = Global;
    let capacity = 4; // Power of two
    let mut table = RawTable::with_capacity_in(capacity, alloc);

    // Simulating some insertions (assuming a method exists). 
    unsafe {
        table.insert(1, "Test1", |s: &str| s.len() as u64);
        table.insert(2, "Test2", |s: &str| s.len() as u64);
    }

    // Calling the iter function.
    unsafe {
        let iterator = table.iter();
    }
}

#[test]
fn test_iter_with_empty_table() {
    // Initializing an empty RawTable
    let alloc = Global;
    let capacity = 8; // Power of two
    let table = RawTable::with_capacity_in(capacity, alloc);

    // Calling the iter function on an empty table.
    unsafe {
        let iterator = table.iter();
    }
}

#[test]
fn test_iter_after_resizing_table() {
    // Initialize and fill the RawTable
    let alloc = Global;
    let capacity = 4;
    let mut table = RawTable::with_capacity_in(capacity, alloc);

    unsafe {
        table.insert(3, "Test3", |s: &str| s.len() as u64);
    }

    // Simulate resizing the table.
    let new_capacity = 8; // Resize to a larger capacity (which is also power of two)
    let mut new_table = RawTable::with_capacity_in(new_capacity, alloc);
    
    unsafe {
        for bucket in table.iter() {
            new_table.insert(bucket.hash, bucket.value, |s: &str| s.len() as u64);
        }
    }

    // Call iter on the resized table.
    unsafe {
        let iterator = new_table.iter();
    }
}

#[test]
fn test_iter_with_boundary_buckets() {
    let alloc = Global;
    let buckets = 16; // Power of two
    let mut table = RawTable::with_capacity_in(buckets, alloc);

    // Insert enough items to fill all buckets.
    for i in 0..buckets {
        unsafe {
            table.insert(i as u64, "Test", |s: &str| s.len() as u64);
        }
    }

    // Call iter when all buckets are filled.
    unsafe {
        let iterator = table.iter();
    }
}

