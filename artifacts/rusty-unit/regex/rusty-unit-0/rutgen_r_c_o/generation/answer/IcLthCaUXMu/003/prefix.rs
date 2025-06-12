// Answer 0

#[test]
fn test_class_exceeds_limits_limit_class_equal_size() {
    let mut literals = Literals {
        lits: vec![Literal::Unicode('a'), Literal::Unicode('b')],
        limit_size: 10,
        limit_class: 2,
    };
    literals.add(Literal::Unicode('c'));
    let result = literals.class_exceeds_limits(2);
}

#[test]
fn test_class_exceeds_limits_size_with_lits_not_empty() {
    let mut literals = Literals {
        lits: vec![Literal::Unicode('x'), Literal::Byte(255)],
        limit_size: 8,
        limit_class: 2,
    };
    literals.add(Literal::Unicode('y'));
    let result = literals.class_exceeds_limits(2);
}

#[test]
fn test_class_exceeds_limits_large_limit_size() {
    let mut literals = Literals {
        lits: vec![Literal::Unicode('d'), Literal::Unicode('e')],
        limit_size: 100,
        limit_class: 4,
    };
    literals.add(Literal::Byte(100));
    let result = literals.class_exceeds_limits(4);
}

#[test]
fn test_class_exceeds_limits_small_limit_size() {
    let mut literals = Literals {
        lits: vec![Literal::Unicode('f')],
        limit_size: 5,
        limit_class: 1,
    };
    let result = literals.class_exceeds_limits(1);
}

#[test]
fn test_class_exceeds_limits_multiple_literas() {
    let mut literals = Literals {
        lits: vec![Literal::Byte(10), Literal::Byte(20), Literal::Unicode('g')],
        limit_size: 15,
        limit_class: 3,
    };
    let result = literals.class_exceeds_limits(3);
}

