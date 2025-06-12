// Answer 0

#[test]
fn test_prefixes_with_alternation() {
    struct DummyLiterals {
        data: Vec<u8>,
    }

    impl DummyLiterals {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn cross_add(&mut self, bytes: &[u8]) {
            self.data.extend_from_slice(bytes);
        }

        fn add(&mut self, literal: &Hir) {
            // Simplified: Just push a byte representation of the literal for testing
            self.data.push(0);
        }

        fn cut(&mut self) {
            self.data.clear();
        }

        fn to_empty(&self) -> Self {
            DummyLiterals::new()
        }

        fn cross_product(&mut self, other: &Self) -> bool {
            self.data.extend_from_slice(&other.data);
            true
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        fn any_complete(&self) -> bool {
            !self.data.is_empty()
        }
    }

    enum HirKind {
        Alternation(Vec<Box<Hir>>),
        Literal(Literal),
    }

    struct Hir {
        kind: HirKind,
    }

    impl Hir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    enum Literal {
        Unicode(char),
        Byte(u8),
        Empty,
    }

    let mut lits = DummyLiterals::new();

    let expr = Hir {
        kind: HirKind::Alternation(vec![
            Box::new(Hir { kind: HirKind::Literal(Literal::Unicode('a')) }),
            Box::new(Hir { kind: HirKind::Literal(Literal::Unicode('b')) }),
            Box::new(Hir { kind: HirKind::Literal(Literal::Byte(0x2b)) }),
        ]),
    };

    prefixes(&expr, &mut lits);

    assert!(!lits.is_empty());
}

#[test]
fn test_prefixes_with_empty_alternation() {
    struct DummyLiterals {
        data: Vec<u8>,
    }

    impl DummyLiterals {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn cross_add(&mut self, bytes: &[u8]) {
            self.data.extend_from_slice(bytes);
        }

        fn add(&mut self, literal: &Hir) {
            self.data.push(0);
        }

        fn cut(&mut self) {
            self.data.clear();
        }

        fn to_empty(&self) -> Self {
            DummyLiterals::new()
        }

        fn cross_product(&mut self, other: &Self) -> bool {
            self.data.extend_from_slice(&other.data);
            true
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        fn any_complete(&self) -> bool {
            !self.data.is_empty()
        }
    }

    let mut lits = DummyLiterals::new();

    let expr = Hir {
        kind: HirKind::Alternation(vec![]),
    };

    prefixes(&expr, &mut lits);

    assert!(lits.is_empty());
}

