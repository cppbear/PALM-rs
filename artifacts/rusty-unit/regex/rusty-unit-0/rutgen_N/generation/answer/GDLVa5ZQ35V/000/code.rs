// Answer 0

#[test]
fn test_visit_post_empty() {
    struct TestVisitor {
        wtr: Vec<u8>,
    }

    impl TestVisitor {
        fn fmt_set_flags(&mut self, _flags: &()) -> fmt::Result { Ok(()) }
        fn fmt_literal(&mut self, _literal: &str) -> fmt::Result { Ok(()) }
        fn fmt_assertion(&mut self, _assertion: &()) -> fmt::Result { Ok(()) }
        fn fmt_class_perl(&mut self, _class: &()) -> fmt::Result { Ok(()) }
        fn fmt_class_unicode(&mut self, _class: &()) -> fmt::Result { Ok(()) }
        fn fmt_class_bracketed_post(&mut self, _class: &()) -> fmt::Result { Ok(()) }
        fn fmt_repetition(&mut self, _repetition: &()) -> fmt::Result { Ok(()) }
        fn fmt_group_post(&mut self, _group: &()) -> fmt::Result { Ok(()) }
    }

    let mut visitor = TestVisitor { wtr: Vec::new() };
    let ast = Ast::Empty(());
    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_visit_post_literal() {
    struct TestVisitor {
        wtr: Vec<u8>,
    }

    impl TestVisitor {
        fn fmt_set_flags(&mut self, _flags: &()) -> fmt::Result { Ok(()) }
        fn fmt_literal(&mut self, literal: &str) -> fmt::Result { 
            self.wtr.extend_from_slice(literal.as_bytes()); 
            Ok(()) 
        }
        fn fmt_assertion(&mut self, _assertion: &()) -> fmt::Result { Ok(()) }
        fn fmt_class_perl(&mut self, _class: &()) -> fmt::Result { Ok(()) }
        fn fmt_class_unicode(&mut self, _class: &()) -> fmt::Result { Ok(()) }
        fn fmt_class_bracketed_post(&mut self, _class: &()) -> fmt::Result { Ok(()) }
        fn fmt_repetition(&mut self, _repetition: &()) -> fmt::Result { Ok(()) }
        fn fmt_group_post(&mut self, _group: &()) -> fmt::Result { Ok(()) }
    }

    let mut visitor = TestVisitor { wtr: Vec::new() };
    let ast = Ast::Literal("test");
    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(visitor.wtr).unwrap(), "test");
}

#[test]
fn test_visit_post_dot() {
    struct TestVisitor {
        wtr: Vec<u8>,
    }

    impl TestVisitor {
        fn fmt_set_flags(&mut self, _flags: &()) -> fmt::Result { Ok(()) }
        fn fmt_literal(&mut self, _literal: &str) -> fmt::Result { Ok(()) }
        fn fmt_assertion(&mut self, _assertion: &()) -> fmt::Result { Ok(()) }
        fn fmt_class_perl(&mut self, _class: &()) -> fmt::Result { Ok(()) }
        fn fmt_class_unicode(&mut self, _class: &()) -> fmt::Result { Ok(()) }
        fn fmt_class_bracketed_post(&mut self, _class: &()) -> fmt::Result { Ok(()) }
        fn fmt_repetition(&mut self, _repetition: &()) -> fmt::Result { Ok(()) }
        fn fmt_group_post(&mut self, _group: &()) -> fmt::Result { Ok(()) }
    }

    let mut visitor = TestVisitor { wtr: Vec::new() };
    let ast = Ast::Dot(());
    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(visitor.wtr).unwrap(), ".");
}

