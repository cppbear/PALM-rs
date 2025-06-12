// Answer 0

#[test]
fn test_unambiguous_prefixes_empty() {
    let literals = Literals::empty();
    let result = literals.unambiguous_prefixes();
    assert!(result.is_empty());
}

#[test]
fn test_unambiguous_prefixes_single_literal() {
    let mut literals = Literals::empty();
    let lit = Literal::new(vec![b'a']);
    literals.add(lit.clone());
    let result = literals.unambiguous_prefixes();
    assert_eq!(result.literals(), &[lit]);
}

#[test]
fn test_unambiguous_prefixes_no_overlap() {
    let mut literals = Literals::empty();
    literals.add(Literal::new(vec![b'a']));
    literals.add(Literal::new(vec![b'b']));
    let result = literals.unambiguous_prefixes();
    assert_eq!(result.literals(), &[
        Literal::new(vec![b'a']),
        Literal::new(vec![b'b']),
    ]);
}

#[test]
fn test_unambiguous_prefixes_with_overlap() {
    let mut literals = Literals::empty();
    literals.add(Literal::new(vec![b"a".to_vec()]));
    literals.add(Literal::new(vec![b"ab".to_vec()]));
    let result = literals.unambiguous_prefixes();
    assert_eq!(result.literals().len(), 1);
    assert_eq!(result.literals()[0], Literal::new(vec![b"a".to_vec()]));
}

#[test]
fn test_unambiguous_prefixes_with_cut_literals() {
    let mut literals = Literals::empty();
    let mut cut_lit = Literal::new(vec![b"a".to_vec()]);
    cut_lit.cut();
    literals.add(cut_lit);
    literals.add(Literal::new(vec![b"ab".to_vec()]));
    let result = literals.unambiguous_prefixes();
    assert_eq!(result.literals().len(), 1);
    assert_eq!(result.literals()[0], Literal::new(vec![b"a".to_vec()]));
}

