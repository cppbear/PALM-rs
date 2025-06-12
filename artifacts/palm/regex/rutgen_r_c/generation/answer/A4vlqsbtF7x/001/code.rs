// Answer 0

#[test]
fn test_unambiguous_prefixes_empty_literals() {
    let literals = Literals::empty();
    let result = literals.unambiguous_prefixes();
    assert_eq!(result.is_empty(), true);
}

#[test]
fn test_unambiguous_prefixes_one_empty_literal() {
    let mut literals = Literals::empty();
    literals.add(Literal::empty());
    let result = literals.unambiguous_prefixes();
    assert_eq!(result.is_empty(), true);
}

#[test]
fn test_unambiguous_prefixes_single_character() {
    let mut literals = Literals::empty();
    literals.add(Literal::Unicode('a'));
    let result = literals.unambiguous_prefixes();
    assert_eq!(result.literals().len(), 1);
    assert_eq!(result.literals()[0], Literal::Unicode('a'));
}

#[test]
fn test_unambiguous_prefixes_multiple_identical_literals() {
    let mut literals = Literals::empty();
    literals.add(Literal::Unicode('a'));
    literals.add(Literal::Unicode('a')); // duplicate
    let result = literals.unambiguous_prefixes();
    assert_eq!(result.literals().len(), 1);
    assert_eq!(result.literals()[0], Literal::Unicode('a'));
}

#[test]
fn test_unambiguous_prefixes_prefixes_with_cut_literals() {
    let mut literals = Literals::empty();
    let mut lit_a = Literal::new(vec![b'a']);
    lit_a.cut();
    let mut lit_b = Literal::new(vec![b'a', b'b']);
    literals.add(lit_a);
    literals.add(lit_b);
    
    let result = literals.unambiguous_prefixes();
    assert_eq!(result.literals().len(), 1);
    assert_eq!(result.literals()[0].len(), 1);
}

