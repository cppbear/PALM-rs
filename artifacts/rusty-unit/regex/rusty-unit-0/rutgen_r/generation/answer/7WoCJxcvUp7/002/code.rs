// Answer 0

#[derive(Default)]
struct LineNumberPadding {
    line_number_width: usize,
}

impl LineNumberPadding {
    fn line_number_padding(&self) -> usize {
        if self.line_number_width == 0 {
            4
        } else {
            2 + self.line_number_width
        }
    }
}

#[test]
fn test_line_number_padding_with_zero_width() {
    let instance = LineNumberPadding { line_number_width: 0 };
    assert_eq!(instance.line_number_padding(), 4);
}

#[test]
fn test_line_number_padding_with_non_zero_width() {
    let instance = LineNumberPadding { line_number_width: 5 };
    assert_eq!(instance.line_number_padding(), 7);
}

#[test]
fn test_line_number_padding_with_other_non_zero_width() {
    let instance = LineNumberPadding { line_number_width: 1 };
    assert_eq!(instance.line_number_padding(), 3);
}

