// Answer 0

#[test]
fn test_add_one_line_span() {
    struct Span {
        start: LineColumn,
        end: LineColumn,
    }

    struct LineColumn {
        line: usize,
        column: usize,
    }

    struct SpanContainer {
        by_line: Vec<Vec<Span>>,
        multi_line: Vec<Span>,
    }

    impl SpanContainer {
        fn new() -> Self {
            SpanContainer {
                by_line: vec![vec![]; 10], // 10 lines for the sake of this example
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

    let mut container = SpanContainer::new();
    let span = Span {
        start: LineColumn { line: 1, column: 1 },
        end: LineColumn { line: 1, column: 5 },
    };

    container.add(span);

    assert_eq!(container.by_line[0].len(), 1);
}

#[test]
fn test_add_multi_line_span() {
    struct Span {
        start: LineColumn,
        end: LineColumn,
    }

    struct LineColumn {
        line: usize,
        column: usize,
    }

    struct SpanContainer {
        by_line: Vec<Vec<Span>>,
        multi_line: Vec<Span>,
    }

    impl SpanContainer {
        fn new() -> Self {
            SpanContainer {
                by_line: vec![vec![]; 10],
                multi_line: vec![],
            }
        }

        fn add(&mut self, span: Span) {
            if span.is_one_line() {
                let i = span.start.line - 1;
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

    let mut container = SpanContainer::new();
    let span = Span {
        start: LineColumn { line: 1, column: 1 },
        end: LineColumn { line: 3, column: 5 },
    };

    container.add(span);

    assert_eq!(container.multi_line.len(), 1);
}

#[test]
fn test_add_multiple_one_line_spans() {
    struct Span {
        start: LineColumn,
        end: LineColumn,
    }

    struct LineColumn {
        line: usize,
        column: usize,
    }

    struct SpanContainer {
        by_line: Vec<Vec<Span>>,
        multi_line: Vec<Span>,
    }

    impl SpanContainer {
        fn new() -> Self {
            SpanContainer {
                by_line: vec![vec![]; 10],
                multi_line: vec![],
            }
        }

        fn add(&mut self, span: Span) {
            if span.is_one_line() {
                let i = span.start.line - 1;
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

    let mut container = SpanContainer::new();
    let span1 = Span { start: LineColumn { line: 1, column: 1 }, end: LineColumn { line: 1, column: 5 } };
    let span2 = Span { start: LineColumn { line: 1, column: 3 }, end: LineColumn { line: 1, column: 7 } };

    container.add(span1);
    container.add(span2);

    assert_eq!(container.by_line[0].len(), 2);
}

