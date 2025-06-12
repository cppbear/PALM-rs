// Answer 0

#[test]
fn test_cross_product_success() {
    let mut literals_a = Literals {
        lits: vec![
            Literal::Unicode('a'),
            Literal::Unicode('b'),
        ],
        limit_size: 5,
        limit_class: 1,
    };

    let literals_b = Literals {
        lits: vec![
            Literal::Unicode('c'),
            Literal::Byte(1),
        ],
        limit_size: 10,
        limit_class: 1,
    };

    literals_a.cross_product(&literals_b);
}

#[test]
fn test_cross_product_exceed_limit() {
    let mut literals_a = Literals {
        lits: vec![
            Literal::Unicode('x'),
            Literal::Unicode('y'),
        ],
        limit_size: 5,
        limit_class: 1,
    };

    let literals_b = Literals {
        lits: vec![
            Literal::Unicode('z'),
            Literal::Byte(2),
        ],
        limit_size: 10,
        limit_class: 1,
    };

    let result = literals_a.cross_product(&literals_b);
    assert_eq!(result, false);
}

#[test]
fn test_cross_product_with_empty_lits() {
    let mut literals_a = Literals {
        lits: vec![
            Literal::Unicode('a'),
            Literal::Unicode('b'),
        ],
        limit_size: 5,
        limit_class: 1,
    };

    let literals_b = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 1,
    };

    let result = literals_a.cross_product(&literals_b);
    assert_eq!(result, true);
}

#[test]
fn test_cross_product_with_complete_literals() {
    let mut literals_a = Literals {
        lits: vec![
            Literal::Unicode('a'),
            Literal::Unicode('b'),
        ],
        limit_size: 10,
        limit_class: 1,
    };

    let literals_b = Literals {
        lits: vec![
            Literal::Unicode('c'),
            Literal::Byte(1),
        ],
        limit_size: 10,
        limit_class: 1,
    };

    literals_a.lits[0].cut = false;
    literals_a.lits[1].cut = false;
    literals_b.lits[0].cut = false;
    literals_b.lits[1].cut = false;

    let result = literals_a.cross_product(&literals_b);
    assert_eq!(result, true);
}

