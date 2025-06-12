// Answer 0

#[test]
fn test_next_impl_with_index_in_first_group() {
    struct TestGroup {
        data: BitMask,
    }

    unsafe impl Group for TestGroup {
        const WIDTH: usize = 8;
        fn load_aligned(ptr: *const u8) -> Self {
            // Initialize with a specific pattern for testing purposes
            TestGroup { data: BitMask(0b00001111) } // Four set bits
        }
    }

    let mut group = TestGroup { data: BitMask(0b00001111) };
    let mut full_buckets_indices = FullBucketsIndices {
        current_group: BitMaskIter(group.data),
        group_first_index: 0,
        ctrl: NonNull::new_unchecked(&mut 0u8),
        items: 4,
    };

    assert_eq!(unsafe { full_buckets_indices.next_impl() }, Some(0));
    assert_eq!(unsafe { full_buckets_indices.next_impl() }, Some(1));
    assert_eq!(unsafe { full_buckets_indices.next_impl() }, Some(2));
    assert_eq!(unsafe { full_buckets_indices.next_impl() }, Some(3));
    assert_eq!(unsafe { full_buckets_indices.next_impl() }, None);
}

#[test]
fn test_next_impl_handles_empty_group() {
    struct TestGroup {
        data: BitMask,
    }

    unsafe impl Group for TestGroup {
        const WIDTH: usize = 8;
        fn load_aligned(ptr: *const u8) -> Self {
            TestGroup { data: BitMask(0b00000000) } // No set bits
        }
    }

    let mut group = TestGroup { data: BitMask(0b00000000) };
    let mut full_buckets_indices = FullBucketsIndices {
        current_group: BitMaskIter(group.data),
        group_first_index: 0,
        ctrl: NonNull::new_unchecked(&mut 0u8),
        items: 0,
    };

    assert_eq!(unsafe { full_buckets_indices.next_impl() }, None);
}

#[test]
fn test_next_impl_iterations_over_multiple_groups() {
    struct TestGroup {
        data: BitMask,
    }

    unsafe impl Group for TestGroup {
        const WIDTH: usize = 8;
        fn load_aligned(ptr: *const u8) -> Self {
            TestGroup { data: BitMask(0b11111111) } // All bits set
        }
    }

    let mut group1 = TestGroup { data: BitMask(0b11111111) };
    let mut group2 = TestGroup { data: BitMask(0b11111111) };
    let mut full_buckets_indices = FullBucketsIndices {
        current_group: BitMaskIter(group1.data),
        group_first_index: 0,
        ctrl: NonNull::new_unchecked(&mut 0u8),
        items: 16,
    };

    for i in 0..16 {
        assert_eq!(unsafe { full_buckets_indices.next_impl() }, Some(i));
    }
    assert_eq!(unsafe { full_buckets_indices.next_impl() }, None);
}

