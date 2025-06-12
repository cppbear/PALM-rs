// Answer 0

#[test]
fn test_fmt_flags_case_insensitive() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
    }

    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct Formatter {
        wtr: MockWriter,
    }

    impl Formatter {
        fn fmt_flags(&mut self, ast: &Flags) -> std::fmt::Result {
            use ast::{Flag, FlagsItemKind};

            for item in &ast.items {
                match item.kind {
                    FlagsItemKind::Negation => self.wtr.write_str("-"),
                    FlagsItemKind::Flag(ref flag) => {
                        match *flag {
                            Flag::CaseInsensitive => self.wtr.write_str("i"),
                            Flag::MultiLine => self.wtr.write_str("m"),
                            Flag::DotMatchesNewLine => self.wtr.write_str("s"),
                            Flag::SwapGreed => self.wtr.write_str("U"),
                            Flag::Unicode => self.wtr.write_str("u"),
                            Flag::IgnoreWhitespace => self.wtr.write_str("x"),
                        }
                    }
                }?;
            }
            Ok(())
        }
    }

    struct Flags {
        items: Vec<FlagsItem>,
    }

    struct FlagsItem {
        kind: FlagsItemKind,
    }

    let ast = Flags {
        items: vec![
            FlagsItem {
                kind: FlagsItemKind::Flag(Box::new(Flag::CaseInsensitive)),
            },
            FlagsItem {
                kind: FlagsItemKind::Flag(Box::new(Flag::IgnoreWhitespace)),
            },
        ],
    };

    let mut formatter = Formatter {
        wtr: MockWriter::new(),
    };

    assert!(formatter.fmt_flags(&ast).is_ok());
    assert_eq!(formatter.wtr.output, "ix");
}

#[test]
fn test_fmt_flags_ignore_whitespace_only() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
    }

    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct Formatter {
        wtr: MockWriter,
    }

    impl Formatter {
        fn fmt_flags(&mut self, ast: &Flags) -> std::fmt::Result {
            use ast::{Flag, FlagsItemKind};

            for item in &ast.items {
                match item.kind {
                    FlagsItemKind::Negation => self.wtr.write_str("-"),
                    FlagsItemKind::Flag(ref flag) => {
                        match *flag {
                            Flag::CaseInsensitive => self.wtr.write_str("i"),
                            Flag::MultiLine => self.wtr.write_str("m"),
                            Flag::DotMatchesNewLine => self.wtr.write_str("s"),
                            Flag::SwapGreed => self.wtr.write_str("U"),
                            Flag::Unicode => self.wtr.write_str("u"),
                            Flag::IgnoreWhitespace => self.wtr.write_str("x"),
                        }
                    }
                }?;
            }
            Ok(())
        }
    }

    struct Flags {
        items: Vec<FlagsItem>,
    }

    struct FlagsItem {
        kind: FlagsItemKind,
    }

    let ast = Flags {
        items: vec![
            FlagsItem {
                kind: FlagsItemKind::Flag(Box::new(Flag::IgnoreWhitespace)),
            },
        ],
    };

    let mut formatter = Formatter {
        wtr: MockWriter::new(),
    };

    assert!(formatter.fmt_flags(&ast).is_ok());
    assert_eq!(formatter.wtr.output, "x");
}

