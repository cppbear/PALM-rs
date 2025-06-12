// Answer 0

#[test]
fn test_bucket_ptr_with_zero_index() {
    let alloc = Global; 
    let table_layout = TableLayout::default(); 
    let capacity = 1;
    let table = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    let size_of = 1; 
    unsafe {
        let ptr = table.bucket_ptr(0, size_of);
    }
}

#[test]
fn test_bucket_ptr_with_upper_index() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let capacity = 2; 
    let table = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    let size_of = 1; 
    unsafe {
        let _ptr = table.bucket_ptr(table.buckets(), size_of);
    }
}

#[test]
#[should_panic]
fn test_bucket_ptr_with_invalid_index_exceeding_buckets() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let capacity = 4; 
    let table = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    let size_of = 4; 
    unsafe {
        let _ptr = table.bucket_ptr(table.buckets(), size_of);
    }
}

#[test]
fn test_bucket_ptr_with_multiple_size_of() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let capacity = 4; 
    let table = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    let size_of = 4; 
    unsafe {
        let ptr1 = table.bucket_ptr(0, size_of);
        let ptr2 = table.bucket_ptr(1, size_of);
        let ptr3 = table.bucket_ptr(2, size_of);
        let ptr4 = table.bucket_ptr(3, size_of);
    }
}

#[test]
fn test_bucket_ptr_with_large_size_of() {
    let alloc = Global;
    let table_layout = TableLayout::default();
    let capacity = 8; 
    let table = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    let size_of = 1024; 
    unsafe {
        let ptr = table.bucket_ptr(0, size_of);
    }
}

