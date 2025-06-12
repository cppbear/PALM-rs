// Answer 0

#[test]
fn test_class_exceeds_limits_true_1() {
    let mut literals = Literals::empty();
    literals.set_limit_class(0);
    assert!(literals.class_exceeds_limits(1));
}

#[test]
fn test_class_exceeds_limits_true_2() {
    let mut literals = Literals::empty();
    literals.set_limit_class(5);
    assert!(literals.class_exceeds_limits(6));
}

#[test]
fn test_class_exceeds_limits_true_3() {
    let mut literals = Literals::empty();
    literals.set_limit_class(10);
    assert!(literals.class_exceeds_limits(11));
}

#[test]
fn test_class_exceeds_limits_true_4() {
    let mut literals = Literals::empty();
    literals.set_limit_class(3);
    literals.set_limit_size(10);
    literals.add(Literal::Unicode('a'));
    assert!(literals.class_exceeds_limits(4));
}

