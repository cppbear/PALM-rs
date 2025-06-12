// Answer 0

#[test]
fn test_induct_group() {
    struct DummyVisitor;
    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let group_hir = Group {
        kind: GroupKind::Capturing { index: 0, name: None },
        hir: Box::new(Hir::empty()),
    };

    let hir = Hir {
        kind: HirKind::Group(group_hir),
        info: HirInfo::default(),
    };

    let mut visitor = HeapVisitor::new();
    let frame = visitor.induct(&hir);

    match frame {
        Some(Frame::Group(ref x)) => {
            assert_eq!(x, &group_hir);
        }
        _ => panic!("Expected Some(Frame::Group), but got {:?}", frame),
    }
}

#[test]
fn test_induct_repetition() {
    struct DummyVisitor;
    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let repetition_hir = Repetition {
        kind: RepetitionKind::Plus,
        greedy: true,
        hir: Box::new(Hir::literal(Literal::from_char('a'))),
    };

    let hir = Hir {
        kind: HirKind::Repetition(repetition_hir),
        info: HirInfo::default(),
    };

    let mut visitor = HeapVisitor::new();
    let frame = visitor.induct(&hir);

    match frame {
        Some(Frame::Repetition(ref x)) => {
            assert_eq!(x, &repetition_hir);
        }
        _ => panic!("Expected Some(Frame::Repetition), but got {:?}", frame),
    }
}

#[test]
fn test_induct_concat() {
    struct DummyVisitor;
    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let concat_hir = Hir::concat(vec![
        Hir::literal(Literal::from_char('a')),
        Hir::literal(Literal::from_char('b')),
    ]);

    let hir = Hir {
        kind: HirKind::Concat(vec![concat_hir.clone(), concat_hir.clone()]),
        info: HirInfo::default(),
    };

    let mut visitor = HeapVisitor::new();
    let frame = visitor.induct(&hir);

    match frame {
        Some(Frame::Concat { head, tail }) => {
            assert_eq!(head, &concat_hir);
            assert_eq!(tail.len(), 1); // since we have two elements in the concat
        }
        _ => panic!("Expected Some(Frame::Concat), but got {:?}", frame),
    }
}

#[test]
fn test_induct_empty_concat() {
    struct DummyVisitor;
    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let hir = Hir {
        kind: HirKind::Concat(vec![]),
        info: HirInfo::default(),
    };

    let mut visitor = HeapVisitor::new();
    let frame = visitor.induct(&hir);

    assert!(frame.is_none(), "Expected None for empty concat");
}

#[test]
fn test_induct_empty_alternation() {
    struct DummyVisitor;
    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let hir = Hir {
        kind: HirKind::Alternation(vec![]),
        info: HirInfo::default(),
    };

    let mut visitor = HeapVisitor::new();
    let frame = visitor.induct(&hir);

    assert!(frame.is_none(), "Expected None for empty alternation");
}

