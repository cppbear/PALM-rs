// Answer 0

#[test]
fn test_limit_size() {
    let literals = Literals {
        lits: Vec::new(),
        limit_size: 0,
        limit_class: 0,
    };
    assert_eq!(literals.limit_size(), 0);

    let literals_with_limit = Literals {
        lits: vec![Literal::Unicode('a')],
        limit_size: 10,
        limit_class: 0,
    };
    assert_eq!(literals_with_limit.limit_size(), 10);

    let literals_with_another_limit = Literals {
        lits: vec![Literal::Byte(255)],
        limit_size: 100,
        limit_class: 1,
    };
    assert_eq!(literals_with_another_limit.limit_size(), 100);
}

