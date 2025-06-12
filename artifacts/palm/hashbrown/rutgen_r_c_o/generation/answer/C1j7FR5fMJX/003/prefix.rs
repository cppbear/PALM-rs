// Answer 0

#[test]
fn test_bucket_index_valid() {
    let alloc = Global;
    let mut table: RawTable<u32, _> = RawTable::with_capacity_in(8, alloc);
    let bucket_index = table.buckets(); // This is expected to be valid in the next test
    unsafe {
        let bucket = table.bucket(bucket_index - 1);
    }
}

#[test]
#[should_panic]
fn test_bucket_index_out_of_bounds() {
    let alloc = Global;
    let mut table: RawTable<u32, _> = RawTable::with_capacity_in(8, alloc);
    let bucket_index = table.buckets(); // This will cause an out of bounds access
    unsafe {
        let bucket = table.bucket(bucket_index);
    }
}

