// Answer 0

#[test]
fn test_prefixes_unicode_literal() {
    // Setup
    struct MockLiterals {
        data: Vec<u8>,
    }

    impl MockLiterals {
        fn new() -> Self {
            MockLiterals { data: Vec::new() }
        }

        fn cross_add(&mut self, bytes: &[u8]) {
            self.data.extend_from_slice(bytes);
        }

        fn cut(&mut self) {
            self.data.clear();
        }

        fn add(&mut self, _literal: &Literals) {
            // Mock implementation
        }

        fn to_empty(&self) -> Self {
            MockLiterals::new()
        }

        fn cross_product(&mut self, _other: &Self) -> bool {
            true
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        fn any_complete(&self) -> bool {
            true
        }
    }

    let c = 'a';
    let expr = Hir::new(HirKind::Literal(hir::Literal::Unicode(c)));
    let mut lits = MockLiterals::new();

    // Act
    prefixes(&expr, &mut lits);

    // Assert
    assert!(!lits.is_empty());
}

#[test]
fn test_prefixes_byte_literal() {
    // Setup
    struct MockLiterals {
        data: Vec<u8>,
    }

    impl MockLiterals {
        fn new() -> Self {
            MockLiterals { data: Vec::new() }
        }

        fn cross_add(&mut self, bytes: &[u8]) {
            self.data.extend_from_slice(bytes);
        }

        fn cut(&mut self) {
            self.data.clear();
        }

        fn add(&mut self, _literal: &Literals) {
            // Mock implementation
        }

        fn to_empty(&self) -> Self {
            MockLiterals::new()
        }

        fn cross_product(&mut self, _other: &Self) -> bool {
            true
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        fn any_complete(&self) -> bool {
            true
        }
    }

    let b = b'a';
    let expr = Hir::new(HirKind::Literal(hir::Literal::Byte(b)));
    let mut lits = MockLiterals::new();

    // Act
    prefixes(&expr, &mut lits);

    // Assert
    assert!(!lits.is_empty());
}

