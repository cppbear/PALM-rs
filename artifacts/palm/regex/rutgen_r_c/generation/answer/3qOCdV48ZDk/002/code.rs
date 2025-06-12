// Answer 0

#[test]
fn test_longest_common_suffix_multiple_literals() {
    struct HirDummy;

    let lit1 = Literal::Byte(0b11000001); // Let's assume it's a byte character
    let lit2 = Literal::Byte(0b11000001);
    let lit3 = Literal::Byte(0b11000011);
    let mut literals = Literals {
        lits: vec![
            lit1.clone(),
            lit2.clone(),
            lit3.clone(),
        ],
        limit_size: 10,
        limit_class: 10,
    };

    let result = literals.longest_common_suffix();

    assert_eq!(result, &[0b11000001]);
}

#[test]
fn test_longest_common_suffix_no_common_suffix() {
    struct HirDummy;

    let lit1 = Literal::Byte(0b11000001);
    let lit2 = Literal::Byte(0b11000010);
    let mut literals = Literals {
        lits: vec![lit1, lit2],
        limit_size: 10,
        limit_class: 10,
    };

    let result = literals.longest_common_suffix();

    assert_eq!(result, &[]);
}

#[test]
fn test_longest_common_suffix_empty() {
    struct HirDummy;

    let literals = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 10,
    };

    let result = literals.longest_common_suffix();

    assert_eq!(result, &[]);
}

#[test]
fn test_longest_common_suffix_single_literal() {
    struct HirDummy;

    let lit = Literal::Byte(0b11000001);
    let mut literals = Literals {
        lits: vec![lit],
        limit_size: 10,
        limit_class: 10,
    };

    let result = literals.longest_common_suffix();

    assert_eq!(result, &[0b11000001]);
}

