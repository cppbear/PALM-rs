// Answer 0

#[test]
fn test_add_single_line_span() {
    struct Dummy {
        by_line: Vec<Vec<ast::Span>>,
        multi_line: Vec<ast::Span>,
    }

    impl Dummy {
        fn new() -> Self {
            Dummy {
                by_line: vec![Vec::new(); 10], // Initialize 10 lines
                multi_line: Vec::new(),
            }
        }

        fn add(&mut self, span: ast::Span) {
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

    struct MockSpan {
        start: ast::Position,
        end: ast::Position,
    }

    impl MockSpan {
        fn is_one_line(&self) -> bool {
            self.start.line == self.end.line
        }
    }

    let mut dummy = Dummy::new();
    let span1 = MockSpan { start: ast::Position { line: 1 }, end: ast::Position { line: 1 }};
    let span2 = MockSpan { start: ast::Position { line: 1 }, end: ast::Position { line: 1 }};

    // Add two spans to the first line
    dummy.add(span1);
    dummy.add(span2);

    assert_eq!(dummy.by_line[0].len(), 2); // Check that two spans were added
}

#[test]
#[should_panic]
fn test_add_single_line_span_out_of_bounds() {
    struct Dummy {
        by_line: Vec<Vec<ast::Span>>,
        multi_line: Vec<ast::Span>,
    }

    impl Dummy {
        fn new() -> Self {
            Dummy {
                by_line: vec![Vec::new(); 10], // Initialize 10 lines
                multi_line: Vec::new(),
            }
        }

        fn add(&mut self, span: ast::Span) {
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

    struct MockSpan {
        start: ast::Position,
        end: ast::Position,
    }

    impl MockSpan {
        fn is_one_line(&self) -> bool {
            self.start.line == self.end.line
        }
    }

    let mut dummy = Dummy::new();
    let out_of_bounds_span = MockSpan { start: ast::Position { line: 11 }, end: ast::Position { line: 11 }};
    
    // This should trigger a panic due to accessing an out-of-bounds index
    dummy.add(out_of_bounds_span);
}

