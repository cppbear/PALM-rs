// Answer 0

#[test]
fn test_induct_repetition() {
    let hir = Hir::repetition(Repetition {
        kind: RepetitionKind::ZeroOrMore,
        greedy: true,
        hir: Box::new(Hir::literal(Literal::from('a'))),
    });
    let mut visitor = HeapVisitor::new();
    let frame = visitor.induct(&hir);
    assert!(frame.is_some());
}

#[test]
fn test_induct_group() {
    let hir = Hir::group(Group {
        kind: GroupKind::Capturing(1),
        hir: Box::new(Hir::literal(Literal::from('b'))),
    });
    let mut visitor = HeapVisitor::new();
    let frame = visitor.induct(&hir);
    assert!(frame.is_some());
}

#[test]
fn test_induct_concat_non_empty() {
    let hir = Hir::concat(vec![
        Hir::literal(Literal::from('c')),
        Hir::literal(Literal::from('d')),
    ]);
    let mut visitor = HeapVisitor::new();
    let frame = visitor.induct(&hir);
    assert!(frame.is_some());
}

#[test]
fn test_induct_concat_empty() {
    let hir = Hir::concat(vec![]);
    let mut visitor = HeapVisitor::new();
    let frame = visitor.induct(&hir);
    assert!(frame.is_none());
}

#[test]
fn test_induct_alternation_non_empty() {
    let hir = Hir::alternation(vec![
        Hir::literal(Literal::from('e')),
        Hir::literal(Literal::from('f')),
    ]);
    let mut visitor = HeapVisitor::new();
    let frame = visitor.induct(&hir);
    assert!(frame.is_some());
}

#[test]
fn test_induct_alternation_empty() {
    let hir = Hir::alternation(vec![]);
    let mut visitor = HeapVisitor::new();
    let frame = visitor.induct(&hir);
    assert!(frame.is_none());
}

#[test]
fn test_induct_empty() {
    let hir = Hir::empty();
    let mut visitor = HeapVisitor::new();
    let frame = visitor.induct(&hir);
    assert!(frame.is_none());
}

