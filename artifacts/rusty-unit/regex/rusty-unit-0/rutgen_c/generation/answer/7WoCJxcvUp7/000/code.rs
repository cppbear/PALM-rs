// Answer 0

#[test]
fn test_line_number_padding_zero_width() {
    struct SpansTest<'p> {
        pattern: &'p str,
        line_number_width: usize,
        by_line: Vec<Vec<ast::Span>>,
        multi_line: Vec<ast::Span>,
    }

    let spans = SpansTest {
        pattern: "a",
        line_number_width: 0,
        by_line: vec![vec![]],
        multi_line: vec![],
    };

    assert_eq!(spans.line_number_padding(), 4);
}

#[test]
fn test_line_number_padding_non_zero_width() {
    struct SpansTest<'p> {
        pattern: &'p str,
        line_number_width: usize,
        by_line: Vec<Vec<ast::Span>>,
        multi_line: Vec<ast::Span>,
    }

    let spans = SpansTest {
        pattern: "a\nb",
        line_number_width: 3,
        by_line: vec![vec![], vec![]],
        multi_line: vec![],
    };

    assert_eq!(spans.line_number_padding(), 5);
}

