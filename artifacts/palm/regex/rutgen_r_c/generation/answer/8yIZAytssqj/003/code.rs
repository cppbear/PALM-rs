// Answer 0

#[test]
fn test_visit_pre_with_bracketed_class() {
    struct MockWriter<'p> {
        writer_output: String,
    }
    
    impl fmt::Write for MockWriter<'_> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.writer_output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { writer_output: String::new() };

    let ast = ast::Ast::Class(ast::Class::Bracketed(ast::ClassBracketed {
        span: Span::default(),
        negated: false,
        kind: ast::ClassSet::Union,
    }));

    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    
    let result = visitor.visit_pre(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_visit_pre_with_group() {
    struct MockWriter<'p> {
        writer_output: String,
    }

    impl fmt::Write for MockWriter<'_> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.writer_output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { writer_output: String::new() };

    let ast = ast::Ast::Group(ast::Group {
        span: Span::default(),
        kind: ast::GroupKind::CaptureIndex(0),
        ast: Box::new(ast::Ast::Empty(Span::default())),
    });

    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let result = visitor.visit_pre(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_visit_pre_with_non_matching_ast() {
    struct MockWriter<'p> {
        writer_output: String,
    }

    impl fmt::Write for MockWriter<'_> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.writer_output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { writer_output: String::new() };

    let ast = ast::Ast::Literal(ast::Literal {
        span: Span::default(),
        value: 'a',
    });

    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    
    let result = visitor.visit_pre(&ast);
    assert!(result.is_ok());
}

