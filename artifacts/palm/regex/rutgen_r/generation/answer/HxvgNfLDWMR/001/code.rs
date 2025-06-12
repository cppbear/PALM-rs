// Answer 0

#[test]
fn test_capacity_empty_dense() {
    struct Sparse {
        dense: Vec<i32>,
    }

    let sparse = Sparse { dense: Vec::new() };
    assert_eq!(sparse.capacity(), 0);
}

#[test]
fn test_capacity_non_empty_dense() {
    struct Sparse {
        dense: Vec<i32>,
    }

    let sparse = Sparse { dense: vec![1, 2, 3] };
    assert_eq!(sparse.capacity(), 3);
}

#[test]
fn test_capacity_large_dense() {
    struct Sparse {
        dense: Vec<i32>,
    }

    let sparse = Sparse { dense: vec![0; 1000] };
    assert_eq!(sparse.capacity(), 1000);
}

#[test]
fn test_capacity_single_element_dense() {
    struct Sparse {
        dense: Vec<i32>,
    }

    let sparse = Sparse { dense: vec![42] };
    assert_eq!(sparse.capacity(), 1);
}

