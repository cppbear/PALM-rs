// Answer 0

fn fmt_flags_test() -> fmt::Result {
    use regex_syntax::ast::{Flag, FlagsItemKind, Flags};
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                output: String::new(),
            }
        }
    }

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    
    // Test with a single Unicode flag
    let ast = Flags {
        items: vec![
            FlagsItemKind::Flag(Box::new(Flag::Unicode)),
        ],
    };
    
    writer.fmt_flags(&ast).unwrap();
    assert_eq!(writer.output, "u");

    // Test with multiple flags including a negation for coverage
    let ast = Flags {
        items: vec![
            FlagsItemKind::Flag(Box::new(Flag::CaseInsensitive)),
            FlagsItemKind::Flag(Box::new(Flag::MultiLine)),
            FlagsItemKind::Negation,  
            FlagsItemKind::Flag(Box::new(Flag::Unicode)),
        ],
    };
    
    writer = TestWriter::new(); // reset the writer
    writer.fmt_flags(&ast).unwrap();
    assert_eq!(writer.output, "im-u");

    // Test with an empty Flags
    let ast = Flags {
        items: vec![],
    };

    writer = TestWriter::new(); // reset the writer
    writer.fmt_flags(&ast).unwrap();
    assert_eq!(writer.output, "");

    Ok(())
}

#[test]
fn test_fmt_flags() {
    fmt_flags_test().unwrap();
}

