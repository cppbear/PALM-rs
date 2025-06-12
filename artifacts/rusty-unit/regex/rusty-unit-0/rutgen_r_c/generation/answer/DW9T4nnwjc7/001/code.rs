// Answer 0

#[test]
fn test_set_limit_class_zero() {
    let mut literals = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 0,
    };
    let result = literals.set_limit_class(0);
    assert_eq!(literals.limit_class, 0);
    assert_eq!(result.limit_class, 0);
}

#[test]
fn test_set_limit_class_positive() {
    let mut literals = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 5,
    };
    let result = literals.set_limit_class(3);
    assert_eq!(literals.limit_class, 3);
    assert_eq!(result.limit_class, 3);
}

#[test]
fn test_set_limit_class_increase() {
    let mut literals = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 2,
    };
    let result = literals.set_limit_class(4);
    assert_eq!(literals.limit_class, 4);
    assert_eq!(result.limit_class, 4);
}

#[test]
fn test_set_limit_class_same_value() {
    let mut literals = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 1,
    };
    let result = literals.set_limit_class(1);
    assert_eq!(literals.limit_class, 1);
    assert_eq!(result.limit_class, 1);
}

#[test]
#[should_panic]
fn test_set_limit_class_negative() {
    let mut literals = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 5,
    };
    literals.set_limit_class((-1) as usize);
}

