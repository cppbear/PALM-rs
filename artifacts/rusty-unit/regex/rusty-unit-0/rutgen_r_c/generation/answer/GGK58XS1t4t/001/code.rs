// Answer 0

#[test]
fn test_literals_empty() {
    let literals = Literals::empty();
    assert_eq!(literals.literals(), &[]);
}

#[test]
fn test_literals_non_empty() {
    let mut literals = Literals::empty();
    literals.add(Literal::Unicode('a'));
    literals.add(Literal::Byte(1));
    let result = literals.literals();
    assert_eq!(result.len(), 2);
    assert!(result.contains(&Literal::Unicode('a')));
    assert!(result.contains(&Literal::Byte(1)));
}

#[test]
fn test_literals_same_character() {
    let mut literals = Literals::empty();
    literals.add(Literal::Unicode('a'));
    literals.add(Literal::Unicode('a'));
    let result = literals.literals();
    assert_eq!(result.len(), 2);
}

#[test]
fn test_literals_multiple_bytes() {
    let mut literals = Literals::empty();
    literals.add(Literal::Byte(1));
    literals.add(Literal::Byte(2));
    literals.add(Literal::Byte(3));
    let result = literals.literals();
    assert_eq!(result.len(), 3);
    assert!(result.contains(&Literal::Byte(1)));
    assert!(result.contains(&Literal::Byte(2)));
    assert!(result.contains(&Literal::Byte(3)));
}

