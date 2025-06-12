// Answer 0

#[derive(Debug)]
struct LineNumberWidth {
    line_number_width: usize,
}

impl LineNumberWidth {
    fn left_pad_line_number(&self, n: usize) -> String {
        let n = n.to_string();
        let pad = self.line_number_width.checked_sub(n.len()).unwrap();
        let mut result = repeat_char(' ', pad);
        result.push_str(&n);
        result
    }
}

fn repeat_char(c: char, times: usize) -> String {
    c.to_string().repeat(times)
}

#[test]
fn test_left_pad_line_number_zero() {
    let lw = LineNumberWidth { line_number_width: 5 };
    assert_eq!(lw.left_pad_line_number(0), "    0");
}

#[test]
fn test_left_pad_line_number_single_digit() {
    let lw = LineNumberWidth { line_number_width: 5 };
    assert_eq!(lw.left_pad_line_number(3), "    3");
}

#[test]
fn test_left_pad_line_number_two_digits() {
    let lw = LineNumberWidth { line_number_width: 5 };
    assert_eq!(lw.left_pad_line_number(12), "   12");
}

#[test]
fn test_left_pad_line_number_three_digits() {
    let lw = LineNumberWidth { line_number_width: 5 };
    assert_eq!(lw.left_pad_line_number(123), "  123");
}

#[test]
fn test_left_pad_line_number_exact_width() {
    let lw = LineNumberWidth { line_number_width: 5 };
    assert_eq!(lw.left_pad_line_number(12345), "12345");
}

#[test]
#[should_panic]
fn test_left_pad_line_number_too_large() {
    let lw = LineNumberWidth { line_number_width: 5 };
    lw.left_pad_line_number(123456); // should panic due to checked_sub underflow
}

