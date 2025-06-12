// Answer 0

#[test]
fn test_find_inner_valid_index() {
    let allocator = Global;
    let table_layout = TableLayout {};
    let capacity = 8; // Power of two
    let mut table = unsafe { RawTableInner::with_capacity(&allocator, table_layout, capacity) };

    let hash: u64 = 42;  
    let index: usize = 3;   
    let eq = &mut |i| i == index; 

    unsafe {
        // Simulate a full bucket with the tag
        table.set_ctrl_hash(index, Tag::full(hash).0);
        table.ctrl(index).write_bytes(Tag::full(hash).0, 1); // Simulate 'FULL'

        let result = table.find_inner(hash, eq);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), index);
    }
}

#[test]
fn test_find_inner_empty_bucket() {
    let allocator = Global;
    let table_layout = TableLayout {};
    let capacity = 8; // Power of two
    let mut table = unsafe { RawTableInner::with_capacity(&allocator, table_layout, capacity) };

    let hash: u64 = 99; 
    let eq = &mut |_| false; // No index will match, simulating empty buckets

    unsafe {
        // Ensure all buckets are full before testing
        for i in 0..capacity {
            table.set_ctrl_hash(i, Tag::full(123).0);
            table.ctrl(i).write_bytes(Tag::full(123).0, 1); // Simulate 'FULL'
        }

        let result = table.find_inner(hash, eq);
        assert!(result.is_none());
    }
}

#[test]
fn test_find_inner_multiple_buckets() {
    let allocator = Global;
    let table_layout = TableLayout {};
    let capacity = 16; // Power of two
    let mut table = unsafe { RawTableInner::with_capacity(&allocator, table_layout, capacity) };

    let hash: u64 = 55;
    let valid_indices = vec![1, 3, 5]; // Indices that should be considered valid
    let eq = &mut |i| valid_indices.contains(&i);

    unsafe {
        for i in 0..capacity {
            table.set_ctrl_hash(i, Tag::full(hash).0);
            if valid_indices.contains(&i) {
                table.ctrl(i).write_bytes(Tag::full(hash).0, 1); // Simulate 'FULL'
            } else {
                table.ctrl(i).write_bytes(Tag::EMPTY.0, 1); // EMPTY
            }
        }

        let result = table.find_inner(hash, eq);
        assert!(result.is_some());
        assert!(valid_indices.contains(&result.unwrap()));
    }
}

