// Answer 0

#[test]
fn test_is_bucket_full_index_zero() {
    let alloc = Global; // Using the global allocator
    let table_layout = TableLayout::default(); // Assuming a default layout
    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, 1);

    unsafe {
        let result = raw_table.is_bucket_full(0);
    }
}

#[test]
fn test_is_bucket_full_index_one() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, 2);

    unsafe {
        let result = raw_table.is_bucket_full(1);
    }
}

#[test]
fn test_is_bucket_full_max_index() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let max_buckets = usize::MAX;
    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, max_buckets);

    unsafe {
        let result = raw_table.is_bucket_full(max_buckets - 1);
    }
}

#[test]
fn test_is_bucket_full_edge_case() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, 4);

    unsafe {
        let result_zero = raw_table.is_bucket_full(0);
        let result_three = raw_table.is_bucket_full(3);
    }
}

#[should_panic]
fn test_is_bucket_full_out_of_bounds() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, 5);

    unsafe {
        let result = raw_table.is_bucket_full(5); // This should panic
    }
}

