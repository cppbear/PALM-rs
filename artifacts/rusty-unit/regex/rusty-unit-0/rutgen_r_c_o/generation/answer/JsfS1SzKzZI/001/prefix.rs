// Answer 0

#[test]
fn test_visit_class_with_empty_class() {
    let mut visitor = TestVisitor::new();
    let ast = ast::ClassBracketed {
        span: Span::new(0, 1),
        negated: false,
        kind: ast::ClassSet::Item(Box::new(ast::ClassSetItem::Empty)),
    };
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit_class(&ast, &mut visitor);
}

#[test]
fn test_visit_class_with_single_item() {
    let mut visitor = TestVisitor::new();
    let ast = ast::ClassBracketed {
        span: Span::new(0, 3),
        negated: false,
        kind: ast::ClassSet::Item(Box::new(ast::ClassSetItem::Single('a'))),
    };
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit_class(&ast, &mut visitor);
}

#[test]
fn test_visit_class_with_union() {
    let mut visitor = TestVisitor::new();
    let item1 = ast::ClassSetItem::Single('a');
    let item2 = ast::ClassSetItem::Single('b');
    let ast = ast::ClassBracketed {
        span: Span::new(0, 5),
        negated: false,
        kind: ast::ClassSet::Union(vec![Box::new(item1), Box::new(item2)]),
    };
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit_class(&ast, &mut visitor);
}

#[test]
fn test_visit_class_with_binary_operation() {
    let mut visitor = TestVisitor::new();
    let lhs = ast::ClassSet::Item(Box::new(ast::ClassSetItem::Single('a')));
    let rhs = ast::ClassSet::Item(Box::new(ast::ClassSetItem::Single('b')));
    let ast = ast::ClassBracketed {
        span: Span::new(0, 6),
        negated: false,
        kind: ast::ClassSet::BinaryOp(Box::new(ast::ClassSetBinaryOp {
            span: Span::new(0, 6),
            kind: ast::ClassSetBinaryOpKind::Union,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })),
    };
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit_class(&ast, &mut visitor);
}

#[test]
fn test_visit_class_with_negated_class() {
    let mut visitor = TestVisitor::new();
    let ast = ast::ClassBracketed {
        span: Span::new(0, 4),
        negated: true,
        kind: ast::ClassSet::Item(Box::new(ast::ClassSetItem::Single('a'))),
    };
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit_class(&ast, &mut visitor);
}

#[test]
fn test_visit_class_with_large_stack_class() {
    let mut visitor = TestVisitor::new();
    let items: Vec<_> = (1..=1000).map(|i| ast::ClassSetItem::Single(char::from(i))) 
                                     .collect();
    let ast = ast::ClassBracketed {
        span: Span::new(0, 1000),
        negated: false,
        kind: ast::ClassSet::Union(items.into_iter().map(Box::new).collect()),
    };
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit_class(&ast, &mut visitor);
}

