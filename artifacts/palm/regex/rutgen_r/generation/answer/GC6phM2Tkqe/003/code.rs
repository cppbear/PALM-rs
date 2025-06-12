// Answer 0

#[test]
fn test_induct_alternation_non_empty() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    enum HirKind {
        Alternation(Vec<TestHir>),
        // other variants omitted for simplicity
    }

    struct Frame<'a> {
        alternation: &'a [TestHir],
    }

    let child1 = TestHir { kind: HirKind::Alternation(Vec::new()) };
    let child2 = TestHir { kind: HirKind::Alternation(Vec::new()) };
    
    let hir = TestHir {
        kind: HirKind::Alternation(vec![child1, child2]),
    };

    let mut stack_frame = induct(&mut hir);

    assert!(stack_frame.is_some());
    if let Some(Frame::Alternation { head, tail }) = stack_frame {
        assert_eq!(head, &hir.kind().alternation()[0]);
        assert_eq!(tail, &hir.kind().alternation()[1..]);
    }
}

#[test]
fn test_induct_alternation_empty() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    enum HirKind {
        Alternation(Vec<TestHir>),
        // other variants omitted for simplicity
    }

    let hir = TestHir {
        kind: HirKind::Alternation(vec![]),
    };

    let mut stack_frame = induct(&mut hir);

    assert!(stack_frame.is_none());
}

