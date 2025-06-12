// Answer 0

#[test]
fn test_to_empty() {
    let literals = Literals::empty();
    let empty_lit = literals.to_empty();
    assert_eq!(empty_lit.lits.len(), 0);
    assert_eq!(empty_lit.limit_size, 250);
    assert_eq!(empty_lit.limit_class, 10);
}

#[test]
fn test_to_empty_with_limits() {
    let mut literals = Literals::empty();
    literals.set_limit_size(300);
    literals.set_limit_class(5);
    let empty_lit = literals.to_empty();
    assert_eq!(empty_lit.lits.len(), 0);
    assert_eq!(empty_lit.limit_size, 300);
    assert_eq!(empty_lit.limit_class, 5);
}

#[test]
fn test_to_empty_after_adding_literals() {
    let mut literals = Literals::empty();
    literals.add(Literal::Unicode('a'));
    literals.add(Literal::Byte(0xFF));
    let empty_lit = literals.to_empty();
    assert_eq!(empty_lit.lits.len(), 0);
    assert_eq!(empty_lit.limit_size, 250);
    assert_eq!(empty_lit.limit_class, 10);
}

