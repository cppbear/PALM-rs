// Answer 0

#[test]
fn test_left_pad_line_number_no_padding() {
    let spans = Spans {
        pattern: "abc",
        line_number_width: 0,
        by_line: vec![vec![]],
        multi_line: vec![],
    };
    let result = spans.left_pad_line_number(1);
    assert_eq!(result, "1");
}

#[test]
fn test_left_pad_line_number_with_padding() {
    let spans = Spans {
        pattern: "abc",
        line_number_width: 3,
        by_line: vec![vec![]],
        multi_line: vec![],
    };
    let result = spans.left_pad_line_number(1);
    assert_eq!(result, "  1");
}

#[test]
fn test_left_pad_line_number_exact_width() {
    let spans = Spans {
        pattern: "abc",
        line_number_width: 2,
        by_line: vec![vec![]],
        multi_line: vec![],
    };
    let result = spans.left_pad_line_number(12);
    assert_eq!(result, "12");
}

#[test]
fn test_left_pad_line_number_large_number() {
    let spans = Spans {
        pattern: "abc",
        line_number_width: 5,
        by_line: vec![vec![]],
        multi_line: vec![],
    };
    let result = spans.left_pad_line_number(123456);
    assert_eq!(result, "123456");
}

