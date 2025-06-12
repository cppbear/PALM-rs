// Answer 0

fn test_visit_pre_group() {
    struct MockVisitor;

    impl MockVisitor {
        fn fmt_group_pre(&self, _: &()) -> std::fmt::Result {
            Ok(())
        }
    }

    let mut visitor = MockVisitor;
    let ast = regex_syntax::ast::Ast::Group(());
    let result = visitor.visit_pre(&ast);
    assert_eq!(result, Ok(()));
}

fn test_visit_pre_class_bracketed() {
    struct MockVisitor;

    impl MockVisitor {
        fn fmt_class_bracketed_pre(&self, _: &()) -> std::fmt::Result {
            Ok(())
        }
    }

    let mut visitor = MockVisitor;
    let ast = regex_syntax::ast::Ast::Class(regex_syntax::ast::Class::Bracketed(()));
    let result = visitor.visit_pre(&ast);
    assert_eq!(result, Ok(()));
}

fn test_visit_pre_other() {
    struct MockVisitor;

    impl MockVisitor {
        fn fmt_group_pre(&self, _: &()) -> std::fmt::Result {
            Ok(())
        }

        fn fmt_class_bracketed_pre(&self, _: &()) -> std::fmt::Result {
            Ok(())
        }
    }

    let mut visitor = MockVisitor;
    let ast = regex_syntax::ast::Ast::Class(regex_syntax::ast::Class::Unbracketed(()));
    let result = visitor.visit_pre(&ast);
    assert_eq!(result, Ok(()));
}

