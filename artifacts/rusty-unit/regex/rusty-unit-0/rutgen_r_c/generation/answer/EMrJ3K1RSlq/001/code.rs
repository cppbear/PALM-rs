// Answer 0

#[test]
fn test_add_one_line_span() {
    struct DummyFormatter<'e, E: fmt::Display> {
        _marker: std::marker::PhantomData<&'e E>,
    }

    impl<'p, 'e, E: fmt::Display> Spans<'p> {
        fn with_pattern(pattern: &'p str, line_number_width: usize) -> Self {
            Self {
                pattern,
                line_number_width,
                by_line: vec![Vec::new(); pattern.lines().count()],
                multi_line: Vec::new(),
            }
        }
    }

    let mut spans = Spans::with_pattern("abc\ndef\nghi", 3);
    let span1 = ast::Span {
        start: Position::new(1, 1),
        end: Position::new(1, 4),
    };
    let span2 = ast::Span {
        start: Position::new(2, 1),
        end: Position::new(2, 4),
    };

    spans.add(span1);
    spans.add(span2);

    assert_eq!(spans.by_line[0].len(), 1);
    assert_eq!(spans.by_line[1].len(), 1);
    assert!(spans.multi_line.is_empty());
}

#[test]
#[should_panic]
fn test_add_one_line_span_out_of_bounds() {
    struct DummyFormatter<'e, E: fmt::Display> {
        _marker: std::marker::PhantomData<&'e E>,
    }

    impl<'p, 'e, E: fmt::Display> Spans<'p> {
        fn with_pattern(pattern: &'p str, line_number_width: usize) -> Self {
            Self {
                pattern,
                line_number_width,
                by_line: vec![Vec::new(); pattern.lines().count()],
                multi_line: Vec::new(),
            }
        }
    }

    let mut spans = Spans::with_pattern("abc\ndef\nghi", 3);
    let out_of_bounds_span = ast::Span {
        start: Position::new(4, 1), // This position causes out of bounds panic
        end: Position::new(4, 4),
    };

    spans.add(out_of_bounds_span);
}

#[test]
fn test_add_multi_line_span() {
    struct DummyFormatter<'e, E: fmt::Display> {
        _marker: std::marker::PhantomData<&'e E>,
    }

    impl<'p, 'e, E: fmt::Display> Spans<'p> {
        fn with_pattern(pattern: &'p str, line_number_width: usize) -> Self {
            Self {
                pattern,
                line_number_width,
                by_line: vec![Vec::new(); pattern.lines().count()],
                multi_line: Vec::new(),
            }
        }
    }

    let mut spans = Spans::with_pattern("abc\ndef\nghi", 3);
    let multi_line_span = ast::Span {
        start: Position::new(2, 2),
        end: Position::new(3, 3),
    };

    spans.add(multi_line_span);

    assert!(spans.multi_line.len() == 1);
    assert_eq!(spans.by_line[1].len(), 0);
}

