// Answer 0

#[test]
fn test_bucket_ptr_valid_index() {
    let allocator = Global;
    let table_layout = TableLayout::default(); // Assuming a default layout
    let capacity = 8; // Example capacity to yield a specific number of buckets
    let table = RawTableInner::with_capacity(&allocator, table_layout, capacity);
    let index = 0; // Valid index
    let size_of = 1; // Valid size_of
    unsafe {
        table.bucket_ptr(index, size_of);
    }
}

#[test]
fn test_bucket_ptr_mid_index() {
    let allocator = Global;
    let table_layout = TableLayout::default();
    let capacity = 8;
    let table = RawTableInner::with_capacity(&allocator, table_layout, capacity);
    let index = 3; // Valid index
    let size_of = 2; // Valid size_of
    unsafe {
        table.bucket_ptr(index, size_of);
    }
}

#[test]
fn test_bucket_ptr_max_index() {
    let allocator = Global;
    let table_layout = TableLayout::default();
    let capacity = 8; 
    let table = RawTableInner::with_capacity(&allocator, table_layout, capacity);
    let index = 7; // Valid index, last index in buckets 
    let size_of = 4; // Valid size_of
    unsafe {
        table.bucket_ptr(index, size_of);
    }
}

#[test]
fn test_bucket_ptr_large_size_of() {
    let allocator = Global;
    let table_layout = TableLayout::default();
    let capacity = 8; 
    let table = RawTableInner::with_capacity(&allocator, table_layout, capacity);
    let index = 2; // Valid index
    let size_of = 256; // Maximum size_of
    unsafe {
        table.bucket_ptr(index, size_of);
    }
}

#[should_panic]
fn test_bucket_ptr_invalid_index() {
    let allocator = Global;
    let table_layout = TableLayout::default();
    let capacity = 8; 
    let table = RawTableInner::with_capacity(&allocator, table_layout, capacity);
    let index = 8; // Invalid index (equal to buckets)
    let size_of = 1; 
    unsafe {
        table.bucket_ptr(index, size_of);
    }
}

