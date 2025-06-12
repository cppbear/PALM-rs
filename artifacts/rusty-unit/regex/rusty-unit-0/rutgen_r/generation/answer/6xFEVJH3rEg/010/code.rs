// Answer 0

fn test_fmt_with_multiline_error() {
    use std::fmt;

    struct RegexParseError {
        pattern: String,
        err: String,
    }

    impl fmt::Display for RegexParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let spans = Spans::from_formatter(self);
            if self.pattern.contains('\n') {
                let divider = repeat_char('~', 79);

                writeln!(f, "regex parse error:")?;
                writeln!(f, "{}", divider)?;
                let notated = spans.notate();
                write!(f, "{}", notated)?;
                writeln!(f, "{}", divider)?;
                // Simulate multi-line spans
                let mut notes = vec![format!(
                    "on line {} (column {}) through line {} (column {})",
                    1, 1, 2, 1
                )];
                writeln!(f, "{}", notes.join("\n"))?; // This should trigger an error
                write!(f, "error: {}", self.err)?;
            } else {
                writeln!(f, "regex parse error:")?;
                let notated = spans.notate();
                write!(f, "{}", notated)?;
                write!(f, "error: {}", self.err)?;
            }
            Ok(())
        }
    }

    struct Spans;

    impl Spans {
        fn from_formatter(_error: &RegexParseError) -> Self {
            Spans
        }

        fn notate(&self) -> String {
            "notated information".to_string()
        }

        fn multi_line(&self) -> Vec<Span> {
            vec![Span {
                start: Position { line: 1, column: 1 },
                end: Position { line: 2, column: 1 },
            }]
        }
    }

    struct Position {
        line: usize,
        column: usize,
    }

    struct Span {
        start: Position,
        end: Position,
    }

    fn repeat_char(c: char, n: usize) -> String {
        c.to_string().repeat(n)
    }

    let error = RegexParseError {
        pattern: "a\nb".to_string(),
        err: "syntax error".to_string(),
    };
    let mut output = String::new();
    let result = error.fmt(&mut output);
    
    // Ensure the function doesn't panic and returns Ok
    assert!(result.is_ok());
    // Ensure the output contains the expected content
    assert!(output.contains("regex parse error:"));
    assert!(output.contains("notated information"));
    assert!(output.contains("error: syntax error"));
}

