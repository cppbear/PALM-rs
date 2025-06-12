// Answer 0

#[test]
fn test_literals_empty() {
    let literals = Literals::empty();
    assert_eq!(literals.literals().len(), 0);
}

#[test]
fn test_literals_single_unicode() {
    let mut literals = Literals::empty();
    literals.add(Literal::Unicode('a'));
    assert_eq!(literals.literals().len(), 1);
    assert_eq!(literals.literals()[0], Literal::Unicode('a'));
}

#[test]
fn test_literals_single_byte() {
    let mut literals = Literals::empty();
    literals.add(Literal::Byte(97)); // 'a' in ASCII
    assert_eq!(literals.literals().len(), 1);
    assert_eq!(literals.literals()[0], Literal::Byte(97));
}

#[test]
fn test_literals_multiple_literals() {
    let mut literals = Literals::empty();
    literals.add(Literal::Unicode('a'));
    literals.add(Literal::Byte(98)); // 'b' in ASCII
    literals.add(Literal::Unicode('c'));

    assert_eq!(literals.literals().len(), 3);
    assert_eq!(literals.literals()[0], Literal::Unicode('a'));
    assert_eq!(literals.literals()[1], Literal::Byte(98));
    assert_eq!(literals.literals()[2], Literal::Unicode('c'));
}

#[test]
fn test_literals_exceed_limit_size() {
    let mut literals = Literals::empty();
    literals.set_limit_size(2);
    literals.add(Literal::Unicode('x'));
    literals.add(Literal::Byte(121)); // 'y' in ASCII
    literals.add(Literal::Unicode('z')); // This should not be added

    assert_eq!(literals.literals().len(), 2);
    assert_eq!(literals.literals()[0], Literal::Unicode('x'));
    assert_eq!(literals.literals()[1], Literal::Byte(121));
}

