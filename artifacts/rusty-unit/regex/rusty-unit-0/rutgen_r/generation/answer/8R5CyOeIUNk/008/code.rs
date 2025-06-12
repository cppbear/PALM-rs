// Answer 0

#[test]
fn test_induct_with_class() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Err = ();

        fn visit_class(&mut self, _class: &ast::Class, _visitor: &mut Self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let ast = Ast::Class(ast::Class::Bracketed(vec![]));
    let mut visitor = TestVisitor;

    let result = induct(&mut visitor, &ast, &mut visitor);
    assert_eq!(result, Ok(None));
}

#[test]
fn test_induct_with_repetition() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Err = ();

        fn visit_class(&mut self, _class: &ast::Class, _visitor: &mut Self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let ast = Ast::Repetition(Box::new(Ast::Class(ast::Class::Bracketed(vec![]))));
    let mut visitor = TestVisitor;

    let result = induct(&mut visitor, &ast, &mut visitor);
    assert_eq!(result, Ok(Some(Frame::Repetition(&ast))));
}

#[test]
fn test_induct_with_group() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Err = ();

        fn visit_class(&mut self, _class: &ast::Class, _visitor: &mut Self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let ast = Ast::Group(Box::new(Ast::Class(ast::Class::Bracketed(vec![]))));
    let mut visitor = TestVisitor;

    let result = induct(&mut visitor, &ast, &mut visitor);
    assert_eq!(result, Ok(Some(Frame::Group(&ast))));
}

#[test]
fn test_induct_with_concat_non_empty() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Err = ();

        fn visit_class(&mut self, _class: &ast::Class, _visitor: &mut Self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let ast = Ast::Concat(vec![
        Box::new(Ast::Class(ast::Class::Bracketed(vec![]))),
        Box::new(Ast::Class(ast::Class::Bracketed(vec![]))),
    ]);
    let mut visitor = TestVisitor;

    let result = induct(&mut visitor, &ast, &mut visitor);
    assert_eq!(result, Ok(Some(Frame::Concat {
        head: &ast.asts[0],
        tail: &ast.asts[1..],
    })));
}

#[test]
fn test_induct_with_alternation_non_empty() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Err = ();

        fn visit_class(&mut self, _class: &ast::Class, _visitor: &mut Self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let ast = Ast::Alternation(vec![
        Box::new(Ast::Class(ast::Class::Bracketed(vec![]))),
        Box::new(Ast::Class(ast::Class::Bracketed(vec![]))),
    ]);
    let mut visitor = TestVisitor;

    let result = induct(&mut visitor, &ast, &mut visitor);
    assert_eq!(result, Ok(Some(Frame::Alternation {
        head: &ast.asts[0],
        tail: &ast.asts[1..],
    })));
}

