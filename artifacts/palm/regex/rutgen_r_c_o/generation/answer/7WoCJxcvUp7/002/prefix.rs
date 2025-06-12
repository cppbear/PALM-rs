// Answer 0

#[test]
fn test_line_number_padding_zero() {
    struct DummyFormatter<'e, E> {
        _marker: std::marker::PhantomData<&'e E>,
    }

    let pattern = "example pattern";
    let line_number_width = 0;
    let by_line: Vec<Vec<ast::Span>> = vec![vec![]];
    let multi_line: Vec<ast::Span> = vec![];

    let spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };

    spans.line_number_padding();
}

