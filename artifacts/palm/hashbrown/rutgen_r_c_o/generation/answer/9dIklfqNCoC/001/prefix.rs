// Answer 0

#[derive(Default)]
struct TestAllocator;

#[test]
fn test_find_inner_with_full_buckets() {
    let alloc = TestAllocator::default();
    let mut table = RawTableInner::with_capacity(&alloc, TableLayout::default(), 4);
    
    unsafe {
        let hash = 42;
        let tag = Tag::full(hash);
        let index = 1;

        // Set up a full bucket
        table.set_ctrl_hash(index, hash);
        
        let result = table.find_inner(hash, &mut |i| i == index);
        assert!(result.is_some());
    }
}

#[test]
fn test_find_inner_with_multiple_matching_buckets() {
    let alloc = TestAllocator::default();
    let mut table = RawTableInner::with_capacity(&alloc, TableLayout::default(), 8);
    
    unsafe {
        let hash = 100;
        let tag = Tag::full(hash);
        
        // Set up multiple full buckets
        for i in 0..4 {
            table.set_ctrl_hash(i, hash);
        }

        let result = table.find_inner(hash, &mut |i| i == 2);
        assert!(result.is_some());
    }
}

#[test]
fn test_find_inner_with_empty_bucket() {
    let alloc = TestAllocator::default();
    let mut table = RawTableInner::with_capacity(&alloc, TableLayout::default(), 4);
    
    unsafe {
        let hash = 99;
        let tag = Tag::full(hash);
        let index = 2;

        // Set up a full bucket, leaving one bucket empty
        table.set_ctrl_hash(index, hash);
        
        let result = table.find_inner(hash, &mut |i| i == index);
        assert!(result.is_some());
    }
}

#[test]
fn test_find_inner_with_capacity_limit() {
    let alloc = TestAllocator::default();
    let mut table = RawTableInner::with_capacity(&alloc, TableLayout::default(), 1);
    
    unsafe {
        let hash = 1;
        let tag = Tag::full(hash);
        
        // Set the single bucket to be full
        table.set_ctrl_hash(0, hash);
        
        let result = table.find_inner(hash, &mut |i| i == 0);
        assert!(result.is_some());
    }
}

