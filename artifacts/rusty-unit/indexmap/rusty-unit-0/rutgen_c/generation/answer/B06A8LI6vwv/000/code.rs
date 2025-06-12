// Answer 0

#[test]
fn test_get_disjoint_mut_valid_indices() {
    struct TestBucket {
        hash: u64,
        key: usize,
        value: String,
    }

    let mut slice = Slice {
        entries: [
            Bucket { hash: 1, key: 0, value: "value0".to_string() },
            Bucket { hash: 2, key: 1, value: "value1".to_string() },
            Bucket { hash: 3, key: 2, value: "value2".to_string() },
        ],
    };
    
    let indices = [0, 1];
    
    let result = slice.get_disjoint_mut(indices);
    
    assert!(result.is_ok());
    let key_values = result.unwrap();

    assert_eq!(key_values[0].0, &0);
    assert_eq!(key_values[0].1, &mut "value0".to_string());
    assert_eq!(key_values[1].0, &1);
    assert_eq!(key_values[1].1, &mut "value1".to_string());
}

#[test]
#[should_panic]
fn test_get_disjoint_mut_out_of_bounds_index() {
    struct TestBucket {
        hash: u64,
        key: usize,
        value: String,
    }

    let mut slice = Slice {
        entries: [
            Bucket { hash: 1, key: 0, value: "value0".to_string() },
            Bucket { hash: 2, key: 1, value: "value1".to_string() },
        ],
    };
    
    let indices = [0, 2]; // 2 is out of bounds
    
    let _ = slice.get_disjoint_mut(indices).unwrap();
}

#[test]
#[should_panic]
fn test_get_disjoint_mut_repeating_indices() {
    struct TestBucket {
        hash: u64,
        key: usize,
        value: String,
    }

    let mut slice = Slice {
        entries: [
            Bucket { hash: 1, key: 0, value: "value0".to_string() },
            Bucket { hash: 2, key: 1, value: "value1".to_string() },
        ],
    };
    
    let indices = [0, 0]; // Repeating index
    
    let _ = slice.get_disjoint_mut(indices).unwrap();
}

#[test]
fn test_get_disjoint_mut_empty_slice() {
    struct TestBucket {
        hash: u64,
        key: usize,
        value: String,
    }

    let mut slice: Slice<usize, String> = Slice { entries: [] };

    let indices = [0]; // Out of bounds
    
    let result = slice.get_disjoint_mut(indices);
    
    assert!(result.is_err());
}

