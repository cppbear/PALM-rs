// Answer 0

#[test]
fn test_get_disjoint_opt_mut_index_out_of_bounds() {
    // Create a Slice with a fixed number of entries
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: 10 },
            Bucket { hash: 1, key: 2, value: 20 },
            Bucket { hash: 2, key: 3, value: 30 },
        ],
    };

    // Attempt to access an out-of-bounds index
    let indices = [Some(3), Some(2)]; // 3 is out of bounds (slice length is 3)
    let result = slice.get_disjoint_opt_mut(indices);
    
    // Assert that we got the expected error
    assert_eq!(result, Err(GetDisjointMutError::IndexOutOfBounds));
}

#[test]
fn test_get_disjoint_opt_mut_overlapping_indices() {
    // Create a Slice with a few entries
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: 10 },
            Bucket { hash: 1, key: 2, value: 20 },
            Bucket { hash: 2, key: 3, value: 30 },
        ],
    };

    // Attempt to access overlapping indices
    let indices = [Some(1), Some(1)]; // Both access the same index (1)
    let result = slice.get_disjoint_opt_mut(indices);

    // Assert that we got the expected error
    assert_eq!(result, Err(GetDisjointMutError::OverlappingIndices));
}

#[test]
fn test_get_disjoint_opt_mut_success() {
    // Create a Slice with a few entries
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: 10 },
            Bucket { hash: 1, key: 2, value: 20 },
            Bucket { hash: 2, key: 3, value: 30 },
        ],
    };

    // Access valid indices
    let indices = [Some(0), Some(1)]; // Both are valid indices
    let result = slice.get_disjoint_opt_mut(indices);

    // Assert that we got the expected result
    let expected = [
        Some((&1, &mut 10)),
        Some((&2, &mut 20)),
    ];
    assert_eq!(result, Ok(expected));
}

