// Answer 0

#[test]
fn test_fmt_single_line_error() {
    struct ParseError {
        pattern: String,
        err: String,
    }

    impl ParseError {
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

    let error = ParseError {
        pattern: "a*b".to_string(),
        err: "Invalid regex".to_string(),
    };
    
    let result = error.fmt(&mut std::fmt::Formatter::new());
    assert!(result.is_ok());
}

#[test]
fn test_fmt_multi_line_error() {
    struct ParseError {
        pattern: String,
        err: String,
    }

    impl ParseError {
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

    let error = ParseError {
        pattern: "a\nb".to_string(),
        err: "Invalid regex".to_string(),
    };
    
    let result = error.fmt(&mut std::fmt::Formatter::new());
    assert!(result.is_ok());
}

