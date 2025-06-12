// Answer 0

#[test]
fn test_capacity_non_empty() {
    struct Sparse {
        dense: Vec<usize>,
    }

    let sparse = Sparse {
        dense: vec![1, 2, 3],
    };
    
    assert_eq!(sparse.capacity(), 3);
}

#[test]
fn test_capacity_empty() {
    struct Sparse {
        dense: Vec<usize>,
    }

    let sparse = Sparse {
        dense: vec![],
    };
    
    assert_eq!(sparse.capacity(), 0);
}

