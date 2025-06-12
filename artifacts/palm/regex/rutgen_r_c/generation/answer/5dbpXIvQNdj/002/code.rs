// Answer 0

#[test]
fn test_cross_add_with_non_empty_bytes_and_empty_literals() {
    let mut literals = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 1,
    };

    let result = literals.cross_add(&[1, 2, 3, 4, 5]);

    assert!(result);
    assert_eq!(literals.lits.len(), 1);
    assert_eq!(literals.lits[0].v, vec![1]);
    assert!(!literals.lits[0].is_cut());
}

#[test]
fn test_cross_add_with_exact_limit_reached() {
    let mut literals = Literals {
        lits: vec![],
        limit_size: 5,
        limit_class: 1,
    };

    let result = literals.cross_add(&[1, 2, 3, 4, 5]);

    assert!(result);
    assert_eq!(literals.lits.len(), 1);
    assert_eq!(literals.lits[0].v, vec![1, 2, 3, 4, 5]);
    assert!(literals.lits[0].is_cut());
}

#[test]
fn test_cross_add_with_exceeding_limit() {
    let mut literals = Literals {
        lits: vec![],
        limit_size: 2,
        limit_class: 1,
    };

    let result = literals.cross_add(&[1, 2, 3]);

    assert!(!result);
    assert!(literals.lits.is_empty());
}

#[test]
fn test_cross_add_with_single_byte() {
    let mut literals = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 1,
    };

    let result = literals.cross_add(&[10]);

    assert!(result);
    assert_eq!(literals.lits.len(), 1);
    assert_eq!(literals.lits[0].v, vec![10]);
    assert!(!literals.lits[0].is_cut());
}

