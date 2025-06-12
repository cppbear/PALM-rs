// Answer 0

#[test]
fn test_byte_offset() {
    struct TestStruct {
        start_of_line: usize,
        col: usize,
    }
    
    impl TestStruct {
        fn byte_offset(&self) -> usize {
            self.start_of_line + self.col
        }
    }

    let instance = TestStruct { start_of_line: 10, col: 5 };
    let result = instance.byte_offset();
    assert_eq!(result, 15);
}

#[test]
fn test_byte_offset_with_zero_col() {
    struct TestStruct {
        start_of_line: usize,
        col: usize,
    }
    
    impl TestStruct {
        fn byte_offset(&self) -> usize {
            self.start_of_line + self.col
        }
    }

    let instance = TestStruct { start_of_line: 20, col: 0 };
    let result = instance.byte_offset();
    assert_eq!(result, 20);
}

#[test]
fn test_byte_offset_with_zero_start_of_line() {
    struct TestStruct {
        start_of_line: usize,
        col: usize,
    }
    
    impl TestStruct {
        fn byte_offset(&self) -> usize {
            self.start_of_line + self.col
        }
    }

    let instance = TestStruct { start_of_line: 0, col: 25 };
    let result = instance.byte_offset();
    assert_eq!(result, 25);
}

#[test]
fn test_byte_offset_with_large_values() {
    struct TestStruct {
        start_of_line: usize,
        col: usize,
    }
    
    impl TestStruct {
        fn byte_offset(&self) -> usize {
            self.start_of_line + self.col
        }
    }

    let instance = TestStruct { start_of_line: usize::MAX - 1, col: 1 };
    let result = instance.byte_offset();
    assert_eq!(result, usize::MAX);
}

