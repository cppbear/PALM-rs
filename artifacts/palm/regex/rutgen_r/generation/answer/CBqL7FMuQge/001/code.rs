// Answer 0

#[derive(Default)]
struct Sparse {
    size: usize,
}

#[test]
fn test_empty_sparse() {
    let sparse = Sparse::default(); // size is 0
    assert!(sparse.is_empty());
}

#[test]
fn test_non_empty_sparse() {
    let sparse = Sparse { size: 1 }; // size is not 0
    assert!(!sparse.is_empty());
}

#[test]
fn test_another_non_empty_sparse() {
    let sparse = Sparse { size: 100 }; // size is not 0
    assert!(!sparse.is_empty());
}

#[test]
fn test_large_non_empty_sparse() {
    let sparse = Sparse { size: usize::MAX }; // boundary condition for usize
    assert!(!sparse.is_empty());
}

