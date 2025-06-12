// Answer 0

fn fmt_class_unicode_test() {
    use std::fmt::{self, Write};
    use regex_syntax::ast::{ClassUnicode, ClassUnicodeKind, ClassUnicodeOpKind};

    struct TestWriter {
        output: String,
    }

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
        
        fn write_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };

    // Test case for negated: true, with NamedValue { op: NotEqual }
    {
        let ast = ClassUnicode {
            negated: true,
            kind: ClassUnicodeKind::NamedValue {
                op: ClassUnicodeOpKind::NotEqual,
                name: "test".to_string(),
                value: "value".to_string(),
            },
        };
        writer.fmt_class_unicode(&ast).unwrap();
        assert_eq!(writer.output, r"\P{{test!=value}}");
    }
    
    // Test case for negated: true, with NamedValue { op: Equal }
    {
        let mut writer = TestWriter { output: String::new() };
        let ast = ClassUnicode {
            negated: true,
            kind: ClassUnicodeKind::NamedValue {
                op: ClassUnicodeOpKind::Equal,
                name: "test".to_string(),
                value: "value".to_string(),
            },
        };
        writer.fmt_class_unicode(&ast).unwrap();
        assert_eq!(writer.output, r"\P{{test=value}}");
    }

    // Test case for negated: true, with NamedValue { op: Colon }
    {
        let mut writer = TestWriter { output: String::new() };
        let ast = ClassUnicode {
            negated: true,
            kind: ClassUnicodeKind::NamedValue {
                op: ClassUnicodeOpKind::Colon,
                name: "test".to_string(),
                value: "value".to_string(),
            },
        };
        writer.fmt_class_unicode(&ast).unwrap();
        assert_eq!(writer.output, r"\P{{test:value}}");
    }
    
    // Test case for negated: false, with Named
    {
        let mut writer = TestWriter { output: String::new() };
        let ast = ClassUnicode {
            negated: false,
            kind: ClassUnicodeKind::Named("test".to_string()),
        };
        writer.fmt_class_unicode(&ast).unwrap();
        assert_eq!(writer.output, r"\p{{test}}");
    }
}

