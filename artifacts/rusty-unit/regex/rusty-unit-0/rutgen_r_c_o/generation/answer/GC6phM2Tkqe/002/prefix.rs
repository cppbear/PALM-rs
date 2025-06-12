// Answer 0

#[test]
fn test_induct_alternation_empty() {
    let mut visitor = HeapVisitor::new();
    let hir = Hir::alternation(vec![]);
    let _ = visitor.induct(&hir);
}

#[test]
fn test_induct_alternation_single_element() {
    let mut visitor = HeapVisitor::new();
    let inner_hir = Hir::literal(Literal::from('a'));
    let hir = Hir::alternation(vec![inner_hir]);
    let _ = visitor.induct(&hir);
}

#[test]
fn test_induct_concat_empty() {
    let mut visitor = HeapVisitor::new();
    let hir = Hir::concat(vec![]);
    let _ = visitor.induct(&hir);
}

#[test]
fn test_induct_group() {
    let mut visitor = HeapVisitor::new();
    let inner_hir = Hir::literal(Literal::from('b'));
    let group = Group { kind: GroupKind::Capturing(0), hir: Box::new(inner_hir) };
    let hir = Hir::group(group);
    let _ = visitor.induct(&hir);
}

#[test]
fn test_induct_repetition() {
    let mut visitor = HeapVisitor::new();
    let inner_hir = Hir::literal(Literal::from('c'));
    let repetition = Repetition { kind: RepetitionKind::ZeroOrMore, greedy: true, hir: Box::new(inner_hir) };
    let hir = Hir::repetition(repetition);
    let _ = visitor.induct(&hir);
}

