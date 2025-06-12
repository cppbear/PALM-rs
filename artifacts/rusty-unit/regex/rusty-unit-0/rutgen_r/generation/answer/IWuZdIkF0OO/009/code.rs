// Answer 0

fn test_suffixes_literal_unicode() {
    struct MockLiterals {
        added: Vec<u8>,
        is_empty: bool,
    }

    impl MockLiterals {
        fn new() -> Self {
            MockLiterals {
                added: Vec::new(),
                is_empty: true,
            }
        }
        
        fn cross_add(&mut self, buf: &[u8]) {
            self.added.extend(buf.iter());
            self.is_empty = false;
        }
        
        fn cut(&mut self) {
            self.added.clear();
            self.is_empty = true;
        }
        
        fn cross_product(&self, other: &MockLiterals) -> bool {
            !self.added.is_empty() && !other.added.is_empty()
        }
        
        fn to_empty(&self) -> MockLiterals {
            MockLiterals::new()
        }
        
        fn any_complete(&self) -> bool {
            false
        }
    }

    struct MockHir {
        kind: HirKind,
    }

    impl MockHir {
        fn new(kind: HirKind) -> Self {
            MockHir { kind }
        }
        
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    enum HirKind {
        Literal(hir::Literal),
        Concat(Vec<MockHir>),
        Anchor(hir::Anchor),
        // ... other variants as needed
    }

    mod hir {
        pub enum Literal {
            Unicode(char),
            Byte(u8),
        }

        pub enum Anchor {
            EndText,
        }
    }

    let mut lits = MockLiterals::new();
    let expr = MockHir::new(HirKind::Concat(vec![
        MockHir::new(HirKind::Anchor(hir::Anchor::EndText)),
    ]));

    suffixes(&expr, &mut lits);

    assert!(!lits.added.is_empty());
}

fn test_suffixes_literal_byte() {
    struct MockLiterals {
        added: Vec<u8>,
    }

    impl MockLiterals {
        fn new() -> Self {
            MockLiterals { added: Vec::new() }
        }

        fn cross_add(&mut self, buf: &[u8]) {
            self.added.extend(buf.iter());
        }

        fn cut(&mut self) {
            self.added.clear();
        }

        fn cross_product(&self, other: &MockLiterals) -> bool {
            !self.added.is_empty() && !other.added.is_empty()
        }

        fn to_empty(&self) -> MockLiterals {
            MockLiterals::new()
        }

        fn any_complete(&self) -> bool {
            false
        }
    }

    struct MockHir {
        kind: HirKind,
    }

    impl MockHir {
        fn new(kind: HirKind) -> Self {
            MockHir { kind }
        }

        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    enum HirKind {
        Literal(hir::Literal),
        Concat(Vec<MockHir>),
        // ... other variants as needed
    }

    mod hir {
        pub enum Literal {
            Byte(u8),
        }
    }

    let mut lits = MockLiterals::new();
    let expr = MockHir::new(HirKind::Concat(vec![
        MockHir::new(HirKind::Literal(hir::Literal::Byte(b'a'))),
    ]));

    suffixes(&expr, &mut lits);

    assert_eq!(lits.added, vec![b'a']);
}

fn test_suffixes_empty_concat() {
    struct MockLiterals {
        added: Vec<u8>,
    }

    impl MockLiterals {
        fn new() -> Self {
            MockLiterals { added: Vec::new() }
        }

        fn cross_add(&mut self, buf: &[u8]) {
            self.added.extend(buf.iter());
        }

        fn cut(&mut self) {
            self.added.clear();
        }

        fn cross_product(&self, other: &MockLiterals) -> bool {
            !self.added.is_empty() && !other.added.is_empty()
        }

        fn to_empty(&self) -> MockLiterals {
            MockLiterals::new()
        }

        fn any_complete(&self) -> bool {
            false
        }
    }

    struct MockHir {
        kind: HirKind,
    }

    impl MockHir {
        fn new(kind: HirKind) -> Self {
            MockHir { kind }
        }

        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    enum HirKind {
        Concat(Vec<MockHir>),
        // ... other variants as needed
    }

    let mut lits = MockLiterals::new();
    let expr = MockHir::new(HirKind::Concat(vec![]));

    suffixes(&expr, &mut lits);

    assert!(lits.added.is_empty());
}

