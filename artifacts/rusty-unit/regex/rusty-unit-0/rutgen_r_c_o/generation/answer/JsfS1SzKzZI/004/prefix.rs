// Answer 0

#[test]
fn test_visit_class_basic() {
    let ast = ast::ClassBracketed { 
        span: Span::new(0, 10), 
        negated: false, 
        kind: ast::ClassSet::Item(ast::ClassSetItem::Bracketed(Box::new(ast::ClassBracketed {
            span: Span::new(0, 5), 
            negated: false, 
            kind: ast::ClassSet::Item(ast::ClassSetItem::Literal('a')) 
        }))) 
    };
    let mut visitor = MyVisitor::new();
    let mut heap_visitor = HeapVisitor::new();
    
    heap_visitor.visit_class(&ast, &mut visitor).unwrap();
}

#[test]
fn test_visit_class_with_union() {
    let ast = ast::ClassBracketed { 
        span: Span::new(0, 15), 
        negated: false, 
        kind: ast::ClassSet::Union(ast::ClassSetUnion { 
            items: vec![
                ast::ClassSetItem::Bracketed(Box::new(ast::ClassBracketed {
                    span: Span::new(0, 5), 
                    negated: false, 
                    kind: ast::ClassSet::Item(ast::ClassSetItem::Literal('b')) 
                })),
                ast::ClassSetItem::Bracketed(Box::new(ast::ClassBracketed {
                    span: Span::new(5, 10), 
                    negated: false, 
                    kind: ast::ClassSet::Item(ast::ClassSetItem::Literal('c'))
                })),
            ]
        }) 
    };
    let mut visitor = MyVisitor::new();
    let mut heap_visitor = HeapVisitor::new();

    heap_visitor.visit_class(&ast, &mut visitor).unwrap();
}

#[test]
fn test_visit_class_binary_operation() {
    let ast = ast::ClassBracketed {
        span: Span::new(0, 20),
        negated: false,
        kind: ast::ClassSet::BinaryOp(Box::new(ast::ClassSetBinaryOp {
            span: Span::new(0, 10),
            kind: ast::ClassSetBinaryOpKind::Union,
            lhs: Box::new(ast::ClassSet::Item(ast::ClassSetItem::Literal('a'))),
            rhs: Box::new(ast::ClassSet::Item(ast::ClassSetItem::Literal('d'))),
        })),
    };
    let mut visitor = MyVisitor::new();
    let mut heap_visitor = HeapVisitor::new();

    heap_visitor.visit_class(&ast, &mut visitor).unwrap();
}

#[test]
fn test_visit_class_empty() {
    let ast = ast::ClassBracketed {
        span: Span::new(0, 5),
        negated: false,
        kind: ast::ClassSet::Union(ast::ClassSetUnion { items: vec![] }),
    };
    let mut visitor = MyVisitor::new();
    let mut heap_visitor = HeapVisitor::new();

    heap_visitor.visit_class(&ast, &mut visitor).unwrap();
}

