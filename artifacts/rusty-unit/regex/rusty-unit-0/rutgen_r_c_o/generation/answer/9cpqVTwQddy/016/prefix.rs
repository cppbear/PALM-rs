// Answer 0

#[test]
fn test_fmt_class_ascii_graph_not_negated() {
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
    let printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut writer, wtr: writer };

    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Graph,
        negated: false,
    };

    writer_instance.fmt_class_ascii(&ast);
}

