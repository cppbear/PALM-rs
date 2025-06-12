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

    let result = empty();
    assert_eq!(result.lits, vec![]);
    assert_eq!(result.limit_size, 250);
    assert_eq!(result.limit_class, 10);
}

