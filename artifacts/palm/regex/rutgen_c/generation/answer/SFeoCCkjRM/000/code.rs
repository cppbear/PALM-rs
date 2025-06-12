// Answer 0

#[test]
fn test_remove_complete_empty() {
    let mut literals = Literals {
        lits: vec![],
        limit_size: 0,
        limit_class: 0,
    };
    let result = literals.remove_complete();
    assert!(result.is_empty());
    assert!(literals.lits.is_empty());
}

#[test]
fn test_remove_complete_no_cuts() {
    let mut literals = Literals {
        lits: vec![
            Literal { v: vec![b'a'], cut: false },
            Literal { v: vec![b'b'], cut: false },
        ],
        limit_size: 0,
        limit_class: 0,
    };
    let result = literals.remove_complete();
    assert_eq!(result.len(), 2);
    assert_eq!(literals.lits.len(), 0);
}

#[test]
fn test_remove_complete_with_cuts() {
    let mut literals = Literals {
        lits: vec![
            Literal { v: vec![b'a'], cut: true },
            Literal { v: vec![b'b'], cut: false },
            Literal { v: vec![b'c'], cut: true },
        ],
        limit_size: 0,
        limit_class: 0,
    };
    let result = literals.remove_complete();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].v, vec![b'b']);
    assert_eq!(literals.lits.len(), 2);
    assert_eq!(literals.lits[0].v, vec![b'a']);
    assert_eq!(literals.lits[1].v, vec![b'c']);
}

