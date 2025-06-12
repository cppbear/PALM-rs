// Answer 0

#[test]
fn test_visit_with_literal() {
    let visitor = MyVisitor::new();
    let ast = Ast::Literal(Literal::new('a'));
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_with_repetition() {
    let visitor = MyVisitor::new();
    let ast = Ast::Repetition(Repetition::new(1, 5, Box::new(Ast::Literal(Literal::new('a')))));
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_with_group() {
    let visitor = MyVisitor::new();
    let ast = Ast::Group(Group::new(vec![
        Ast::Literal(Literal::new('a')),
        Ast::Literal(Literal::new('b')),
    ]));
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_with_concat() {
    let visitor = MyVisitor::new();
    let ast = Ast::Concat(Concat::new(vec![
        Ast::Literal(Literal::new('a')),
        Ast::Repetition(Repetition::new(1, 5, Box::new(Ast::Literal(Literal::new('b'))))),
    ]));
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_with_alternation() {
    let visitor = MyVisitor::new();
    let ast = Ast::Alternation(Alternation::new(vec![
        Ast::Literal(Literal::new('a')),
        Ast::Literal(Literal::new('b')),
    ]));
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit(&ast, visitor);
}

#[test]
#[should_panic]
fn test_visit_with_empty() {
    let visitor = MyVisitor::new();
    let ast = Ast::Empty(Span::new(0, 0));
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit(&ast, visitor);
}

