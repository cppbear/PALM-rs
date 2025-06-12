// Answer 0

#[test]
fn test_all_complete_non_empty_lits() {
    let mut literals = Literals {
        lits: vec![
            Literal::Unicode('a'),
            Literal::Unicode('b'),
            Literal::Unicode('c'),
        ],
        limit_size: 10,
        limit_class: 5,
    };
    literals.all_complete();
}

#[test]
fn test_all_complete_large_vector() {
    let mut literals = Literals {
        lits: (0..1000).map(|i| Literal::Unicode(char::from((i % 26) + 97))).collect(),
        limit_size: 100,
        limit_class: 50,
    };
    literals.all_complete();
}

#[test]
fn test_all_complete_with_cut_lit() {
    let mut literals = Literals {
        lits: vec![
            Literal::Unicode('x'),
            Literal::Unicode('y'),
            {
                let mut lit = Literal::Unicode('z');
                lit.cut();
                lit
            }
        ],
        limit_size: 5,
        limit_class: 3,
    };
    literals.all_complete();
}

#[test]
fn test_all_complete_single_literal() {
    let mut literals = Literals {
        lits: vec![Literal::Unicode('m')],
        limit_size: 1,
        limit_class: 1,
    };
    literals.all_complete();
}

#[test]
fn test_all_complete_multiple_non_cut_literals() {
    let mut literals = Literals {
        lits: vec![
            Literal::Unicode('j'),
            Literal::Unicode('k'),
            Literal::Unicode('l'),
            Literal::Unicode('m'),
            Literal::Unicode('n'),
        ],
        limit_size: 10,
        limit_class: 5,
    };
    literals.all_complete();
}

