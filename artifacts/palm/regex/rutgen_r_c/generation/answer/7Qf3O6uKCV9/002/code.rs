// Answer 0

#[test]
fn test_fmt_assertion_word_boundary() {
    use std::fmt::Write;

    struct DummyWriter {
        output: String,
    }

    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct DummyAst {
        kind: ast::AssertionKind,
    }

    let mut writer = DummyWriter { output: String::new() };
    let assertion = DummyAst { kind: ast::AssertionKind::WordBoundary };

    let mut assertion_writer = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    let result = assertion_writer.fmt_assertion(&assertion);
    
    assert!(result.is_ok());
    assert_eq!(assertion_writer.wtr.output, r"\b");
} 

#[test]
fn test_fmt_assertion_not_word_boundary() {
    use std::fmt::Write;

    struct DummyWriter {
        output: String,
    }

    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct DummyAst {
        kind: ast::AssertionKind,
    }

    let mut writer = DummyWriter { output: String::new() };
    let assertion = DummyAst { kind: ast::AssertionKind::NotWordBoundary };

    let mut assertion_writer = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    let result = assertion_writer.fmt_assertion(&assertion);
    
    assert!(result.is_ok());
    assert_eq!(assertion_writer.wtr.output, r"\B");
} 

#[test]
fn test_fmt_assertion_start_line() {
    use std::fmt::Write;

    struct DummyWriter {
        output: String,
    }

    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct DummyAst {
        kind: ast::AssertionKind,
    }

    let mut writer = DummyWriter { output: String::new() };
    let assertion = DummyAst { kind: ast::AssertionKind::StartLine };

    let mut assertion_writer = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    let result = assertion_writer.fmt_assertion(&assertion);
    
    assert!(result.is_ok());
    assert_eq!(assertion_writer.wtr.output, "^");
} 

#[test]
fn test_fmt_assertion_end_line() {
    use std::fmt::Write;

    struct DummyWriter {
        output: String,
    }

    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct DummyAst {
        kind: ast::AssertionKind,
    }

    let mut writer = DummyWriter { output: String::new() };
    let assertion = DummyAst { kind: ast::AssertionKind::EndLine };

    let mut assertion_writer = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    let result = assertion_writer.fmt_assertion(&assertion);
    
    assert!(result.is_ok());
    assert_eq!(assertion_writer.wtr.output, "$");
} 

#[test]
fn test_fmt_assertion_start_text() {
    use std::fmt::Write;

    struct DummyWriter {
        output: String,
    }

    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct DummyAst {
        kind: ast::AssertionKind,
    }

    let mut writer = DummyWriter { output: String::new() };
    let assertion = DummyAst { kind: ast::AssertionKind::StartText };

    let mut assertion_writer = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    let result = assertion_writer.fmt_assertion(&assertion);
    
    assert!(result.is_ok());
    assert_eq!(assertion_writer.wtr.output, r"\A");
} 

#[test]
fn test_fmt_assertion_end_text() {
    use std::fmt::Write;

    struct DummyWriter {
        output: String,
    }

    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct DummyAst {
        kind: ast::AssertionKind,
    }

    let mut writer = DummyWriter { output: String::new() };
    let assertion = DummyAst { kind: ast::AssertionKind::EndText };

    let mut assertion_writer = Writer { printer: &mut Printer { _priv: () }, wtr: writer };
    let result = assertion_writer.fmt_assertion(&assertion);
    
    assert!(result.is_ok());
    assert_eq!(assertion_writer.wtr.output, r"\z");
}

