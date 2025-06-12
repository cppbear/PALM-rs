// Answer 0

#[test]
fn test_induct_concat_non_empty() {
    struct TestVisitor;
    
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
    }

    let head_hir = Hir::literal(Literal::from('a'));
    let tail_hir = Hir::literal(Literal::from('b'));

    let mut concat_hir = Hir::concat(vec![head_hir.clone(), tail_hir.clone()]);

    let mut visitor = TestVisitor;
    let mut heap_visitor = HeapVisitor::new();

    if let Some(frame) = heap_visitor.induct(&concat_hir) {
        match frame {
            Frame::Concat { head, tail } => {
                assert_eq!(head, &head_hir);
                assert_eq!(tail, &[tail_hir]);
            }
            _ => panic!("Expected a Concat frame."),
        }
    } else {
        panic!("Expected a frame to be returned.");
    }
}

#[test]
fn test_induct_concat_empty() {
    struct TestVisitor;
    
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
    }

    let concat_hir = Hir::concat(vec![]);

    let mut heap_visitor = HeapVisitor::new();

    assert!(heap_visitor.induct(&concat_hir).is_none());
}

#[test]
fn test_induct_alternation_non_empty() {
    struct TestVisitor;
    
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
    }

    let head_hir = Hir::literal(Literal::from('a'));
    let tail_hir = Hir::literal(Literal::from('b'));

    let mut alternation_hir = Hir::alternation(vec![head_hir.clone(), tail_hir.clone()]);

    let mut visitor = TestVisitor;
    let mut heap_visitor = HeapVisitor::new();

    if let Some(frame) = heap_visitor.induct(&alternation_hir) {
        match frame {
            Frame::Alternation { head, tail } => {
                assert_eq!(head, &head_hir);
                assert_eq!(tail, &[tail_hir]);
            }
            _ => panic!("Expected an Alternation frame."),
        }
    } else {
        panic!("Expected a frame to be returned.");
    }
}

#[test]
fn test_induct_alternation_empty() {
    struct TestVisitor;
    
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
    }

    let alternation_hir = Hir::alternation(vec![]);

    let mut heap_visitor = HeapVisitor::new();

    assert!(heap_visitor.induct(&alternation_hir).is_none());
}

