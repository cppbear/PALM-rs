// Answer 0

#[test]
fn test_into_kind_non_empty() {
    struct TestHir {
        kind: regex_syntax::hir::HirKind,
    }

    impl TestHir {
        pub fn new(kind: regex_syntax::hir::HirKind) -> Self {
            TestHir { kind }
        }

        pub fn into_kind(self) -> regex_syntax::hir::HirKind {
            use std::mem;
            mem::replace(&mut self.kind, regex_syntax::hir::HirKind::Empty)
        }
    }

    let hir = TestHir::new(regex_syntax::hir::HirKind::Literal("test".into()));
    let kind = hir.into_kind();
    assert_eq!(kind, regex_syntax::hir::HirKind::Literal("test".into()));
}

#[test]
fn test_into_kind_empty() {
    struct TestHir {
        kind: regex_syntax::hir::HirKind,
    }

    impl TestHir {
        pub fn new(kind: regex_syntax::hir::HirKind) -> Self {
            TestHir { kind }
        }

        pub fn into_kind(self) -> regex_syntax::hir::HirKind {
            use std::mem;
            mem::replace(&mut self.kind, regex_syntax::hir::HirKind::Empty)
        }
    }

    let hir = TestHir::new(regex_syntax::hir::HirKind::Empty);
    let kind = hir.into_kind();
    assert_eq!(kind, regex_syntax::hir::HirKind::Empty);
}

