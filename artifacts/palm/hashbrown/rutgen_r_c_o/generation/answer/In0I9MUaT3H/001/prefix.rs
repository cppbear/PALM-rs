// Answer 0

#[test]
fn test_bucket_index_valid() {
    let alloc = Global; // Using the global allocator
    let table = RawTable::<u32, Global>::new_in(alloc);
    let bucket_index = 3; // Valid bucket index
    // Assume we initialize the table to contain a valid number of buckets
    let bucket = unsafe { table.bucket(bucket_index) };
    let index = unsafe { table.bucket_index(&bucket) };
}

#[test]
fn test_bucket_index_zero() {
    let alloc = Global; 
    let table = RawTable::<u32, Global>::new_in(alloc);
    let bucket_index = 0; // Minimum valid bucket index
    let bucket = unsafe { table.bucket(bucket_index) };
    let index = unsafe { table.bucket_index(&bucket) };
}

#[test]
fn test_bucket_index_max() {
    let alloc = Global; 
    let table = RawTable::<u32, Global>::new_in(alloc);
    let bucket_index = (1 << 32) - 1; // Maximum valid bucket index
    let bucket = unsafe { table.bucket(bucket_index) };
    let index = unsafe { table.bucket_index(&bucket) };
}

#[test]
#[should_panic]
fn test_bucket_index_out_of_bounds() {
    let alloc = Global; 
    let table = RawTable::<u32, Global>::new_in(alloc);
    let bucket_index = (1 << 32); // Invalid bucket index, should panic
    let bucket = unsafe { table.bucket(bucket_index) };
    let index = unsafe { table.bucket_index(&bucket) };
}

#[test]
#[should_panic]
fn test_bucket_index_invalid_non_null() {
    let alloc = Global; 
    let table = RawTable::<u32, Global>::new_in(alloc);
    let bucket_index = 2; // Choose an index we expect to be valid initially
    let bucket = unsafe { table.bucket(bucket_index) };
    
    let bucket_null = Bucket { ptr: NonNull::dangling() }; // Create an invalid Bucket
    let index = unsafe { table.bucket_index(&bucket_null) }; // Expect it to panic
}

