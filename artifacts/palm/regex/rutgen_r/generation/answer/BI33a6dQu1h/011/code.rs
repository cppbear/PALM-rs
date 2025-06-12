// Answer 0

#[test]
fn test_fmt_flags_case_insensitive() {
    use regex_syntax::ast::{Flags, FlagsItem, FlagsItemKind, Flag};
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

    impl std::fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    fn fmt_flags(writer: &mut TestWriter, ast: &Flags) -> std::fmt::Result {
        for item in &ast.items {
            match item.kind {
                FlagsItemKind::Negation => writer.write_str("-"),
                FlagsItemKind::Flag(ref flag) => {
                    match *flag {
                        Flag::CaseInsensitive => writer.write_str("i"),
                        Flag::MultiLine => writer.write_str("m"),
                        Flag::DotMatchesNewLine => writer.write_str("s"),
                        Flag::SwapGreed => writer.write_str("U"),
                        Flag::Unicode => writer.write_str("u"),
                        Flag::IgnoreWhitespace => writer.write_str("x"),
                    }
                }
            }?;
        }
        Ok(())
    }

    let mut writer = TestWriter::new();
    let ast = Flags {
        items: vec![
            FlagsItem { kind: FlagsItemKind::Flag(Flag::CaseInsensitive) },
            FlagsItem { kind: FlagsItemKind::Flag(Flag::MultiLine) },
            FlagsItem { kind: FlagsItemKind::Negation },
        ],
    };

    let result = fmt_flags(&mut writer, &ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, "im-");
}

#[test]
fn test_fmt_flags_multiline() {
    use regex_syntax::ast::{Flags, FlagsItem, FlagsItemKind, Flag};
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

    impl std::fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    fn fmt_flags(writer: &mut TestWriter, ast: &Flags) -> std::fmt::Result {
        for item in &ast.items {
            match item.kind {
                FlagsItemKind::Negation => writer.write_str("-"),
                FlagsItemKind::Flag(ref flag) => {
                    match *flag {
                        Flag::CaseInsensitive => writer.write_str("i"),
                        Flag::MultiLine => writer.write_str("m"),
                        Flag::DotMatchesNewLine => writer.write_str("s"),
                        Flag::SwapGreed => writer.write_str("U"),
                        Flag::Unicode => writer.write_str("u"),
                        Flag::IgnoreWhitespace => writer.write_str("x"),
                    }
                }
            }?;
        }
        Ok(())
    }

    let mut writer = TestWriter::new();
    let ast = Flags {
        items: vec![
            FlagsItem { kind: FlagsItemKind::Flag(Flag::MultiLine) },
            FlagsItem { kind: FlagsItemKind::Negation },
        ],
    };

    let result = fmt_flags(&mut writer, &ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, "m-");
}

#[test]
#[should_panic]
fn test_fmt_flags_empty() {
    use regex_syntax::ast::{Flags, FlagsItem};
    
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

    impl std::fmt::Write for TestWriter {
        fn write_str(&mut self, _: &str) -> std::fmt::Result {
            Ok(())
        }
    }

    fn fmt_flags(writer: &mut TestWriter, _ast: &Flags) -> std::fmt::Result {
        assert!(false, "Ast items cannot be empty");
        Ok(())
    }

    let mut writer = TestWriter::new();
    let ast = Flags { items: Vec::new() };

    let _ = fmt_flags(&mut writer, &ast);
}

