// Answer 0

#[test]
fn test_visit_with_simple_ast() {
    let mut visitor = SimpleVisitor::new();
    let ast = Ast::Literal(Literal::new('a'));

    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_with_complex_ast() {
    let mut visitor = ComplexVisitor::new();
    let ast = Ast::Alternation(Alternation::new(vec![
        Ast::Literal(Literal::new('a')),
        Ast::Literal(Literal::new('b')),
    ]));

    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_with_repetition_ast() {
    let mut visitor = RepetitionVisitor::new();
    let ast = Ast::Repetition(Repetition::new(Ast::Literal(Literal::new('a'))));

    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_with_group_ast() {
    let mut visitor = GroupVisitor::new();
    let ast = Ast::Group(Group::new(vec![
        Ast::Literal(Literal::new('a')),
        Ast::Literal(Literal::new('b')),
    ]));

    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_with_concat_ast() {
    let mut visitor = ConcatVisitor::new();
    let ast = Ast::Concat(Concat::new(vec![
        Ast::Literal(Literal::new('a')),
        Ast::Literal(Literal::new('b')),
    ]));

    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_with_deeply_nested_ast() {
    let mut visitor = DeepVisitor::new();
    let ast = Ast::Repetition(Repetition::new(Ast::Group(Group::new(vec![
        Ast::Literal(Literal::new('c')),
        Ast::Repetition(Repetition::new(Ast::Literal(Literal::new('d')))),
    ]))));

    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit(&ast, visitor);
}

