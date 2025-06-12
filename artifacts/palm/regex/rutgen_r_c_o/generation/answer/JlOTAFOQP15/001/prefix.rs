// Answer 0

#[test]
fn test_visit_valid_hir_concat() {
    let mut visitor = MockVisitor::new();
    let valid_hir = Hir::new(HirKind::Concat(vec![Hir::new(HirKind::Literal("a")), Hir::new(HirKind::Literal("b"))]));

    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&valid_hir, visitor);
}

#[test]
fn test_visit_invalid_hir_empty_concat() {
    let mut visitor = MockVisitor::new();
    let invalid_hir = Hir::new(HirKind::Concat(vec![]));

    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.visit(&invalid_hir, visitor);
}

#[test]
fn test_visit_valid_hir_repetition() {
    let mut visitor = MockVisitor::new();
    let valid_hir = Hir::new(HirKind::Repetition(hir::Repetition::new(Hir::new(HirKind::Literal("a")))));

    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&valid_hir, visitor);
}

#[test]
fn test_visit_invalid_hir_empty_alternation() {
    let mut visitor = MockVisitor::new();
    let invalid_hir = Hir::new(HirKind::Alternation(vec![]));

    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.visit(&invalid_hir, visitor);
}

#[test]
fn test_visit_valid_hir_group() {
    let mut visitor = MockVisitor::new();
    let valid_hir = Hir::new(HirKind::Group(hir::Group::new(Hir::new(HirKind::Literal("a")))));

    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&valid_hir, visitor);
}

#[test]
fn test_visit_complex_hir() {
    let mut visitor = MockVisitor::new();
    let complex_hir = Hir::new(HirKind::Concat(vec![
        Hir::new(HirKind::Repetition(hir::Repetition::new(Hir::new(HirKind::Literal("a"))))),
        Hir::new(HirKind::Group(hir::Group::new(Hir::new(HirKind::Literal("b"))))),
        Hir::new(HirKind::Alternation(vec![
            Hir::new(HirKind::Literal("c")),
            Hir::new(HirKind::Literal("d")),
        ])),
    ]));

    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&complex_hir, visitor);
}

#[derive(Default)]
struct MockVisitor {
    output: i32,
}

impl MockVisitor {
    fn new() -> Self {
        Self { output: 0 }
    }
}

impl Visitor for MockVisitor {
    type Output = i32;
    type Err = ();

    fn start(&mut self) {}

    fn visit_pre(&mut self, _hir: &Hir) -> Result<(), Self::Err> {
        Ok(())
    }

    fn visit_post(&mut self, _hir: &Hir) -> Result<(), Self::Err> {
        Ok(())
    }

    fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
        Ok(())
    }

    fn finish(self) -> Result<Self::Output, Self::Err> {
        Ok(self.output)
    }
}

