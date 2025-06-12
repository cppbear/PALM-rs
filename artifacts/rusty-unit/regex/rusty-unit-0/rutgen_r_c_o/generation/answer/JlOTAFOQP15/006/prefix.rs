// Answer 0

#[test]
fn test_visit_with_repetition() {
    let mut visitor = MockVisitor::new();
    let mut hir = Hir::new(HirKind::Repetition(Repetition::new()));
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit(&mut hir, visitor).unwrap();
}

#[test]
fn test_visit_with_group() {
    let mut visitor = MockVisitor::new();
    let mut hir = Hir::new(HirKind::Group(Group::new()));
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit(&mut hir, visitor).unwrap();
}

#[test]
fn test_visit_with_concat() {
    let mut visitor = MockVisitor::new();
    let hirs = vec![
        Hir::new(HirKind::Repetition(Repetition::new())),
        Hir::new(HirKind::Group(Group::new()))
    ];
    let mut hir = Hir::new(HirKind::Concat(hirs.clone()));
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit(&mut hir, visitor).unwrap();
}

#[test]
fn test_visit_with_alternation() {
    let mut visitor = MockVisitor::new();
    let hirs = vec![
        Hir::new(HirKind::Repetition(Repetition::new())),
        Hir::new(HirKind::Group(Group::new()))
    ];
    let mut hir = Hir::new(HirKind::Alternation(hirs));
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit(&mut hir, visitor).unwrap();
}

#[test]
fn test_visit_with_nested_elements() {
    let mut visitor = MockVisitor::new();
    let hirs = vec![
        Hir::new(HirKind::Repetition(Repetition::new())),
        Hir::new(HirKind::Alternation(vec![Hir::new(HirKind::Group(Group::new()))])),
    ];
    let mut hir = Hir::new(HirKind::Concat(hirs.clone()));
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit(&mut hir, visitor).unwrap();
}

