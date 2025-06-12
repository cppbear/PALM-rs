// Answer 0

#[test]
fn test_bucket_valid_index() {
    let allocator = Global;
    let table_layout = TableLayout::default();
    let capacity = 4; // A power of two
    let mut raw_table = RawTableInner::with_capacity(&allocator, table_layout, capacity);
    
    let index = 1; // 1 is a valid index
    let bucket = unsafe { raw_table.bucket::<i32>(index) };
}

#[test]
fn test_bucket_edge_case_zero_index() {
    let allocator = Global;
    let table_layout = TableLayout::default();
    let capacity = 4; // A power of two
    let mut raw_table = RawTableInner::with_capacity(&allocator, table_layout, capacity);
    
    let index = 0; // Edge case for the lowest valid index
    let bucket = unsafe { raw_table.bucket::<i32>(index) };
}

#[test]
fn test_bucket_max_index() {
    let allocator = Global;
    let table_layout = TableLayout::default();
    let capacity = 4; // A power of two
    let mut raw_table = RawTableInner::with_capacity(&allocator, table_layout, capacity);
    
    let index = 3; // 3 is a valid index for a table with 4 buckets
    let bucket = unsafe { raw_table.bucket::<i32>(index) };
}

#[test]
#[should_panic]
fn test_bucket_invalid_index_too_high() {
    let allocator = Global;
    let table_layout = TableLayout::default();
    let capacity = 4; // A power of two
    let mut raw_table = RawTableInner::with_capacity(&allocator, table_layout, capacity);
    
    let index = 4; // Invalid because it's equal to the number of buckets
    let _bucket = unsafe { raw_table.bucket::<i32>(index) };
}

#[test]
#[should_panic]
fn test_bucket_invalid_index_negative() {
    let allocator = Global;
    let table_layout = TableLayout::default();
    let capacity = 4; // A power of two
    let mut raw_table = RawTableInner::with_capacity(&allocator, table_layout, capacity);
    
    let index = usize::MAX; // Invalid as it's too high
    let _bucket = unsafe { raw_table.bucket::<i32>(index) };
}

