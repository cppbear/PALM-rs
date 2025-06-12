// Answer 0

#[test]
fn test_cut_with_non_empty_literals() {
    struct TestHir;
    let mut literals = Literals {
        lits: vec![
            Literal::Unicode('a'),
            Literal::Byte(0),
        ],
        limit_size: 10,
        limit_class: 10,
    };

    literals.cut();

    assert!(matches!(literals.lits[0], Literal::Unicode('a') if literals.lits[0].is_cut()));
    assert!(matches!(literals.lits[1], Literal::Byte(0) if literals.lits[1].is_cut()));
}

#[test]
fn test_cut_with_empty_literals() {
    struct TestHir;
    let mut literals = Literals {
        lits: vec![],
        limit_size: 10,
        limit_class: 10,
    };

    literals.cut();

    assert!(literals.lits.is_empty());
}

#[test]
fn test_cut_with_literally_cut_and_uncut_literals() {
    struct TestHir;
    let mut literals = Literals {
        lits: vec![
            Literal::Unicode('b'),
            Literal::Byte(1),
        ],
        limit_size: 10,
        limit_class: 10,
    };

    literals.lits[0].cut(); // Manually cut one literal
    
    literals.cut(); // Now cut all again

    assert!(literals.lits[0].is_cut());
    assert!(literals.lits[1].is_cut());
}

