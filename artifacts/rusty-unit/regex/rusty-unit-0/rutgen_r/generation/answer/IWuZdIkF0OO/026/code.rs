// Answer 0

#[test]
fn test_suffixes_unicode_literal() {
    struct MockHir {
        kind: HirKind,
    }

    struct MockLiterals {
        data: Vec<u8>,
    }

    impl MockLiterals {
        fn new() -> Self {
            Self { data: vec![] }
        }

        fn cross_add(&mut self, buf: &[u8]) {
            self.data.extend_from_slice(buf);
        }

        fn cut(&mut self) {
            self.data.clear();
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }
    }

    let unicode_literal = hir::Literal::Unicode('𝟘'); // Using a Unicode character
    let expr = MockHir { kind: HirKind::Literal(unicode_literal) };
    let mut lits = MockLiterals::new();

    suffixes(&expr, &mut lits);

    assert!(!lits.is_empty());
}

#[test]
fn test_suffixes_byte_literal() {
    struct MockHir {
        kind: HirKind,
    }

    struct MockLiterals {
        data: Vec<u8>,
    }

    impl MockLiterals {
        fn new() -> Self {
            Self { data: vec![] }
        }

        fn cross_add(&mut self, buf: &[u8]) {
            self.data.extend_from_slice(buf);
        }

        fn cut(&mut self) {
            self.data.clear();
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }
    }

    let byte_literal = hir::Literal::Byte(0xFF); // Using a sample byte
    let expr = MockHir { kind: HirKind::Literal(byte_literal) };
    let mut lits = MockLiterals::new();

    suffixes(&expr, &mut lits);

    assert!(!lits.is_empty());
}

#[test]
fn test_suffixes_empty_concat() {
    struct MockHir {
        kind: HirKind,
    }

    struct MockLiterals {
        data: Vec<u8>,
    }

    impl MockLiterals {
        fn new() -> Self {
            Self { data: vec![] }
        }

        fn cut(&mut self) {
            self.data.clear();
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }
    }

    let expr = MockHir { kind: HirKind::Concat(vec![]) }; // Empty concat
    let mut lits = MockLiterals::new();

    suffixes(&expr, &mut lits);

    assert!(lits.is_empty());
}

#[test]
fn test_suffixes_single_concat() {
    struct MockHir {
        kind: HirKind,
    }

    struct MockLiterals {
        data: Vec<u8>,
    }

    impl MockLiterals {
        fn new() -> Self {
            Self { data: vec![] }
        }

        fn cross_add(&mut self, buf: &[u8]) {
            self.data.extend_from_slice(buf);
        }

        fn cut(&mut self) {
            self.data.clear();
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }
    }

    let byte_literal = hir::Literal::Byte(0x41); // 'A' in ASCII
    let single_expr = MockHir { kind: HirKind::Literal(byte_literal) };
    let concat_expr = MockHir { kind: HirKind::Concat(vec![single_expr]) };
    let mut lits = MockLiterals::new();

    suffixes(&concat_expr, &mut lits);

    assert!(!lits.is_empty());
}

