// Answer 0

#[test]
fn test_bucket_with_valid_index() {
    let alloc = Global;
    let mut table: RawTable<u32, _> = RawTable::new_in(alloc);
    let buckets = 4; // 2^n where n = 2
    unsafe {
        // Ensure the table has the valid capacity
        table = RawTable::new_uninitialized(alloc, buckets, Fallibility::All).unwrap();
        let bucket = table.bucket(1); // valid index
        let bucket2 = table.bucket(2); // valid index
    }
}

#[test]
#[should_panic]
fn test_bucket_with_out_of_bounds_index() {
    let alloc = Global;
    let mut table: RawTable<u32, _> = RawTable::new_in(alloc);
    let buckets = 4; // 2^n where n = 2
    unsafe {
        table = RawTable::new_uninitialized(alloc, buckets, Fallibility::All).unwrap();
        let _bucket = table.bucket(4); // index equal to bucket count (invalid)
    }
}

#[test]
fn test_bucket_with_zero_index() {
    let alloc = Global;
    let mut table: RawTable<u32, _> = RawTable::new_in(alloc);
    let buckets = 4; // 2^n where n = 2
    unsafe {
        table = RawTable::new_uninitialized(alloc, buckets, Fallibility::All).unwrap();
        let bucket = table.bucket(0); // valid index
    }
}

#[test]
fn test_bucket_with_max_valid_index() {
    let alloc = Global;
    let mut table: RawTable<u32, _> = RawTable::new_in(alloc);
    let buckets = 4; // 2^n where n = 2
    unsafe {
        table = RawTable::new_uninitialized(alloc, buckets, Fallibility::All).unwrap();
        let bucket = table.bucket(3); // valid index
    }
}

#[test]
fn test_bucket_after_growth() {
    let alloc = Global;
    let mut table: RawTable<u32, _> = RawTable::with_capacity_in(4, alloc); // initial capacity
    unsafe {
        table.insert(1, 10, |v| *v); // Insert to ensure there's data
        table.resize(8, |v| *v, Fallibility::All).unwrap(); // Grow the table
        let bucket = table.bucket(1); // valid index after growth
    }
}

