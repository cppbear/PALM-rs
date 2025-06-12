// Answer 0

#[test]
fn test_notate_line_empty_spans() {
    struct DummySpan {
        start: Position,
        end: Position,
    }

    struct DummyAst {
        span: DummySpan,
    }

    struct DummyFormatter<'e, E: fmt::Display> {
        _marker: std::marker::PhantomData<&'e E>,
    }

    let pattern = "abc";
    let line_number_width = 2;
    let by_line: Vec<Vec<DummyAst>> = vec![vec![]]; // Entry for line 0 is empty
    let multi_line: Vec<DummyAst> = vec![];

    let spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };

    assert_eq!(spans.notate_line(0), None); // Should return None for empty spans
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_notate_line_out_of_bounds_index() {
    struct DummySpan {
        start: Position,
        end: Position,
    }

    struct DummyAst {
        span: DummySpan,
    }

    struct DummyFormatter<'e, E: fmt::Display> {
        _marker: std::marker::PhantomData<&'e E>,
    }

    let pattern = "abc";
    let line_number_width = 2;
    let by_line: Vec<Vec<DummyAst>> = vec![vec![]]; // Entry for line 0 is empty
    let multi_line: Vec<DummyAst> = vec![];

    let spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };

    let _ = spans.notate_line(1); // This should panic due to out-of-bounds access
}

