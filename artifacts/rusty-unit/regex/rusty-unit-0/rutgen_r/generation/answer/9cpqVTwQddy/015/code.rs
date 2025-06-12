// Answer 0

#[test]
fn test_fmt_class_ascii_graph_negated() {
    use regex_syntax::ast::{ClassAscii, ClassAsciiKind};
    use std::fmt;

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
    let ast = ClassAscii {
        kind: ClassAsciiKind::Graph,
        negated: true,
    };
    
    let result = writer.write_str(if ast.negated { "[:^graph:]" } else { "[:graph:]" });
    assert!(result.is_ok());
    assert_eq!(writer.output, "[:^graph:]");
}

#[test]
fn test_fmt_class_ascii_graph_non_negated() {
    use regex_syntax::ast::{ClassAscii, ClassAsciiKind};
    use std::fmt;

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
    let ast = ClassAscii {
        kind: ClassAsciiKind::Graph,
        negated: false,
    };
    
    let result = writer.write_str(if ast.negated { "[:^graph:]" } else { "[:graph:]" });
    assert!(result.is_ok());
    assert_eq!(writer.output, "[:graph:]");
}

