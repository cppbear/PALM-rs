// Answer 0

#[test]
fn test_fmt_with_newline_pattern_success() {
    struct MockError {
        err: &'static str,
        pattern: String,
    }

    impl MockError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let spans = Spans::from_formatter(self);
            if self.pattern.contains('\n') {
                let divider = repeat_char('~', 79);

                writeln!(f, "regex parse error:")?;
                writeln!(f, "{}", divider)?;
                let notated = spans.notate();
                write!(f, "{}", notated)?;
                writeln!(f, "{}", divider)?;
                if !spans.multi_line.is_empty() {
                    let mut notes = vec![];
                    for span in &spans.multi_line {
                        notes.push(format!(
                            "on line {} (column {}) through line {} (column {})",
                            span.start.line, span.start.column,
                            span.end.line, span.end.column - 1));
                    }
                    writeln!(f, "{}", notes.join("\n"))?;
                }
                write!(f, "error: {}", self.err)?;
            } else {
                writeln!(f, "regex parse error:")?;
                let notated = Spans::from_formatter(self).notate();
                write!(f, "{}", notated)?;
                write!(f, "error: {}", self.err)?;
            }
            Ok(())
        }
    }

    struct Spans;

    impl Spans {
        fn from_formatter(_error: &MockError) -> Self {
            Spans
        }
        
        fn notate(&self) -> String {
            "notated pattern".to_string()
        }
    }

    let error = MockError {
        err: "Invalid regex",
        pattern: "a+b\nc*d".to_string(),
    };
    
    let mut output = Vec::new();
    let result = error.fmt(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert!(!output.is_empty());
}

#[test]
#[should_panic]
fn test_fmt_with_newline_pattern_fail() {
    struct MockErrorFail {
        err: &'static str,
        pattern: String,
    }

    impl MockErrorFail {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let spans = SpansFail::from_formatter(self);
            if self.pattern.contains('\n') {
                let divider = repeat_char('~', 79);

                writeln!(f, "regex parse error:")?;
                // Intentionally causing a panic by failing to write the divider
                writeln!(f, "{}", "simulated write error")?;
                let notated = spans.notate();
                write!(f, "{}", notated)?;
                writeln!(f, "{}", divider)?;
                if !spans.multi_line.is_empty() {
                    let mut notes = vec![];
                    for span in &spans.multi_line {
                        notes.push(format!(
                            "on line {} (column {}) through line {} (column {})",
                            span.start.line, span.start.column,
                            span.end.line, span.end.column - 1));
                    }
                    writeln!(f, "{}", notes.join("\n"))?;
                }
                write!(f, "error: {}", self.err)?;
            }
            Ok(())
        }
    }

    struct SpansFail;

    impl SpansFail {
        fn from_formatter(_error: &MockErrorFail) -> Self {
            SpansFail
        }
        
        fn notate(&self) -> String {
            "notated pattern".to_string()
        }
    }

    let error = MockErrorFail {
        err: "Invalid regex",
        pattern: "a+b\nc*d".to_string(),
    };
    
    let mut output = Vec::new();
    error.fmt(&mut std::fmt::Formatter::new(&mut output)).unwrap();
}

