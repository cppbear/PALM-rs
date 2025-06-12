// Answer 0

#[test]
fn test_fmt_set_flags_success() {
    use std::fmt::Write;
    
    struct TestWriter {
        output: String,
    }

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut ast = ast::SetFlags {
        span: Span { start: 0, end: 10 },
        flags: Flags {
            span: Span { start: 0, end: 10 },
            items: vec![],
        },
    };
    
    let mut writer_ref = Writer { printer: &mut printer, wtr: writer };

    let result = writer_ref.fmt_set_flags(&ast);

    assert!(result.is_ok());
    assert_eq!(writer_ref.wtr.output, "(?)");
}

#[test]
#[should_panic]
fn test_fmt_set_flags_write_str_failure() {
    use std::fmt;

    struct PanicWriter;

    impl Write for PanicWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    let mut printer = Printer { _priv: () };
    let mut ast = ast::SetFlags {
        span: Span { start: 0, end: 10 },
        flags: Flags {
            span: Span { start: 0, end: 10 },
            items: vec![],
        },
    };

    let mut writer_ref = Writer { printer: &mut printer, wtr: PanicWriter };

    writer_ref.fmt_set_flags(&ast).unwrap();
}

#[test]
#[should_panic]
fn test_fmt_set_flags_fmt_flags_failure() {
    use std::fmt;

    struct TestWriter {
        output: String,
    }

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct FailFlagsWriter {
        output: String,
    }

    impl Writer<'_, FailFlagsWriter> {
        fn fmt_flags(&mut self, _: &Flags) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut printer = Printer { _priv: () };
    let mut ast = ast::SetFlags {
        span: Span { start: 0, end: 10 },
        flags: Flags {
            span: Span { start: 0, end: 10 },
            items: vec![],
        },
    };

    let mut writer_ref = Writer { printer: &mut printer, wtr: writer };

    writer_ref.fmt_flags(&ast.flags).unwrap(); // This will cause the panic
    writer_ref.fmt_set_flags(&ast).unwrap();
}

