// Answer 0

#[test]
fn test_add_multi_line_span() {
    struct Span {
        start: LineInfo,
        end: LineInfo,
    }

    struct LineInfo {
        line: usize,
    }

    struct SpanSequence {
        by_line: Vec<Vec<Span>>,
        multi_line: Vec<Span>,
    }

    impl SpanSequence {
        fn new() -> Self {
            SpanSequence {
                by_line: vec![vec![]; 10], // Assuming a max of 10 lines for simplicity
                multi_line: vec![],
            }
        }
        
        fn add(&mut self, span: Span) {
            if span.is_one_line() {
                let i = span.start.line - 1; // because lines are 1-indexed
                self.by_line[i].push(span);
                self.by_line[i].sort();
            } else {
                self.multi_line.push(span);
                self.multi_line.sort();
            }
        }
    }

    impl Span {
        fn is_one_line(&self) -> bool {
            self.start.line == self.end.line
        }
    }

    let mut sequence = SpanSequence::new();

    // Test input with a multi-line span
    let multi_line_span = Span {
        start: LineInfo { line: 1 },
        end: LineInfo { line: 3 },
    };

    sequence.add(multi_line_span);

    assert_eq!(sequence.multi_line.len(), 1);
    assert_eq!(sequence.multi_line[0].start.line, 1);
    assert_eq!(sequence.multi_line[0].end.line, 3);

    // Add another multi-line span that should be sorted
    let another_multi_line_span = Span {
        start: LineInfo { line: 2 },
        end: LineInfo { line: 5 },
    };

    sequence.add(another_multi_line_span);

    assert_eq!(sequence.multi_line.len(), 2);
    assert_eq!(sequence.multi_line[0].start.line, 1);
    assert_eq!(sequence.multi_line[1].start.line, 2);
}

