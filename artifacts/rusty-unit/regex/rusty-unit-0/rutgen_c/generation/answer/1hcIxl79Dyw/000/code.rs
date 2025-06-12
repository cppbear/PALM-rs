// Answer 0

#[test]
fn test_heap_visitor_new() {
    struct TestHir {
        kind: hir::HirKind,
        info: hir::HirInfo,
    }
    
    let visitor: HeapVisitor<()> = HeapVisitor::new();
    
    assert_eq!(visitor.stack.len(), 0);
}

#[test]
fn test_heap_visitor_new_non_empty() {
    struct TestHir {
        kind: hir::HirKind,
        info: hir::HirInfo,
    }
    
    let mut visitor: HeapVisitor<()> = HeapVisitor::new();
    visitor.stack.push((&TestHir { kind: hir::HirKind::Empty, info: hir::HirInfo::default() }, Frame::Concat { head: &TestHir { kind: hir::HirKind::Empty, info: hir::HirInfo::default() }, tail: &[] }));
    
    assert_eq!(visitor.stack.len(), 1);
}

