// Answer 0

#[test]
fn test_fmt_group_post() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let printer = Printer { _priv: () };
    
    let mut writer = Writer {
        printer: &mut printer,
        wtr: mock_writer,
    };

    let ast_group = ast::Group {
        span: Default::default(),
        kind: Default::default(),
        ast: Box::new(ast::Ast::Empty),
    };

    let result = writer.fmt_group_post(&ast_group);
    
    assert!(result.is_ok());
    assert_eq!(writer.wtr.output, ")");
}

