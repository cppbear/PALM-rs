// Answer 0

#[test]
fn test_notate_with_spans() {
    use ast::Span;

    struct TestSpans<'p> {
        pattern: &'p str,
        line_number_width: usize,
        by_line: Vec<Vec<Span>>,
        multi_line: Vec<Span>,
    }

    impl<'p> TestSpans<'p> {
        fn new(
            pattern: &'p str,
            line_number_width: usize,
            by_line: Vec<Vec<Span>>,
            multi_line: Vec<Span>,
        ) -> Self {
            TestSpans {
                pattern,
                line_number_width,
                by_line,
                multi_line,
            }
        }

        fn left_pad_line_number(&self, n: usize) -> String {
            let n = n.to_string();
            let pad = self.line_number_width.checked_sub(n.len()).unwrap_or(0);
            let mut result = " ".repeat(pad);
            result.push_str(&n);
            result
        }

        fn notate_line(&self, i: usize) -> Option<String> {
            let spans = &self.by_line[i];
            if spans.is_empty() {
                return None;
            }
            let mut notes = String::new();
            for _ in 0..self.line_number_width {
                notes.push(' ');
            }
            let mut pos = 0;
            for span in spans {
                for _ in pos..(span.start.column - 1) {
                    notes.push(' ');
                    pos += 1;
                }
                let note_len = span.end.column.saturating_sub(span.start.column);
                for _ in 0..cmp::max(1, note_len) {
                    notes.push('^');
                    pos += 1;
                }
            }
            Some(notes)
        }

        fn notate(&self) -> String {
            let mut notated = String::new();
            for (i, line) in self.pattern.lines().enumerate() {
                if self.line_number_width > 0 {
                    notated.push_str(&self.left_pad_line_number(i + 1));
                    notated.push_str(": ");
                } else {
                    notated.push_str("    ");
                }
                notated.push_str(line);
                notated.push('\n');
                if let Some(notes) = self.notate_line(i) {
                    notated.push_str(&notes);
                    notated.push('\n');
                }
            }
            notated
        }
    }

    // Example test case with spans
    let pattern = "hello world\nthis is a test";
    let line_number_width = 2;
    let by_line = vec![
        vec![Span { start: Position { column: 7 }, end: Position { column: 8 }}], // ^ for 'w'
        vec![Span { start: Position { column: 6 }, end: Position { column: 8 }}], // ^ for 'is'
    ];
    let multi_line: Vec<Span> = vec![];

    let spans = TestSpans::new(pattern, line_number_width, by_line, multi_line);
    
    let expected_notation = " 1: hello world\n  7:       ^\n 2: this is a test\n  6:       ^^\n";
    
    assert_eq!(spans.notate(), expected_notation);
}

#[test]
fn test_notate_with_no_spans() {
    use ast::Span;

    struct TestSpans<'p> {
        pattern: &'p str,
        line_number_width: usize,
        by_line: Vec<Vec<Span>>,
        multi_line: Vec<Span>,
    }

    impl<'p> TestSpans<'p> {
        fn new(
            pattern: &'p str,
            line_number_width: usize,
            by_line: Vec<Vec<Span>>,
            multi_line: Vec<Span>,
        ) -> Self {
            TestSpans {
                pattern,
                line_number_width,
                by_line,
                multi_line,
            }
        }

        fn left_pad_line_number(&self, n: usize) -> String {
            let n = n.to_string();
            let pad = self.line_number_width.checked_sub(n.len()).unwrap_or(0);
            let mut result = " ".repeat(pad);
            result.push_str(&n);
            result
        }

        fn notate_line(&self, i: usize) -> Option<String> {
            let spans = &self.by_line[i];
            if spans.is_empty() {
                return None;
            }
            let mut notes = String::new();
            for _ in 0..self.line_number_width {
                notes.push(' ');
            }
            let mut pos = 0;
            for span in spans {
                for _ in pos..(span.start.column - 1) {
                    notes.push(' ');
                    pos += 1;
                }
                let note_len = span.end.column.saturating_sub(span.start.column);
                for _ in 0..cmp::max(1, note_len) {
                    notes.push('^');
                    pos += 1;
                }
            }
            Some(notes)
        }

        fn notate(&self) -> String {
            let mut notated = String::new();
            for (i, line) in self.pattern.lines().enumerate() {
                if self.line_number_width > 0 {
                    notated.push_str(&self.left_pad_line_number(i + 1));
                    notated.push_str(": ");
                } else {
                    notated.push_str("    ");
                }
                notated.push_str(line);
                notated.push('\n');
                if let Some(notes) = self.notate_line(i) {
                    notated.push_str(&notes);
                    notated.push('\n');
                }
            }
            notated
        }
    }

    // Example test case with no spans
    let pattern = "single line pattern";
    let line_number_width = 2;
    let by_line = vec![vec![]]; // No spans for the line
    let multi_line: Vec<Span> = vec![];

    let spans = TestSpans::new(pattern, line_number_width, by_line, multi_line);
    
    let expected_notation = " 1: single line pattern\n";
    
    assert_eq!(spans.notate(), expected_notation);
}

