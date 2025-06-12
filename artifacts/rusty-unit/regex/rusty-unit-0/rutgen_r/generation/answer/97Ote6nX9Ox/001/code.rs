// Answer 0

#[test]
fn test_len_non_empty() {
    struct Sparse {
        size: usize,
    }

    let sparse = Sparse { size: 10 };
    assert_eq!(sparse.len(), 10);
}

#[test]
fn test_len_zero() {
    struct Sparse {
        size: usize,
    }

    let sparse = Sparse { size: 0 };
    assert_eq!(sparse.len(), 0);
}

#[test]
fn test_len_large_value() {
    struct Sparse {
        size: usize,
    }

    let sparse = Sparse { size: usize::MAX };
    assert_eq!(sparse.len(), usize::MAX);
}

