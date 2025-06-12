// Answer 0

#[test]
fn test_min_len_empty_literals() {
    struct LiteralSet {
        lits: Vec<Literal>,
    }

    struct Literal {
        content: String,
    }

    impl Literal {
        fn len(&self) -> usize {
            self.content.len()
        }
    }

    let empty_set = LiteralSet { lits: vec![] };
    assert_eq!(empty_set.min_len(), None);
}

#[test]
fn test_min_len_single_literal() {
    struct LiteralSet {
        lits: Vec<Literal>,
    }

    struct Literal {
        content: String,
    }

    impl Literal {
        fn len(&self) -> usize {
            self.content.len()
        }
    }

    let single_literal_set = LiteralSet {
        lits: vec![Literal { content: String::from("a") }],
    };
    assert_eq!(single_literal_set.min_len(), Some(1));
}

#[test]
fn test_min_len_multiple_literals() {
    struct LiteralSet {
        lits: Vec<Literal>,
    }

    struct Literal {
        content: String,
    }

    impl Literal {
        fn len(&self) -> usize {
            self.content.len()
        }
    }

    let multiple_literals_set = LiteralSet {
        lits: vec![
            Literal { content: String::from("abc") },
            Literal { content: String::from("a") },
            Literal { content: String::from("ab") },
        ],
    };
    assert_eq!(multiple_literals_set.min_len(), Some(1));
}

#[test]
fn test_min_len_all_same_length() {
    struct LiteralSet {
        lits: Vec<Literal>,
    }

    struct Literal {
        content: String,
    }

    impl Literal {
        fn len(&self) -> usize {
            self.content.len()
        }
    }

    let same_length_set = LiteralSet {
        lits: vec![
            Literal { content: String::from("ab") },
            Literal { content: String::from("cd") },
            Literal { content: String::from("ef") },
        ],
    };
    assert_eq!(same_length_set.min_len(), Some(2));
}

