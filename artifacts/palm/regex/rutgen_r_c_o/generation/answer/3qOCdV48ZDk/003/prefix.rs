// Answer 0

#[test]
fn test_longest_common_suffix_single_literal() {
    let mut literals = Literals::empty();
    literals.add(Literal::Byte(1));
    let suffix = literals.longest_common_suffix();
}

#[test]
fn test_longest_common_suffix_multiple_literals_same_suffix() {
    let mut literals = Literals::empty();
    literals.add(Literal::Byte(1));
    literals.add(Literal::Byte(2));
    literals.add(Literal::Byte(3));
    let suffix = literals.longest_common_suffix();
}

#[test]
fn test_longest_common_suffix_multiple_literals_different_suffixes() {
    let mut literals = Literals::empty();
    literals.add(Literal::Byte(1));
    literals.add(Literal::Byte(2));
    literals.add(Literal::Byte(1));
    literals.add(Literal::Byte(3));
    let suffix = literals.longest_common_suffix();
}

#[test]
fn test_longest_common_suffix_empty_suffixed() {
    let mut literals = Literals::empty();
    literals.add(Literal::Byte(1));
    literals.add(Literal::Byte(2));
    literals.add(Literal::Byte(0));
    let suffix = literals.longest_common_suffix();
}

#[test]
fn test_longest_common_suffix_only_empty_literals() {
    let mut literals = Literals::empty();
    literals.add(Literal::Byte(0));
    literals.add(Literal::Byte(0));
    let suffix = literals.longest_common_suffix();
}

#[test]
fn test_longest_common_suffix_various_lengths() {
    let mut literals = Literals::empty();
    literals.add(Literal::Byte(1));
    literals.add(Literal::Byte(1));
    literals.add(Literal::Byte(1));
    literals.add(Literal::Byte(1));
    let suffix = literals.longest_common_suffix();
}

