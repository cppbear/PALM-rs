// Answer 0

#[test]
fn test_set_ctrl_valid_index_within_bounds() {
    let buckets = 8; // 2^3
    let bucket_mask = buckets - 1;
    let alloc = Global; // Assume we are using the Global allocator
    let table_layout = TableLayout::default(); // Assume default initialization
    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, buckets);
    
    unsafe {
        let ctrl = Tag(1);
        raw_table.set_ctrl(0, ctrl); // Test lower bound
        raw_table.set_ctrl(bucket_mask, ctrl); // Test upper bound
    }
}

#[test]
#[should_panic]
fn test_set_ctrl_index_out_of_bounds() {
    let buckets = 8; // 2^3
    let bucket_mask = buckets - 1;
    let alloc = Global; // Assume we are using the Global allocator
    let table_layout = TableLayout::default(); // Assume default initialization
    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, buckets);
    
    unsafe {
        let ctrl = Tag(1);
        raw_table.set_ctrl(bucket_mask + 1, ctrl); // Should panic
    }
}

#[test]
fn test_set_ctrl_with_different_tags() {
    let buckets = 16; // 2^4
    let bucket_mask = buckets - 1;
    let alloc = Global; 
    let table_layout = TableLayout::default(); 
    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, buckets);
    
    unsafe {
        let tag1 = Tag(1);
        let tag2 = Tag(2);
        raw_table.set_ctrl(0, tag1); // Test with Tag(1) at index 0
        raw_table.set_ctrl(bucket_mask, tag2); // Test with Tag(2) at last index
    }
} 

#[test]
fn test_set_ctrl_edge_case_max_index_within_bounds() {
    let buckets = 4; // 2^2
    let bucket_mask = buckets - 1;
    let alloc = Global; 
    let table_layout = TableLayout::default(); 
    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, buckets);
    
    unsafe {
        let ctrl = Tag(3);
        raw_table.set_ctrl(3, ctrl); // Test with max index 3
    }
} 

#[test]
#[should_panic]
fn test_set_ctrl_negative_index_panic() {
    let buckets = 8; 
    let alloc = Global; 
    let table_layout = TableLayout::default(); 
    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, buckets);
    
    unsafe {
        let ctrl = Tag(0);
        raw_table.set_ctrl(usize::MAX, ctrl); // Should panic due to negative index
    }
}

