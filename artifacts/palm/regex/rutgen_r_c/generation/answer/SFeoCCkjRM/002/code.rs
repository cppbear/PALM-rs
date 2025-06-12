// Answer 0

#[test]
fn test_remove_complete_empty_literals() {
    let mut literals = Literals {
        lits: vec![],
        limit_size: 0,
        limit_class: 0,
    };
    let result = literals.remove_complete();
    assert_eq!(result, vec![]);
}

#[test]
fn test_remove_complete_no_cut_literals() {
    let mut literals = Literals {
        lits: vec![
            Literal::Unicode('a'),
            Literal::Unicode('b'),
            Literal::Unicode('c'),
        ],
        limit_size: 3,
        limit_class: 3,
    };
    let result = literals.remove_complete();
    assert_eq!(result.len(), 3);
    assert!(literals.lits.is_empty());
    assert_eq!(result, vec![
        Literal::Unicode('a'),
        Literal::Unicode('b'),
        Literal::Unicode('c'),
    ]);
}

#[test]
fn test_remove_complete_with_cut_literals() {
    let mut literals = Literals {
        lits: vec![
            Literal::Unicode('x'),
            Literal { cut: true, ..Literal::Unicode('y') },
            Literal::Unicode('z'),
        ],
        limit_size: 3,
        limit_class: 3,
    };
    let result = literals.remove_complete();
    assert_eq!(result.len(), 2);
    assert_eq!(literals.lits.len(), 1);
    assert_eq!(result, vec![
        Literal::Unicode('x'),
        Literal::Unicode('z'),
    ]);
    assert_eq!(literals.lits, vec![
        Literal { cut: true, ..Literal::Unicode('y') },
    ]);
}

#[test]
fn test_remove_complete_all_cut_literals() {
    let mut literals = Literals {
        lits: vec![
            Literal { cut: true, ..Literal::Unicode('a') },
            Literal { cut: true, ..Literal::Unicode('b') },
        ],
        limit_size: 2,
        limit_class: 2,
    };
    let result = literals.remove_complete();
    assert_eq!(result.len(), 0);
    assert_eq!(literals.lits.len(), 2);
}

