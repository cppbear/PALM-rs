// Answer 0

fn test_fmt_with_multiline_pattern() -> std::fmt::Result {
    struct TestError {
        err: String,
        pattern: String,
    }

    impl TestError {
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

    struct TestSpan {
        start: Position,
        end: Position,
    }

    struct Position {
        line: usize,
        column: usize,
    }

    struct Spans {
        multi_line: Vec<TestSpan>,
    }

    impl Spans {
        fn from_formatter(_error: &TestError) -> Self {
            Self {
                multi_line: vec![
                    TestSpan {
                        start: Position { line: 1, column: 0 },
                        end: Position { line: 2, column: 5 },
                    },
                ],
            }
        }

        fn notate(&self) -> String {
            "Notated error".to_string()
        }
    }

    fn repeat_char(c: char, n: usize) -> String {
        c.to_string().repeat(n)
    }

    let test_error = TestError {
        err: "Some parsing error".to_string(),
        pattern: "abc\ndef".to_string(),
    };

    let result = test_error.fmt(&mut std::fmt::Formatter::new());
    assert_eq!(result, Ok(()));
    result
} 

#[test]
fn test_fmt() {
    let _ = test_fmt_with_multiline_pattern();
}

