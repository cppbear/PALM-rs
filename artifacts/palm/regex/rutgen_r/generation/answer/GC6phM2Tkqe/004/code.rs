// Answer 0

#[test]
fn test_induct_concat_empty() {
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

    enum HirKind {
        Concat(Vec<TestHir>),
    }

    let empty_concat_hir = TestHir::new(HirKind::Concat(vec![]));
    
    let result = induct(&mut empty_concat_hir);
    assert_eq!(result, None);
}

#[test]
fn test_induct_alternation_empty() {
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

    enum HirKind {
        Alternation(Vec<TestHir>),
    }

    let empty_alternation_hir = TestHir::new(HirKind::Alternation(vec![]));
    
    let result = induct(&mut empty_alternation_hir);
    assert_eq!(result, None);
}

#[test]
fn test_induct_concat_non_empty() {
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

    enum HirKind {
        Concat(Vec<TestHir>),
    }

    let child_hir = TestHir::new(HirKind::Concat(vec![]));
    let non_empty_concat_hir = TestHir::new(HirKind::Concat(vec![child_hir]));
    
    let result = induct(&mut non_empty_concat_hir);
    assert!(result.is_some());
}

