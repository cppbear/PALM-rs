// Answer 0

#[test]
fn test_fmt_class_ascii_negated() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Ascii,
        negated: true,
    };

    let mut writer_instance = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    let _ = writer_instance.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_non_negated() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Ascii,
        negated: false,
    };

    let mut writer_instance = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    let _ = writer_instance.fmt_class_ascii(&ast);
}

