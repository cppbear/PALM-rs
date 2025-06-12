// Answer 0

#[test]
fn test_cross_add_empty() {
    let mut literals = Literals::empty();
    let result = literals.cross_add(&[]);
    assert!(result);
}

#[test]
fn test_cross_add_initial_addition() {
    let mut literals = Literals::empty();
    let result = literals.cross_add(&[1, 2, 3]);
    assert!(result);
    assert_eq!(literals.lits.len(), 1);
    assert_eq!(literals.lits[0].v, vec![1, 2, 3]);
    assert!(!literals.lits[0].is_cut());
}

#[test]
fn test_cross_add_with_cut() {
    let mut literals = Literals {
        lits: vec![Literal::new(vec![5])],
        limit_size: 5,
        limit_class: 1,
    };
    let result = literals.cross_add(&[1, 2, 3, 4, 5, 6]);
    assert!(result);
    assert_eq!(literals.lits[0].v, vec![5, 1, 2, 3]);
    assert!(literals.lits[0].is_cut());
}

#[test]
fn test_cross_add_exceed_limit() {
    let mut literals = Literals {
        lits: vec![Literal::new(vec![1, 2])],
        limit_size: 3,
        limit_class: 1,
    };
    let result = literals.cross_add(&[4, 5]);
    assert!(!result);
    assert_eq!(literals.lits[0].v, vec![1, 2]);
}

#[test]
fn test_cross_add_partial_fit() {
    let mut literals = Literals {
        lits: vec![Literal::new(vec![10, 11])],
        limit_size: 4,
        limit_class: 1,
    };
    let result = literals.cross_add(&[5, 6, 7]);
    assert!(result);
    assert_eq!(literals.lits[0].v, vec![10, 11, 5]);
    assert!(literals.lits[0].is_cut());
}

#[test]
fn test_cross_add_no_lits() {
    let mut literals = Literals {
        lits: Vec::new(),
        limit_size: 2,
        limit_class: 1,
    };
    let result = literals.cross_add(&[99]);
    assert!(result);
    assert_eq!(literals.lits.len(), 1);
    assert_eq!(literals.lits[0].v, vec![99]);
    assert!(!literals.lits[0].is_cut());
}

