// Answer 0

#[test]
fn test_empty_literls() {
    let literals = Literals::empty();
    assert_eq!(literals.lits.len(), 0);
    assert_eq!(literals.limit_size, 250);
    assert_eq!(literals.limit_class, 10);
}

