// Answer 0

fn test_fmt_class_ascii_punct_negated() {
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
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let class_ascii = ast::ClassAscii {
        span: ast::Span::default(),
        kind: ast::ClassAsciiKind::Punct,
        negated: true,
    };

    writer_instance.fmt_class_ascii(&class_ascii).unwrap();
    assert_eq!(writer.output, "[:-^punct:]");
}

fn test_fmt_class_ascii_punct_non_negated() {
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
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let class_ascii = ast::ClassAscii {
        span: ast::Span::default(),
        kind: ast::ClassAsciiKind::Punct,
        negated: false,
    };

    writer_instance.fmt_class_ascii(&class_ascii).unwrap();
    assert_eq!(writer.output, "[:punct:]");
}

