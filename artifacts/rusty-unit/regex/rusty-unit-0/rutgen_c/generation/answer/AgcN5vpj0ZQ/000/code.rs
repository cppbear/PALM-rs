// Answer 0

#[test]
fn test_to_empty_creates_new_empty_literals() {
    let literals = Literals::empty();
    let empty_literals = literals.to_empty();
    assert_eq!(empty_literals.lits.len(), 0);
    assert_eq!(empty_literals.limit_size, 250);
    assert_eq!(empty_literals.limit_class, 10);
}

#[test]
fn test_to_empty_retains_limits() {
    let mut literals = Literals::empty();
    literals.set_limit_size(300);
    literals.set_limit_class(15);
    
    let empty_literals = literals.to_empty();
    assert_eq!(empty_literals.limit_size, 300);
    assert_eq!(empty_literals.limit_class, 15);
}

#[test]
fn test_to_empty_on_non_empty_literals() {
    let mut literals = Literals::empty();
    literals.add(Literal::Unicode('a'));
    let empty_literals = literals.to_empty();
    assert_eq!(empty_literals.lits.len(), 0);
    assert_eq!(empty_literals.limit_size, 250);
    assert_eq!(empty_literals.limit_class, 10);
}

#[test]
fn test_to_empty_on_modified_literals() {
    let mut literals = Literals::empty();
    literals.set_limit_size(100);
    literals.set_limit_class(5);
    literals.add(Literal::Byte(10));
    
    let empty_literals = literals.to_empty();
    assert_eq!(empty_literals.lits.len(), 0);
    assert_eq!(empty_literals.limit_size, 100);
    assert_eq!(empty_literals.limit_class, 5);
}

