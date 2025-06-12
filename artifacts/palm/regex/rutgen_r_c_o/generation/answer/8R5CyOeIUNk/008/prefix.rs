// Answer 0

#[test]
fn test_induct_with_empty_ast() {
    let span = Span {}; 
    let ast = Ast::Empty(span);
    let mut visitor = MockVisitor {};
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_with_bracketed_class() {
    let span = Span {};
    let class_set = ClassSet {}; 
    let class_bracketed = ClassBracketed {
        span,
        negated: false,
        kind: class_set,
    };
    let ast = Ast::Class(Class::Bracketed(class_bracketed));
    let mut visitor = MockVisitor {};
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_with_literal() {
    let span = Span {};
    let literal = Literal {}; 
    let ast = Ast::Literal(literal);
    let mut visitor = MockVisitor {};
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.induct(&ast, &mut visitor);
}

