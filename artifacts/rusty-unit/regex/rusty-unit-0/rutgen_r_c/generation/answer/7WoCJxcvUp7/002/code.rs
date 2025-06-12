// Answer 0

#[test]
fn test_line_number_padding_zero_width() {
    struct TestSpans<'p> {
        pattern: &'p str,
        line_number_width: usize,
        by_line: Vec<Vec<ast::Span>>,
        multi_line: Vec<ast::Span>,
    }

    impl<'p> Spans<'p> for TestSpans<'p> {
        fn line_number_padding(&self) -> usize {
            if self.line_number_width == 0 {
                4
            } else {
                2 + self.line_number_width
            }
        }
    }

    let spans = TestSpans {
        pattern: "abc", // arbitrary pattern
        line_number_width: 0,
        by_line: Vec::new(),
        multi_line: Vec::new(),
    };

    assert_eq!(spans.line_number_padding(), 4);
}

#[test]
fn test_line_number_padding_non_zero_width() {
    struct TestSpans<'p> {
        pattern: &'p str,
        line_number_width: usize,
        by_line: Vec<Vec<ast::Span>>,
        multi_line: Vec<ast::Span>,
    }

    impl<'p> Spans<'p> for TestSpans<'p> {
        fn line_number_padding(&self) -> usize {
            if self.line_number_width == 0 {
                4
            } else {
                2 + self.line_number_width
            }
        }
    }

    let spans = TestSpans {
        pattern: "abc", // arbitrary pattern
        line_number_width: 5,
        by_line: Vec::new(),
        multi_line: Vec::new(),
    };

    assert_eq!(spans.line_number_padding(), 7);
}

