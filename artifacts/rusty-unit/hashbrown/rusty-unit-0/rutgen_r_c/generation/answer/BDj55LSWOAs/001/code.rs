// Answer 0

#[test]
fn test_probe_seq_with_power_of_two_buckets() {
    struct TestAllocator;
    struct TestTableLayout;

    impl TestTableLayout {
        fn calculate_layout_for(&self, buckets: usize) -> Option<(Layout, usize)> {
            Some((Layout::from_size_align(buckets, 1).unwrap(), 0))
        }
    }

    let alloc = TestAllocator;
    let layout = TestTableLayout;

    let raw_table_inner = unsafe {
        RawTableInner::new_uninitialized(&alloc, layout, 16, Fallibility::Infallible).unwrap()
    };
    
    let hash_1 = 10; // Example hash
    let expected_pos_1 = h1(hash_1) & raw_table_inner.bucket_mask;
    
    let probe_sequence_1 = raw_table_inner.probe_seq(hash_1);
    assert_eq!(probe_sequence_1.pos, expected_pos_1);
    assert_eq!(probe_sequence_1.stride, 0);
}

#[test]
fn test_probe_seq_edge_cases() {
    struct TestAllocator;
    struct TestTableLayout;

    impl TestTableLayout {
        fn calculate_layout_for(&self, buckets: usize) -> Option<(Layout, usize)> {
            Some((Layout::from_size_align(buckets, 1).unwrap(), 0))
        }
    }

    let alloc = TestAllocator;
    let layout = TestTableLayout;

    let raw_table_inner = unsafe {
        RawTableInner::new_uninitialized(&alloc, layout, 32, Fallibility::Infallible).unwrap()
    };

    let hash_2 = 31; // Example hash
    let expected_pos_2 = h1(hash_2) & raw_table_inner.bucket_mask;

    let probe_sequence_2 = raw_table_inner.probe_seq(hash_2);
    assert_eq!(probe_sequence_2.pos, expected_pos_2);
    assert_eq!(probe_sequence_2.stride, 0);

    let hash_3 = 0; // Edge case, at least one bucket should be present
    let expected_pos_3 = h1(hash_3) & raw_table_inner.bucket_mask;

    let probe_sequence_3 = raw_table_inner.probe_seq(hash_3);
    assert_eq!(probe_sequence_3.pos, expected_pos_3);
    assert_eq!(probe_sequence_3.stride, 0);
}

#[test]
fn test_probe_seq_large_hash() {
    struct TestAllocator;
    struct TestTableLayout;

    impl TestTableLayout {
        fn calculate_layout_for(&self, buckets: usize) -> Option<(Layout, usize)> {
            Some((Layout::from_size_align(buckets, 1).unwrap(), 0))
        }
    }

    let alloc = TestAllocator;
    let layout = TestTableLayout;

    let raw_table_inner = unsafe {
        RawTableInner::new_uninitialized(&alloc, layout, 64, Fallibility::Infallible).unwrap()
    };

    let hash_4 = u64::MAX; // Testing with maximum hash value
    let expected_pos_4 = h1(hash_4) & raw_table_inner.bucket_mask;

    let probe_sequence_4 = raw_table_inner.probe_seq(hash_4);
    assert_eq!(probe_sequence_4.pos, expected_pos_4);
    assert_eq!(probe_sequence_4.stride, 0);
}

