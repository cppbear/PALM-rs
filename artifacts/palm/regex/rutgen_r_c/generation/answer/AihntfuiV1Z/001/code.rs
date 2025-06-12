// Answer 0

#[test]
fn test_num_bytes_empty() {
    let literals = Literals {
        lits: vec![],
        limit_size: 0,
        limit_class: 0,
    };
    assert_eq!(literals.num_bytes(), 0);
}

#[test]
fn test_num_bytes_single_unicode() {
    let literals = Literals {
        lits: vec![
            Literal::Unicode('a'),
        ],
        limit_size: 0,
        limit_class: 0,
    };
    assert_eq!(literals.num_bytes(), 1);
}

#[test]
fn test_num_bytes_multiple_unicode() {
    let literals = Literals {
        lits: vec![
            Literal::Unicode('a'),
            Literal::Unicode('b'),
            Literal::Unicode('c'),
        ],
        limit_size: 0,
        limit_class: 0,
    };
    assert_eq!(literals.num_bytes(), 3);
}

#[test]
fn test_num_bytes_single_byte() {
    let literals = Literals {
        lits: vec![
            Literal::Byte(0b10101010),
        ],
        limit_size: 0,
        limit_class: 0,
    };
    assert_eq!(literals.num_bytes(), 1);
}

#[test]
fn test_num_bytes_mixed_literals() {
    let literals = Literals {
        lits: vec![
            Literal::Unicode('a'),
            Literal::Byte(0b10101010),
            Literal::Unicode('b'),
        ],
        limit_size: 0,
        limit_class: 0,
    };
    assert_eq!(literals.num_bytes(), 3);
}

#[test]
fn test_num_bytes_large_input() {
    let literals = Literals {
        lits: (0..1000).map(|i| Literal::Unicode(char::from(i as u8))).collect(),
        limit_size: 0,
        limit_class: 0,
    };
    assert_eq!(literals.num_bytes(), 1000);
}

