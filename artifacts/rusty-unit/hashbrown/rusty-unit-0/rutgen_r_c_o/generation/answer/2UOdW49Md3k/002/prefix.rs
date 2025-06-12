// Answer 0

#[test]
fn test_bucket_valid_index() {
    let table_layout = TableLayout::default();
    let allocator = Global;
    let mut raw_table = RawTableInner::with_capacity(&allocator, table_layout, 8); // 8 buckets (2^3)

    unsafe {
        let bucket = raw_table.bucket::<i32>(1); // Index is 1, which is valid
        let ptr = bucket.as_ptr(); // Fetching the pointer from the bucket
    }
}

#[test]
fn test_bucket_boundary_index() {
    let table_layout = TableLayout::default();
    let allocator = Global;
    let mut raw_table = RawTableInner::with_capacity(&allocator, table_layout, 8); // 8 buckets (2^3)

    unsafe {
        let bucket = raw_table.bucket::<i32>(7); // Last valid index for 8 buckets
        let ptr = bucket.as_ptr(); // Fetching the pointer from the bucket
    }
}

#[test]
#[should_panic]
fn test_bucket_over_index() {
    let table_layout = TableLayout::default();
    let allocator = Global;
    let mut raw_table = RawTableInner::with_capacity(&allocator, table_layout, 8); // 8 buckets (2^3)

    unsafe {
        let bucket = raw_table.bucket::<i32>(8); // Index is out of bounds
        let ptr = bucket.as_ptr(); // This line should panic
    }
}

#[test]
fn test_bucket_with_capacity() {
    let table_layout = TableLayout::default();
    let allocator = Global;
    let raw_table = RawTableInner::with_capacity(&allocator, table_layout, 16); // 16 buckets (2^4)

    unsafe {
        let bucket = raw_table.bucket::<i32>(0); // Using the minimum valid index
        let ptr = bucket.as_ptr(); // Fetching the pointer from the bucket
    }
}

