// Answer 0

#[test]
fn test_fmt_class_ascii_xdigit_not_negated() {
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
    let mut writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut mock_writer,
    };

    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Xdigit,
        negated: false,
    };

    writer.fmt_class_ascii(&ast);
}

