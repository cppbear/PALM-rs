// Answer 0

#[derive(Debug)]
struct LineNumberPadded {
    line_number_width: usize,
}

impl LineNumberPadded {
    fn left_pad_line_number(&self, n: usize) -> String {
        let n = n.to_string();
        let pad = self.line_number_width.checked_sub(n.len()).unwrap();
        let mut result = repeat_char(' ', pad);
        result.push_str(&n);
        result
    }
}

fn repeat_char(c: char, count: usize) -> String {
    std::iter::repeat(c).take(count).collect()
}

#[test]
fn test_left_pad_line_number_small() {
    let line_number_padded = LineNumberPadded { line_number_width: 4 };
    let result = line_number_padded.left_pad_line_number(3);
    assert_eq!(result, "   3");
}

#[test]
fn test_left_pad_line_number_equal() {
    let line_number_padded = LineNumberPadded { line_number_width: 4 };
    let result = line_number_padded.left_pad_line_number(42);
    assert_eq!(result, " 42");
}

#[test]
fn test_left_pad_line_number_large() {
    let line_number_padded = LineNumberPadded { line_number_width: 4 };
    let result = line_number_padded.left_pad_line_number(123);
    assert_eq!(result, "123");
}

#[test]
fn test_left_pad_line_number_zero_width() {
    let line_number_padded = LineNumberPadded { line_number_width: 0 };
    let result = line_number_padded.left_pad_line_number(1);
    assert_eq!(result, "1");
}

