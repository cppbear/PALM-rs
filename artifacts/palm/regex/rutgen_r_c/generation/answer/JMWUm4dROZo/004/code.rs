// Answer 0

#[test]
fn test_fmt_set_flags() {
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };

    let dummy_printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut dummy_printer, wtr: writer };

    let flags_item = ast::FlagsItem {
        kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
    };

    let ast_flags = Flags {
        span: Span::default(),
        items: vec![flags_item],
    };

    let ast_set_flags = SetFlags {
        span: Span::default(),
        flags: ast_flags,
    };

    let result = writer_instance.fmt_set_flags(&ast_set_flags);
    
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, "(?i)");
}

