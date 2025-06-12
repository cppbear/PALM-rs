// Answer 0

#[test]
fn test_fmt_class_ascii_space_negated() {
    use ast::{ClassAscii, ClassAsciiKind};
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
    let mut printer = Printer { _priv: () };
    let mut writer_instance = Writer { printer: &mut printer, wtr: &mut writer };
    
    let ast = ClassAscii {
        span: Span::default(), // Assuming a default or valid Span can be constructed
        kind: ClassAsciiKind::Space,
        negated: true,
    };

    writer_instance.fmt_class_ascii(&ast);
}

