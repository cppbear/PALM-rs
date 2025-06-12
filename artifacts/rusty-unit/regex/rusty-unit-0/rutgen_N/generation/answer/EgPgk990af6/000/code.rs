// Answer 0

#[test]
fn test_empty_literals() {
    struct Literals {
        lits: Vec<String>,
        limit_size: usize,
        limit_class: usize,
    }
    
    fn empty() -> Literals {
        Literals {
            lits: vec![],
            limit_size: 250,
            limit_class: 10,
        }
    }

    let literals = empty();
    assert_eq!(literals.lits.len(), 0);
    assert_eq!(literals.limit_size, 250);
    assert_eq!(literals.limit_class, 10);
}

