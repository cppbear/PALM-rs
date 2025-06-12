// Answer 0

#[test]
fn test_fmt_flags_with_case_insensitive() {
    struct TestWriter(String);
    
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter(String::new());
    let mut printer = Printer { _priv: () };
    let flags = Flags {
        span: Span::default(), // assuming a default method for Span
        items: vec![
            FlagsItem { 
                span: Span::default(), 
                kind: FlagsItemKind::Flag(Box::new(Flag::CaseInsensitive)) 
            },
        ],
    };
    
    let mut format_writer = Writer { printer: &mut printer, wtr: writer };
    format_writer.fmt_flags(&flags).unwrap();
    
    assert_eq!(format_writer.wtr.0, "i");
}

#[test]
fn test_fmt_flags_with_multi_line() {
    struct TestWriter(String);
    
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter(String::new());
    let mut printer = Printer { _priv: () };
    let flags = Flags {
        span: Span::default(),
        items: vec![
            FlagsItem { 
                span: Span::default(), 
                kind: FlagsItemKind::Flag(Box::new(Flag::MultiLine)) 
            },
        ],
    };
    
    let mut format_writer = Writer { printer: &mut printer, wtr: writer };
    format_writer.fmt_flags(&flags).unwrap();
    
    assert_eq!(format_writer.wtr.0, "m");
}

#[test]
fn test_fmt_flags_with_swap_greed() {
    struct TestWriter(String);
    
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter(String::new());
    let mut printer = Printer { _priv: () };
    let flags = Flags {
        span: Span::default(),
        items: vec![
            FlagsItem { 
                span: Span::default(), 
                kind: FlagsItemKind::Flag(Box::new(Flag::SwapGreed)) 
            },
        ],
    };
    
    let mut format_writer = Writer { printer: &mut printer, wtr: writer };
    format_writer.fmt_flags(&flags).unwrap();
    
    assert_eq!(format_writer.wtr.0, "U");
}

#[test]
fn test_fmt_flags_with_unicode() {
    struct TestWriter(String);
    
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter(String::new());
    let mut printer = Printer { _priv: () };
    let flags = Flags {
        span: Span::default(),
        items: vec![
            FlagsItem { 
                span: Span::default(), 
                kind: FlagsItemKind::Flag(Box::new(Flag::Unicode)) 
            },
        ],
    };
    
    let mut format_writer = Writer { printer: &mut printer, wtr: writer };
    format_writer.fmt_flags(&flags).unwrap();
    
    assert_eq!(format_writer.wtr.0, "u");
}

#[test]
fn test_fmt_flags_with_ignore_whitespace() {
    struct TestWriter(String);
    
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter(String::new());
    let mut printer = Printer { _priv: () };
    let flags = Flags {
        span: Span::default(),
        items: vec![
            FlagsItem { 
                span: Span::default(), 
                kind: FlagsItemKind::Flag(Box::new(Flag::IgnoreWhitespace)) 
            },
        ],
    };
    
    let mut format_writer = Writer { printer: &mut printer, wtr: writer };
    format_writer.fmt_flags(&flags).unwrap();
    
    assert_eq!(format_writer.wtr.0, "x");
}

#[test]
fn test_fmt_flags_with_negation() {
    struct TestWriter(String);
    
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter(String::new());
    let mut printer = Printer { _priv: () };
    let flags = Flags {
        span: Span::default(),
        items: vec![
            FlagsItem { 
                span: Span::default(), 
                kind: FlagsItemKind::Negation 
            },
        ],
    };
    
    let mut format_writer = Writer { printer: &mut printer, wtr: writer };
    format_writer.fmt_flags(&flags).unwrap();
    
    assert_eq!(format_writer.wtr.0, "-");
}

