// Answer 0

#[test]
fn test_remove_complete_with_cut_literals() {
    struct TestHir;

    let mut literals = Literals {
        lits: vec![
            Literal { v: vec![b'a'], cut: true },
            Literal { v: vec![b'b'], cut: true },
        ],
        limit_size: 10,
        limit_class: 5,
    };

    let result = literals.remove_complete();

    assert_eq!(result.len(), 0);
    assert_eq!(literals.lits.len(), 2);
    assert!(literals.lits.iter().all(|lit| lit.is_cut()));
}

#[test]
fn test_remove_complete_with_non_cut_literals() {
    struct TestHir;

    let mut literals = Literals {
        lits: vec![
            Literal { v: vec![b'a'], cut: false },
            Literal { v: vec![b'b'], cut: false },
        ],
        limit_size: 10,
        limit_class: 5,
    };

    let result = literals.remove_complete();

    assert_eq!(result.len(), 2);
    assert!(literals.lits.is_empty());
}

#[test]
fn test_remove_complete_with_mixed_literals() {
    struct TestHir;

    let mut literals = Literals {
        lits: vec![
            Literal { v: vec![b'a'], cut: false },
            Literal { v: vec![b'b'], cut: true },
            Literal { v: vec![b'c'], cut: false },
            Literal { v: vec![b'd'], cut: true },
        ],
        limit_size: 10,
        limit_class: 5,
    };

    let result = literals.remove_complete();

    assert_eq!(result.len(), 2);
    assert_eq!(literals.lits.len(), 2);
    assert!(literals.lits.iter().any(|lit| lit.is_cut()));
}

#[test]
fn test_remove_complete_with_empty_literals() {
    struct TestHir;

    let mut literals = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 5,
    };

    let result = literals.remove_complete();

    assert_eq!(result.len(), 0);
    assert!(literals.lits.is_empty());
}

