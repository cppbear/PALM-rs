// Answer 0

#[derive(Debug)]
struct MockWriter {
    output: String,
}

impl fmt::Write for MockWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.output.push_str(s);
        Ok(())
    }
}

#[test]
fn test_fmt_class_ascii_space() {
    let mut writer = MockWriter { output: String::new() };
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::ClassAscii {
        span: Span {},
        kind: ClassAsciiKind::Space,
        negated: false,
    };

    writer_instance.fmt_class_ascii(&ast).unwrap();
    assert_eq!(writer.output, "[:space:]");
}

#[test]
fn test_fmt_class_ascii_negated_space() {
    let mut writer = MockWriter { output: String::new() };
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };

    let ast = ast::ClassAscii {
        span: Span {},
        kind: ClassAsciiKind::Space,
        negated: true,
    };

    writer_instance.fmt_class_ascii(&ast).unwrap();
    assert_eq!(writer.output, "[:^space:]");
}

