// Answer 0

#[test]
fn test_longest_common_prefix_empty() {
    let literals = Literals::empty();
    assert_eq!(literals.longest_common_prefix(), &[]);
}

#[test]
fn test_longest_common_prefix_single_literal() {
    let literal = Literal::Unicode('a');
    let literals = Literals {
        lits: vec![literal.clone()],
        limit_size: 0,
        limit_class: 0,
    };
    assert_eq!(literals.longest_common_prefix(), b"a");
}

#[test]
fn test_longest_common_prefix_multiple_literals_common() {
    let literals = Literals {
        lits: vec![
            Literal::Byte(b'a'),
            Literal::Byte(b'a'),
            Literal::Byte(b'a'),
        ],
        limit_size: 0,
        limit_class: 0,
    };
    assert_eq!(literals.longest_common_prefix(), b"a");
}

#[test]
fn test_longest_common_prefix_multiple_literals_no_common() {
    let literals = Literals {
        lits: vec![
            Literal::Byte(b'a'),
            Literal::Byte(b'b'),
            Literal::Byte(b'c'),
        ],
        limit_size: 0,
        limit_class: 0,
    };
    assert_eq!(literals.longest_common_prefix(), b"");
}

#[test]
fn test_longest_common_prefix_multiple_literals_partial() {
    let literals = Literals {
        lits: vec![
            Literal::Byte(b"abcdef"[0]),
            Literal::Byte(b"abcxyz"[0]),
        ],
        limit_size: 0,
        limit_class: 0,
    };
    assert_eq!(literals.longest_common_prefix(), b"abc");
}

