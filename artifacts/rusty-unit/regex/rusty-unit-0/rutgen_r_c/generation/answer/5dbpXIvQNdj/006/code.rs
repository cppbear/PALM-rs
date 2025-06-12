// Answer 0

#[test]
fn test_cross_add_empty_lits() {
    let mut literals = Literals::empty();
    let result = literals.cross_add(&[1, 2, 3]);
    assert_eq!(result, true);
    assert_eq!(literals.literals().len(), 1);
    assert_eq!(literals.literals()[0].is_cut(), false);
}

#[test]
fn test_cross_add_exceeding_limit() {
    let mut literals = Literals {
        lits: vec![Literal::new(vec![1])],
        limit_size: 1,
        limit_class: 1,
    };
    let result = literals.cross_add(&[1, 2]);
    assert_eq!(result, false);
}

#[test]
fn test_cross_add_fitting_limit() {
    let mut literals = Literals {
        lits: vec![Literal::new(vec![1])],
        limit_size: 5,
        limit_class: 1,
    };
    let result = literals.cross_add(&[1, 2]);
    assert_eq!(result, true);
    assert_eq!(literals.literals.len(), 1);
    assert_eq!(literals.literals()[0].v, vec![1, 1]);
    assert_eq!(literals.literals()[0].is_cut(), false);
}

#[test]
fn test_cross_add_cut_lit() {
    let mut literals = Literals {
        lits: vec![Literal::new(vec![1])],
        limit_size: 3,
        limit_class: 1,
    };
    let result = literals.cross_add(&[1, 2, 3]);
    assert_eq!(result, true);
    assert_eq!(literals.literals.len(), 1);
    assert_eq!(literals.literals()[0].v, vec![1, 1]);
    assert_eq!(literals.literals()[0].is_cut(), true);
}

#[test]
fn test_cross_add_increase_limit() {
    let mut literals = Literals {
        lits: vec![Literal::new(vec![1])],
        limit_size: 3,
        limit_class: 1,
    };
    let result = literals.cross_add(&[1, 2]);
    assert_eq!(result, true);
    assert_eq!(literals.literals.len(), 1);
    assert_eq!(literals.literals()[0].v, vec![1, 1]);
    assert_eq!(literals.literals()[0].is_cut(), false);
}

