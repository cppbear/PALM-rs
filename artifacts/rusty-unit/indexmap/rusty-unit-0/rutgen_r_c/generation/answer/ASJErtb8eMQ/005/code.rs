// Answer 0

#[test]
fn test_get_disjoint_opt_mut_valid_indices() {
    struct TestMap {
        slice: Slice<u32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            let entries = [
                Bucket { hash: 0, key: 1, value: "one".to_string() },
                Bucket { hash: 0, key: 2, value: "two".to_string() },
                Bucket { hash: 0, key: 3, value: "three".to_string() },
            ];
            Self {
                slice: Slice { entries },
            }
        }
    }

    let mut test_map = TestMap::new();
    let indices = [Some(0), Some(1), Some(2)];
    let result = test_map.slice.get_disjoint_opt_mut(indices);
    
    assert!(result.is_ok());
}

#[test]
fn test_get_disjoint_opt_mut_out_of_bounds() {
    struct TestMap {
        slice: Slice<u32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            let entries = [
                Bucket { hash: 0, key: 1, value: "one".to_string() },
                Bucket { hash: 0, key: 2, value: "two".to_string() },
                Bucket { hash: 0, key: 3, value: "three".to_string() },
            ];
            Self {
                slice: Slice { entries },
            }
        }
    }

    let mut test_map = TestMap::new();
    let indices = [Some(0), Some(4)]; // 4 is out of bounds
    let result = test_map.slice.get_disjoint_opt_mut(indices);
    
    assert_eq!(result.unwrap_err(), GetDisjointMutError::IndexOutOfBounds);
}

#[test]
fn test_get_disjoint_opt_mut_overlapping_indices() {
    struct TestMap {
        slice: Slice<u32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            let entries = [
                Bucket { hash: 0, key: 1, value: "one".to_string() },
                Bucket { hash: 0, key: 2, value: "two".to_string() },
                Bucket { hash: 0, key: 3, value: "three".to_string() },
            ];
            Self {
                slice: Slice { entries },
            }
        }
    }

    let mut test_map = TestMap::new();
    let indices = [Some(0), Some(0)]; // both indices are overlapping
    let result = test_map.slice.get_disjoint_opt_mut(indices);
    
    assert_eq!(result.unwrap_err(), GetDisjointMutError::OverlappingIndices);
}

#[test]
fn test_get_disjoint_opt_mut_none_indices() {
    struct TestMap {
        slice: Slice<u32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            let entries = [
                Bucket { hash: 0, key: 1, value: "one".to_string() },
                Bucket { hash: 0, key: 2, value: "two".to_string() },
                Bucket { hash: 0, key: 3, value: "three".to_string() },
            ];
            Self {
                slice: Slice { entries },
            }
        }
    }

    let mut test_map = TestMap::new();
    let indices = [None, None, None]; // no indices
    let result = test_map.slice.get_disjoint_opt_mut(indices);
    
    assert!(result.is_ok());
}

