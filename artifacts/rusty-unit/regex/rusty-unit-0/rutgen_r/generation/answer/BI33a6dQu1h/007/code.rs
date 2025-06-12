// Answer 0

#[test]
fn test_fmt_flags_with_case_insensitive() {
    use std::fmt;
    use regex_syntax::ast::{Flags, FlagsItemKind, Flag};

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
    let flags = Flags {
        items: vec![FlagsItemKind::Flag(Flag::CaseInsensitive)],
    };

    writer.fmt_flags(&flags).unwrap();
    assert_eq!(writer.output, "i");
}

#[test]
fn test_fmt_flags_with_multi_line() {
    use std::fmt;
    use regex_syntax::ast::{Flags, FlagsItemKind, Flag};

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
    let flags = Flags {
        items: vec![FlagsItemKind::Flag(Flag::MultiLine)],
    };

    writer.fmt_flags(&flags).unwrap();
    assert_eq!(writer.output, "m");
}

#[test]
fn test_fmt_flags_with_dot_matches_new_line() {
    use std::fmt;
    use regex_syntax::ast::{Flags, FlagsItemKind, Flag};

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
    let flags = Flags {
        items: vec![FlagsItemKind::Flag(Flag::DotMatchesNewLine)],
    };

    writer.fmt_flags(&flags).unwrap();
    assert_eq!(writer.output, "s");
}

#[test]
fn test_fmt_flags_with_swap_greed() {
    use std::fmt;
    use regex_syntax::ast::{Flags, FlagsItemKind, Flag};

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
    let flags = Flags {
        items: vec![FlagsItemKind::Flag(Flag::SwapGreed)],
    };

    writer.fmt_flags(&flags).unwrap();
    assert_eq!(writer.output, "U");
}

#[test]
fn test_fmt_flags_with_unicode() {
    use std::fmt;
    use regex_syntax::ast::{Flags, FlagsItemKind, Flag};

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
    let flags = Flags {
        items: vec![FlagsItemKind::Flag(Flag::Unicode)],
    };

    writer.fmt_flags(&flags).unwrap();
    assert_eq!(writer.output, "u");
}

#[test]
fn test_fmt_flags_with_ignore_whitespace() {
    use std::fmt;
    use regex_syntax::ast::{Flags, FlagsItemKind, Flag};

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
    let flags = Flags {
        items: vec![FlagsItemKind::Flag(Flag::IgnoreWhitespace)],
    };

    writer.fmt_flags(&flags).unwrap();
    assert_eq!(writer.output, "x");
}

#[test]
fn test_fmt_flags_with_negation() {
    use std::fmt;
    use regex_syntax::ast::{Flags, FlagsItemKind, Flag};

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
    let flags = Flags {
        items: vec![FlagsItemKind::Negation],
    };

    writer.fmt_flags(&flags).unwrap();
    assert_eq!(writer.output, "-");
}

#[test]
fn test_fmt_flags_with_multiple_items() {
    use std::fmt;
    use regex_syntax::ast::{Flags, FlagsItemKind, Flag};

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
    let flags = Flags {
        items: vec![
            FlagsItemKind::Flag(Flag::DotMatchesNewLine),
            FlagsItemKind::Flag(Flag::MultiLine),
            FlagsItemKind::Negation,
        ],
    };

    writer.fmt_flags(&flags).unwrap();
    assert_eq!(writer.output, "sm-");
}

