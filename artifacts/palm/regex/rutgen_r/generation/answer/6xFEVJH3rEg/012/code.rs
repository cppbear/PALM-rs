// Answer 0

fn fmt_test() -> Result<(), std::fmt::Error> {
    use std::fmt::{self, Write}; // Import necessary traits
    
    struct TestError {
        pattern: String,
        err: String,
    }

    impl TestError {
        fn new() -> TestError {
            TestError {
                pattern: String::from("a\nb"), // pattern should contain a newline
                err: String::from("Invalid regex"),
            }
        }
    }
    
    struct Spans {
        multi_line: Vec<Span>,
    }

    impl Spans {
        fn from_formatter(_: &TestError) -> Spans {
            Spans {
                multi_line: vec![Span { start: Position { line: 1, column: 1 }, end: Position { line: 2, column: 1 }}],
            }
        }

        fn notate(&self) -> String {
            "notated span".to_string()
        }
    }

    struct Span {
        start: Position,
        end: Position,
    }

    struct Position {
        line: usize,
        column: usize,
    }

    let error = TestError::new();
    let mut output = String::new();
    let result = {
        let spans = Spans::from_formatter(&error);
        let divider = repeat_char('~', 79);
        
        writeln!(&mut output, "regex parse error:")?;
        writeln!(&mut output, "{}", divider)?;
        let notated = spans.notate();
        write!(&mut output, "{}", notated)?;
        writeln!(&mut output, "{}", divider)?;
        
        if !spans.multi_line.is_empty() {
            let mut notes = vec![];
            for span in &spans.multi_line {
                notes.push(format!(
                    "on line {} (column {}) through line {} (column {})",
                    span.start.line, span.start.column,
                    span.end.line, span.end.column - 1));
            }
            writeln!(&mut output, "{}", notes.join("\n"))?;
        }
        
        write!(&mut output, "error: {}", error.err)?;
        
        Ok(())
    };

    assert!(result.is_ok());
    assert_eq!(output.contains("regex parse error:"), true);
    assert_eq!(output.contains("notated span"), true);
    assert_eq!(output.contains("on line 1 (column 1) through line 2 (column 0)"), true);
    assert!(output.contains("error: Invalid regex"));

    Ok(())
}

