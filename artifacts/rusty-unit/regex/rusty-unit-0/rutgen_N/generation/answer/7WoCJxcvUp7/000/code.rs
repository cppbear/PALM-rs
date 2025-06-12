// Answer 0

#[derive(Debug)]
struct TestStruct {
    line_number_width: usize,
}

impl TestStruct {
    fn line_number_padding(&self) -> usize {
        if self.line_number_width == 0 {
            4
        } else {
            2 + self.line_number_width
        }
    }
}

#[test]
fn test_line_number_padding_zero_width() {
    let test_instance = TestStruct { line_number_width: 0 };
    assert_eq!(test_instance.line_number_padding(), 4);
}

#[test]
fn test_line_number_padding_non_zero_width() {
    let test_instance = TestStruct { line_number_width: 5 };
    assert_eq!(test_instance.line_number_padding(), 7);
}

#[test]
fn test_line_number_padding_another_non_zero_width() {
    let test_instance = TestStruct { line_number_width: 10 };
    assert_eq!(test_instance.line_number_padding(), 12);
}

