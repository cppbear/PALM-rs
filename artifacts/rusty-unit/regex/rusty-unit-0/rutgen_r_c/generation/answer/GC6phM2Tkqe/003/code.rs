// Answer 0

#[test]
fn test_induct_with_non_empty_alternation() {
    struct DummyVisitor;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let head_hir = Hir::literal('a');
    let tail_hir = Hir::literal('b');
    let mut alternation_hir = Hir::alternation(vec![head_hir.clone(), tail_hir.clone()]);

    let mut visitor = HeapVisitor::new();
    let frame = visitor.induct(&alternation_hir).unwrap();

    match frame {
        Frame::Alternation { head, tail } => {
            assert_eq!(head, &head_hir);
            assert_eq!(tail, &[tail_hir]);
        }
        _ => panic!("Expected an Alternation frame"),
    }
}

#[test]
fn test_induct_with_non_empty_concat() {
    struct DummyVisitor;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let first_hir = Hir::literal('x');
    let second_hir = Hir::literal('y');
    let mut concat_hir = Hir::concat(vec![first_hir.clone(), second_hir.clone()]);

    let mut visitor = HeapVisitor::new();
    let frame = visitor.induct(&concat_hir).unwrap();

    match frame {
        Frame::Concat { head, tail } => {
            assert_eq!(head, &first_hir);
            assert_eq!(tail, &[second_hir]);
        }
        _ => panic!("Expected a Concat frame"),
    }
}

#[test]
fn test_induct_with_empty_concat() {
    struct DummyVisitor;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let mut concat_hir = Hir::concat(vec![]);

    let mut visitor = HeapVisitor::new();
    let frame = visitor.induct(&concat_hir);

    assert!(frame.is_none());
}

#[test]
fn test_induct_with_empty_alternation() {
    struct DummyVisitor;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let mut alternation_hir = Hir::alternation(vec![]);

    let mut visitor = HeapVisitor::new();
    let frame = visitor.induct(&alternation_hir);

    assert!(frame.is_none());
}

