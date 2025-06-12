// Answer 0

fn test_fmt_flags_case_insensitive() {
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

    let flags_item = ast::FlagsItem {
        span: Span { start: 0, end: 1 },
        kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
    };

    let flags = ast::Flags {
        span: Span { start: 0, end: 10 },
        items: vec![flags_item],
    };
    
    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    
    let result = writer_instance.fmt_flags(&flags);
    
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, "i");
}

fn test_fmt_flags_multi_line() {
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

    let flags_item = ast::FlagsItem {
        span: Span { start: 0, end: 1 },
        kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
    };

    let flags = ast::Flags {
        span: Span { start: 0, end: 10 },
        items: vec![flags_item],
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    
    let result = writer_instance.fmt_flags(&flags);
    
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, "m");
}

fn test_fmt_flags_dot_matches_new_line() {
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

    let flags_item = ast::FlagsItem {
        span: Span { start: 0, end: 1 },
        kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine),
    };

    let flags = ast::Flags {
        span: Span { start: 0, end: 10 },
        items: vec![flags_item],
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    
    let result = writer_instance.fmt_flags(&flags);
    
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, "s");
}

fn test_fmt_flags_swap_greed() {
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

    let flags_item = ast::FlagsItem {
        span: Span { start: 0, end: 1 },
        kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed),
    };

    let flags = ast::Flags {
        span: Span { start: 0, end: 10 },
        items: vec![flags_item],
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    
    let result = writer_instance.fmt_flags(&flags);
    
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, "U");
}

fn test_fmt_flags_unicode() {
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

    let flags_item = ast::FlagsItem {
        span: Span { start: 0, end: 1 },
        kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode),
    };

    let flags = ast::Flags {
        span: Span { start: 0, end: 10 },
        items: vec![flags_item],
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    
    let result = writer_instance.fmt_flags(&flags);
    
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, "u");
}

fn test_fmt_flags_ignore_whitespace() {
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

    let flags_item = ast::FlagsItem {
        span: Span { start: 0, end: 1 },
        kind: ast::FlagsItemKind::Flag(ast::Flag::IgnoreWhitespace),
    };

    let flags = ast::Flags {
        span: Span { start: 0, end: 10 },
        items: vec![flags_item],
    };

    let mut writer_instance = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    
    let result = writer_instance.fmt_flags(&flags);
    
    assert!(result.is_ok());
    assert_eq!(writer_instance.wtr.output, "x");
}

