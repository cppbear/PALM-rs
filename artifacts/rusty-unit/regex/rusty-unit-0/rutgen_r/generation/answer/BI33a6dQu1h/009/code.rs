// Answer 0

fn test_fmt_flags() {
    use std::fmt;
    use regex_syntax::ast::{self, Flag, FlagsItemKind, Flags};

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
    
    // Test with a single MultiLine flag
    let ast = Flags {
        items: vec![
            ast::FlagsItem { kind: FlagsItemKind::Flag(Flag::MultiLine) }
        ],
    };
    writer.fmt_flags(&ast).unwrap();
    assert_eq!(writer.output, "m");

    // Test with multiple flags including MultiLine
    writer.output.clear();
    let ast = Flags {
        items: vec![
            ast::FlagsItem { kind: FlagsItemKind::Flag(Flag::CaseInsensitive) },
            ast::FlagsItem { kind: FlagsItemKind::Flag(Flag::MultiLine) },
        ],
    };
    writer.fmt_flags(&ast).unwrap();
    assert_eq!(writer.output, "im");

    // Test with only Negation
    writer.output.clear();
    let ast = Flags {
        items: vec![
            ast::FlagsItem { kind: FlagsItemKind::Negation },
            ast::FlagsItem { kind: FlagsItemKind::Flag(Flag::MultiLine) },
        ],
    };
    writer.fmt_flags(&ast).unwrap();
    assert_eq!(writer.output, "-m");

    // Test with no items (should not panic, but return Ok)
    writer.output.clear();
    let ast = Flags { items: vec![] };
    writer.fmt_flags(&ast).unwrap();
    assert_eq!(writer.output, "");
}

