// Answer 0

#[test]
fn test_suffixes_literal_unicode() {
    struct MockLiteral;

    impl MockLiteral {
        fn new() -> Self {
            MockLiteral
        }
    }

    struct MockClass;

    impl MockClass {
        fn new() -> Self {
            MockClass
        }
    }

    struct MockGroup;

    impl MockGroup {
        fn new() -> Self {
            MockGroup
        }
    }

    struct MockRepetition;

    impl MockRepetition {
        fn new() -> Self {
            MockRepetition
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

    struct MockLiterals;

    impl MockLiterals {
        fn new() -> Self {
            MockLiterals
        }

        fn cross_add(&mut self, _buf: &[u8]) {}

        fn add_char_class_reverse(&mut self, _cls: &MockClass) -> bool {
            false
        }

        fn cut(&mut self) {}

        fn to_empty(&mut self) -> Self {
            MockLiterals::new()
        }

        fn cross_product(&mut self, _other: &Self) -> bool {
            false
        }

        fn any_complete(&self) -> bool {
            false
        }

        fn is_empty(&self) -> bool {
            false
        }

        fn add(&mut self, _literal: MockLiteral) {}
    }

    let mut lits = MockLiterals::new();
    let expr = MockHir::new(HirKind::Concat(vec![
        MockHir::new(HirKind::Anchor(hir::Anchor::EndText)),
    ]));

    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_class_unicode() {
    let mut lits = MockLiterals::new();
    let expr = MockHir::new(HirKind::Class(hir::Class::Unicode(MockClass::new())));

    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_group() {
    struct InnerHir;

    impl InnerHir {
        fn new() -> Self {
            InnerHir
        }
    }

    let mut lits = MockLiterals::new();
    let expr = MockHir::new(HirKind::Group {
        hir: Box::new(MockHir::new(HirKind::Literal(hir::Literal::Unicode('a')))),
    });

    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_empty_concat() {
    let mut lits = MockLiterals::new();
    let expr = MockHir::new(HirKind::Concat(vec![]));

    suffixes(&expr, &mut lits);
}

#[test]
fn test_suffixes_concat_single() {
    let mut lits = MockLiterals::new();
    let expr = MockHir::new(HirKind::Concat(vec![
        MockHir::new(HirKind::Literal(hir::Literal::Byte(1))),
    ]));

    suffixes(&expr, &mut lits);
}

