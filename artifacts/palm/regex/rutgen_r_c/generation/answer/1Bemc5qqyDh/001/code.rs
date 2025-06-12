// Answer 0

#[test]
fn test_visit_empty_ast() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_empty(&mut self, _: &Span) {}
    }

    let ast = Ast::Empty(Span::default());
    let result = visit(&ast, TestVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_visit_literal_ast() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_literal(&mut self, _: &Literal) {}
    }

    let ast = Ast::Literal(Literal::default());
    let result = visit(&ast, TestVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_visit_dot_ast() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_dot(&mut self, _: &Span) {}
    }

    let ast = Ast::Dot(Span::default());
    let result = visit(&ast, TestVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_visit_assertion_ast() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_assertion(&mut self, _: &Assertion) {}
    }

    let ast = Ast::Assertion(Assertion::default());
    let result = visit(&ast, TestVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_visit_class_ast() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_class(&mut self, _: &Class) {}
    }

    let ast = Ast::Class(Class::default());
    let result = visit(&ast, TestVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_visit_repetition_ast() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_repetition(&mut self, _: &Repetition) {}
    }

    let ast = Ast::Repetition(Repetition::default());
    let result = visit(&ast, TestVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_visit_group_ast() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_group(&mut self, _: &Group) {}
    }

    let ast = Ast::Group(Group::default());
    let result = visit(&ast, TestVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_visit_alternation_ast() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_alternation(&mut self, _: &Alternation) {}
    }

    let ast = Ast::Alternation(Alternation::default());
    let result = visit(&ast, TestVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_visit_concat_ast() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_concat(&mut self, _: &Concat) {}
    }

    let ast = Ast::Concat(Concat::default());
    let result = visit(&ast, TestVisitor);
    assert!(result.is_ok());
}

