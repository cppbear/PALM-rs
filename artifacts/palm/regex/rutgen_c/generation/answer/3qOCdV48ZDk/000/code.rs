// Answer 0

#[test]
fn test_longest_common_suffix_empty() {
    let literals = Literals {
        lits: vec![],
        limit_size: 0,
        limit_class: 0,
    };
    assert_eq!(literals.longest_common_suffix(), &[]);
}

#[test]
fn test_longest_common_suffix_single_literal() {
    let literals = Literals {
        lits: vec![Literal::Byte(0xFF)],
        limit_size: 0,
        limit_class: 0,
    };
    assert_eq!(literals.longest_common_suffix(), &[0xFF]);
}

#[test]
fn test_longest_common_suffix_multiple_literals_no_common_suffix() {
    let literals = Literals {
        lits: vec![
            Literal::Byte(0x01),
            Literal::Byte(0x02),
            Literal::Byte(0x03),
        ],
        limit_size: 0,
        limit_class: 0,
    };
    assert_eq!(literals.longest_common_suffix(), &[]);
}

#[test]
fn test_longest_common_suffix_multiple_literals_with_common_suffix() {
    let literals = Literals {
        lits: vec![
            Literal::Byte(0x01),
            Literal::Byte(0x21),
            Literal::Byte(0xA1),
        ],
        limit_size: 0,
        limit_class: 0,
    };
    assert_eq!(literals.longest_common_suffix(), &[]);
} 

#[test]
fn test_longest_common_suffix_multiple_literals_exact_suffix() {
    let literals = Literals {
        lits: vec![
            Literal::Byte(0xFF),
            Literal::Byte(0xFF),
            Literal::Byte(0xFF),
        ],
        limit_size: 0,
        limit_class: 0,
    };
    assert_eq!(literals.longest_common_suffix(), &[0xFF]);
} 

#[test]
fn test_longest_common_suffix_multiple_literals_partial_suffix() {
    let literals = Literals {
        lits: vec![
            Literal::Byte(0xAA),
            Literal::Byte(0xBA),
            Literal::Byte(0xCA),
        ],
        limit_size: 0,
        limit_class: 0,
    };
    assert_eq!(literals.longest_common_suffix(), &[0xA]);
}

