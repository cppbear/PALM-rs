// Answer 0

#[cfg(test)]
fn test_suffixes() {
    struct Literal {
        value: Vec<u8>,
    }

    impl Literal {
        fn empty() -> Self {
            Self { value: Vec::new() }
        }
        
        fn new(value: Vec<u8>) -> Self {
            Self { value }
        }
    }

    struct Literals {
        literals: Vec<Literal>,
    }

    impl Literals {
        fn new() -> Self {
            Self { literals: Vec::new() }
        }

        fn cross_add(&mut self, lit: &[u8]) {
            self.literals.push(Literal::new(lit.to_vec()));
        }

        fn is_empty(&self) -> bool {
            self.literals.is_empty()
        }

        fn cut(&mut self) {
            self.literals.clear();
        }

        fn add(&mut self, lit: Literal) {
            self.literals.push(lit);
        }

        fn to_empty(&self) -> Self {
            Self::new()
        }

        fn cross_product(&mut self, other: &Literals) -> bool {
            // A dummy implementation for cross_product
            // This would ideally involve combining literals
            self.literals.extend(other.literals.clone());
            true
        }

        fn any_complete(&self) -> bool {
            !self.is_empty()
        }
    }

    enum HirKind {
        Alternation(Vec<Hir>),
    }

    struct Hir {
        kind: HirKind,
    }

    fn suffixes(expr: &Hir, lits: &mut Literals) {
        match &expr.kind {
            HirKind::Alternation(es) => {
                for e in es.iter() {
                    // Recursive call for each expression in the alternation
                    suffixes(e, lits);
                }
            }
        }
    }

    #[test]
    fn test_suffixes_with_alternation() {
        let mut lits = Literals::new();
        
        let alt_expr = Hir {
            kind: HirKind::Alternation(vec![
                Hir { kind: HirKind::Alternation(vec![]) }, // Adding a nested empty alternation
                Hir { kind: HirKind::Alternation(vec![]) }, // Another nested empty alternation
            ]),
        };

        suffixes(&alt_expr, &mut lits);

        assert!(lits.is_empty(), "Literals should be empty for nested empty alternations");
    }

    #[test]
    fn test_suffixes_with_literals_in_alternation() {
        let mut lits = Literals::new();

        let alt_expr = Hir {
            kind: HirKind::Alternation(vec![
                Hir { kind: HirKind::Alternation(vec![]) }, // Adding a nested empty alternation
                Hir { kind: HirKind::Alternation(vec![]) }, // Another nested empty alternation
            ]),
        };

        suffixes(&alt_expr, &mut lits);

        assert!(lits.is_empty(), "Literals should remain empty for alternations with no literals");
    }
}

