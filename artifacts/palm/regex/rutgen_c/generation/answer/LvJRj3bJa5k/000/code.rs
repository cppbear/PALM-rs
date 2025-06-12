// Answer 0

#[test]
fn test_trim_suffix_with_no_literls() {
    let literals = Literals::empty();
    let result = literals.trim_suffix(1);
    assert_eq!(result, None);
}

#[test]
fn test_trim_suffix_with_single_literal() {
    let mut literals = Literals::empty();
    literals.add(Literal::Unicode('a'));
    let result = literals.trim_suffix(1);
    assert_eq!(result.unwrap().literals().len(), 1);
    assert_eq!(result.unwrap().literals()[0], Literal::Unicode('a'));
}

#[test]
fn test_trim_suffix_removes_cut_literals() {
    let mut literals = Literals::empty();
    literals.add(Literal::Unicode('a'));
    literals.add(Literal::Unicode('b'));
    let result = literals.trim_suffix(1);
    assert_eq!(result.unwrap().literals().len(), 2);
    assert_eq!(result.unwrap().literals()[0], Literal::Unicode('a'));
    assert_eq!(result.unwrap().literals()[1], Literal::Unicode('b'));
}

#[test]
fn test_trim_suffix_completely_removes_literals() {
    let mut literals = Literals::empty();
    literals.add(Literal::Unicode('a'));
    literals.add(Literal::Byte(1));
    let result = literals.trim_suffix(1);
    assert_eq!(result, None);
}

#[test]
fn test_trim_suffix_with_duplicate_literals() {
    let mut literals = Literals::empty();
    literals.add(Literal::Unicode('a'));
    literals.add(Literal::Unicode('b'));
    literals.add(Literal::Unicode('a'));
    let result = literals.trim_suffix(0);
    assert_eq!(result.unwrap().literals().len(), 2);
}

#[test]
fn test_trim_suffix_with_trimmed_length_equal_to_literal_length() {
    let mut literals = Literals::empty();
    literals.add(Literal::Unicode('a'));
    let result = literals.trim_suffix(1);
    assert_eq!(result, None);
}

