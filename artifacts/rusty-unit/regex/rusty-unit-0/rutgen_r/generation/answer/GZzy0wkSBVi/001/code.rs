// Answer 0

#[test]
fn test_prefixes_empty_hir() {
    struct Hir {}
    struct Literals {
        prefixes: Vec<String>,
    }

    impl Literals {
        fn empty() -> Self {
            Literals { prefixes: Vec::new() }
        }

        fn union_prefixes(&mut self, _expr: &Hir) {
            // Simulating an empty prefix for an empty Hir
            self.prefixes.clear();
        }
    }

    let expr = &Hir {};
    let lits = prefixes(expr);
    assert!(lits.prefixes.is_empty());
}

#[test]
fn test_prefixes_single_literal_hir() {
    struct Hir {
        literal: String,
    }
    
    struct Literals {
        prefixes: Vec<String>,
    }

    impl Literals {
        fn empty() -> Self {
            Literals { prefixes: Vec::new() }
        }

        fn union_prefixes(&mut self, expr: &Hir) {
            if !expr.literal.is_empty() {
                self.prefixes.push(expr.literal.clone());
            }
        }
    }

    let expr = &Hir {
        literal: "a".to_string(),
    };
    let lits = prefixes(expr);
    assert_eq!(lits.prefixes.len(), 1);
    assert_eq!(lits.prefixes[0], "a");
}

#[test]
fn test_prefixes_multiple_literal_hir() {
    struct Hir {
        literals: Vec<String>,
    }

    struct Literals {
        prefixes: Vec<String>,
    }

    impl Literals {
        fn empty() -> Self {
            Literals { prefixes: Vec::new() }
        }

        fn union_prefixes(&mut self, expr: &Hir) {
            for literal in &expr.literals {
                self.prefixes.push(literal.clone());
            }
        }
    }

    let expr = &Hir {
        literals: vec!["a".to_string(), "b".to_string(), "c".to_string()],
    };
    let lits = prefixes(expr);
    assert_eq!(lits.prefixes.len(), 3);
    assert_eq!(lits.prefixes, vec!["a", "b", "c"]);
}

#[test]
fn test_prefixes_edge_case_hir() {
    struct Hir {
        literal: String,
    }
    
    struct Literals {
        prefixes: Vec<String>,
    }

    impl Literals {
        fn empty() -> Self {
            Literals { prefixes: Vec::new() }
        }

        fn union_prefixes(&mut self, expr: &Hir) {
            if !expr.literal.is_empty() {
                self.prefixes.push(expr.literal.clone());
            }
        }
    }

    let expr = &Hir {
        literal: "".to_string(),
    };
    let lits = prefixes(expr);
    assert!(lits.prefixes.is_empty());
}

