// Answer 0

#[test]
fn test_suffixes_unicode_literal() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn new(kind: HirKind) -> Self {
            TestHir { kind }
        }
        
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    struct TestLiterals {
        data: Vec<u8>,
    }

    impl TestLiterals {
        fn new() -> Self {
            TestLiterals { data: Vec::new() }
        }

        fn cross_add(&mut self, suffix: &[u8]) {
            self.data.extend_from_slice(suffix);
        }

        fn cut(&mut self) {
            self.data.clear();
        }
    }

    let unicode_literal = TestHir::new(HirKind::Literal(hir::Literal::Unicode('a')));
    let mut literals = TestLiterals::new();
    suffixes(&unicode_literal, &mut literals);
    assert!(!literals.data.is_empty());
}

#[test]
fn test_suffixes_byte_literal() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn new(kind: HirKind) -> Self {
            TestHir { kind }
        }
        
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    struct TestLiterals {
        data: Vec<u8>,
    }

    impl TestLiterals {
        fn new() -> Self {
            TestLiterals { data: Vec::new() }
        }

        fn cross_add(&mut self, suffix: &[u8]) {
            self.data.extend_from_slice(suffix);
        }

        fn cut(&mut self) {
            self.data.clear();
        }
    }

    let byte_literal = TestHir::new(HirKind::Literal(hir::Literal::Byte(b'a')));
    let mut literals = TestLiterals::new();
    suffixes(&byte_literal, &mut literals);
    assert_eq!(literals.data, vec![b'a']);
}

#[test]
fn test_suffixes_empty_concatenation() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn new(kind: HirKind) -> Self {
            TestHir { kind }
        }
        
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    struct TestLiterals {
        data: Vec<u8>,
    }

    impl TestLiterals {
        fn new() -> Self {
            TestLiterals { data: Vec::new() }
        }

        fn cut(&mut self) {
            self.data.clear();
        }
    }

    let empty_concat = TestHir::new(HirKind::Concat(Vec::new()));
    let mut literals = TestLiterals::new();
    suffixes(&empty_concat, &mut literals);
    assert!(literals.data.is_empty());
}

#[test]
fn test_suffixes_one_element_concatenation() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn new(kind: HirKind) -> Self {
            TestHir { kind }
        }
        
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    struct TestLiterals {
        data: Vec<u8>,
    }

    impl TestLiterals {
        fn new() -> Self {
            TestLiterals { data: Vec::new() }
        }

        fn cross_add(&mut self, suffix: &[u8]) {
            self.data.extend_from_slice(suffix);
        }

        fn cut(&mut self) {
            self.data.clear();
        }
    }

    let single_literal_concat = TestHir::new(HirKind::Concat(vec![HirKind::Literal(hir::Literal::Byte(b'b'))]));
    let mut literals = TestLiterals::new();
    suffixes(&single_literal_concat, &mut literals);
    assert_eq!(literals.data, vec![b'b']);
}

