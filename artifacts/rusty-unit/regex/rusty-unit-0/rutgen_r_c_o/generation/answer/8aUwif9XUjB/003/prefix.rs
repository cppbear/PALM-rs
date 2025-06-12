// Answer 0

#[test]
fn test_visit_basic_construct() {
    let mut visitor = SimpleVisitor::new();
    let ast = Ast::Group(Group::new(vec![]));
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_concat_with_children() {
    let mut visitor = SimpleVisitor::new();
    let ast = Ast::Concat(Concat::new(vec![Ast::Literal(Literal::new('a')), Ast::Literal(Literal::new('b'))]));
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_alternation_with_children() {
    let mut visitor = SimpleVisitor::new();
    let ast = Ast::Alternation(Alternation::new(vec![Ast::Literal(Literal::new('a')), Ast::Literal(Literal::new('b'))]));
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_repetition() {
    let mut visitor = SimpleVisitor::new();
    let ast = Ast::Repetition(Repetition::new(Ast::Literal(Literal::new('a')), 1, None));
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_deep_recursion() {
    let mut visitor = SimpleVisitor::new();
    let mut children = Vec::new();
    for _ in 0..50 {
        children.push(Ast::Literal(Literal::new('a')));
    }
    let ast = Ast::Concat(Concat::new(children));
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_empty_group() {
    let mut visitor = SimpleVisitor::new();
    let ast = Ast::Group(Group::new(vec![]));
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_large_input() {
    let mut visitor = SimpleVisitor::new();
    let mut children = Vec::new();
    for i in 0..1000 {
        children.push(Ast::Literal(Literal::new(('a' as u8 + (i % 26) as u8) as char)));
    }
    let ast = Ast::Concat(Concat::new(children));
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&ast, visitor);
}

