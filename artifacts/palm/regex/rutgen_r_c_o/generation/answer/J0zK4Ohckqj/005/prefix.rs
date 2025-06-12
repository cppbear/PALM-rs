// Answer 0

#[test]
fn test_notate_empty_pattern() {
    let spans = Spans {
        pattern: "",
        line_number_width: 0,
        by_line: vec![vec![]],
        multi_line: vec![],
    };
    spans.notate();
}

#[test]
fn test_notate_single_line_no_width() {
    let spans = Spans {
        pattern: "abc",
        line_number_width: 0,
        by_line: vec![vec![]],
        multi_line: vec![],
    };
    spans.notate();
}

#[test]
fn test_notate_two_lines_with_width() {
    let spans = Spans {
        pattern: "abc\ndef",
        line_number_width: 2,
        by_line: vec![vec![], vec![]],
        multi_line: vec![],
    };
    spans.notate();
}

#[test]
fn test_notate_three_lines_with_width() {
    let spans = Spans {
        pattern: "line1\nline2\nline3",
        line_number_width: 3,
        by_line: vec![vec![], vec![], vec![]],
        multi_line: vec![],
    };
    spans.notate();
}

#[test]
fn test_notate_two_lines_small_width() {
    let spans = Spans {
        pattern: "line1\nline2",
        line_number_width: 1,
        by_line: vec![vec![], vec![]],
        multi_line: vec![],
    };
    spans.notate();
}

#[test]
fn test_notate_five_lines_large_width() {
    let spans = Spans {
        pattern: "line1\nline2\nline3\nline4\nline5",
        line_number_width: 5,
        by_line: vec![vec![], vec![], vec![], vec![], vec![]],
        multi_line: vec![],
    };
    spans.notate();
}

#[test]
fn test_notate_four_lines_with_equal_contents() {
    let spans = Spans {
        pattern: "line\nline\nline\nline",
        line_number_width: 4,
        by_line: vec![vec![], vec![], vec![], vec![]],
        multi_line: vec![],
    };
    spans.notate();
}

#[test]
fn test_notate_three_lines_with_width() {
    let spans = Spans {
        pattern: "a\nb\nc",
        line_number_width: 6,
        by_line: vec![vec![], vec![], vec![]],
        multi_line: vec![],
    };
    spans.notate();
}

#[test]
fn test_notate_two_lines_no_width() {
    let spans = Spans {
        pattern: "1\n2",
        line_number_width: 0,
        by_line: vec![vec![], vec![]],
        multi_line: vec![],
    };
    spans.notate();
}

