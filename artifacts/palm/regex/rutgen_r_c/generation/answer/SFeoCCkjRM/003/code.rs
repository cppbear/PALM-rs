// Answer 0

#[test]
fn test_remove_complete_empty_literals() {
    let mut literals = Literals {
        lits: vec![],
        limit_size: 0,
        limit_class: 0,
    };
    let removed = literals.remove_complete();
    assert_eq!(removed, vec![]);
    assert!(literals.is_empty());
}

#[test]
fn test_remove_complete_no_complete_literals() {
    let mut literals = Literals {
        lits: vec![
            Literal {
                v: vec![b'a'],
                cut: false,
            },
            Literal {
                v: vec![b'b'],
                cut: false,
            },
        ],
        limit_size: 0,
        limit_class: 0,
    };
    let removed = literals.remove_complete();
    assert_eq!(removed, vec![]);
    assert_eq!(literals.lits.len(), 2); // No literals should have been removed.
}

#[test]
fn test_remove_complete_with_complete_literals() {
    let mut literals = Literals {
        lits: vec![
            Literal {
                v: vec![b'a'],
                cut: true,
            },
            Literal {
                v: vec![b'b'],
                cut: false,
            },
            Literal {
                v: vec![b'c'],
                cut: true,
            },
        ],
        limit_size: 0,
        limit_class: 0,
    };
    let removed = literals.remove_complete();
    assert_eq!(removed.len(), 1); // One non-cut literal should be returned.
    assert!(!literals.is_empty()); // Literal with cut should remain.
    assert_eq!(literals.lits.len(), 2); // One cut literal and one non-cut should remain.
}

#[test]
fn test_remove_complete_all_complete_literals() {
    let mut literals = Literals {
        lits: vec![
            Literal {
                v: vec![b'a'],
                cut: true,
            },
            Literal {
                v: vec![b'b'],
                cut: true,
            },
        ],
        limit_size: 0,
        limit_class: 0,
    };
    let removed = literals.remove_complete();
    assert_eq!(removed.len(), 0); // No non-cut literals available.
    assert_eq!(literals.lits.len(), 2); // Both cut literals should remain.
}

