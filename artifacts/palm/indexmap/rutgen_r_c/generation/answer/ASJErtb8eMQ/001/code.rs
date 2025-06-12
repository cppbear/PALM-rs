// Answer 0

#[test]
fn test_get_disjoint_opt_mut_success() {
    let mut slice = Box::new(Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: "one" },
            Bucket { hash: 0, key: 2, value: "two" },
            Bucket { hash: 0, key: 3, value: "three" },
        ],
    });

    let result = slice.get_disjoint_opt_mut([Some(0), Some(1), None]);
    assert!(result.is_ok());
    let values = result.unwrap();
    assert_eq!(values[0].as_ref().map(|(&k, _)| k), Some(1));
    assert_eq!(values[1].as_ref().map(|(&k, _)| k), Some(2));
    assert_eq!(values[2], None);
}

#[test]
fn test_get_disjoint_opt_mut_index_out_of_bounds() {
    let mut slice = Box::new(Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: "one" },
            Bucket { hash: 0, key: 2, value: "two" },
        ],
    });

    let result = slice.get_disjoint_opt_mut([Some(0), Some(3)]);
    assert_eq!(result, Err(GetDisjointMutError::IndexOutOfBounds));
}

#[test]
fn test_get_disjoint_opt_mut_overlapping_indices() {
    let mut slice = Box::new(Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: "one" },
            Bucket { hash: 0, key: 2, value: "two" },
        ],
    });

    let result = slice.get_disjoint_opt_mut([Some(0), Some(0)]);
    assert_eq!(result, Err(GetDisjointMutError::OverlappingIndices));
}

#[test]
fn test_get_disjoint_opt_mut_empty_slice() {
    let mut slice = Box::new(Slice {
        entries: [],
    });

    let result = slice.get_disjoint_opt_mut([None; 1]);
    assert!(result.is_ok());
    let values = result.unwrap();
    assert_eq!(values, [None]);
}

#[test]
fn test_get_disjoint_opt_mut_multiple_none() {
    let mut slice = Box::new(Slice {
        entries: [
            Bucket { hash: 0, key: 1, value: "one" },
            Bucket { hash: 0, key: 2, value: "two" },
        ],
    });

    let result = slice.get_disjoint_opt_mut([None, None]);
    assert!(result.is_ok());
    let values = result.unwrap();
    assert_eq!(values, [None, None]);
}

