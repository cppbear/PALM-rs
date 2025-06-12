// Answer 0

#[test]
fn test_from_formatter_single_line_no_aux() {
    struct TestFormatter<'e> {
        pattern: &'e str,
        span: Span,
        aux_span: Option<Span>,
    }

    struct Span {
        start: usize,
        end: usize,
    }

    struct Spans<'p> {
        pattern: &'p str,
        line_number_width: usize,
        by_line: Vec<Vec<Span>>,
        multi_line: Vec<Span>,
    }

    impl<'p> Spans<'p> {
        fn add(&mut self, span: Span) {
            self.by_line[0].push(span);
        }
    }

    let pattern = "test pattern";
    let span = Span { start: 0, end: 11 };
    let formatter = TestFormatter {
        pattern,
        span,
        aux_span: None,
    };
    
    let spans = from_formatter(&formatter);
    
    assert_eq!(spans.line_number_width, 0);
    assert_eq!(spans.by_line.len(), 1);
    assert_eq!(spans.by_line[0].len(), 1);
}

#[test]
fn test_from_formatter_multi_line_with_aux() {
    struct TestFormatter<'e> {
        pattern: &'e str,
        span: Span,
        aux_span: Option<Span>,
    }

    struct Span {
        start: usize,
        end: usize,
    }

    struct Spans<'p> {
        pattern: &'p str,
        line_number_width: usize,
        by_line: Vec<Vec<Span>>,
        multi_line: Vec<Span>,
    }

    impl<'p> Spans<'p> {
        fn add(&mut self, span: Span) {
            self.by_line[0].push(span);
        }
    }

    let pattern = "first line\nsecond line\n";
    let span = Span { start: 0, end: 11 };
    let aux_span = Span { start: 12, end: 25 };
    let formatter = TestFormatter {
        pattern,
        span,
        aux_span: Some(aux_span),
    };

    let spans = from_formatter(&formatter);
    
    assert_eq!(spans.line_number_width, 2);
    assert_eq!(spans.by_line.len(), 3);
    assert_eq!(spans.by_line[0].len(), 1);
    assert_eq!(spans.by_line[1].len(), 0);
    assert_eq!(spans.by_line[2].len(), 0);
}

