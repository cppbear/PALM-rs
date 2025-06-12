// Answer 0

#[test]
fn test_fmt_class_ascii_blank_negated() {
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

    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Blank,
        negated: true,
    };

    let mut writer_instance = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    let _ = writer_instance.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_blank_non_negated() {
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

    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Blank,
        negated: false,
    };

    let mut writer_instance = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    let _ = writer_instance.fmt_class_ascii(&ast);
}

