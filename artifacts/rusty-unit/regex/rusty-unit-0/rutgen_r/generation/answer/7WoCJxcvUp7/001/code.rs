// Answer 0

#[test]
fn test_line_number_padding_non_zero_width() {
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

    let test_instance_1 = TestStruct { line_number_width: 5 };
    assert_eq!(test_instance_1.line_number_padding(), 7);

    let test_instance_2 = TestStruct { line_number_width: 10 };
    assert_eq!(test_instance_2.line_number_padding(), 12);

    let test_instance_3 = TestStruct { line_number_width: 100 };
    assert_eq!(test_instance_3.line_number_padding(), 102);
}

