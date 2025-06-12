// Answer 0

#[test]
fn test_is_io_with_io_error() {
    struct MockErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }
    
    struct MockError {
        err: Box<MockErrorImpl>,
    }
    
    impl MockError {
        pub fn new_io_error(line: usize, column: usize) -> Self {
            Self {
                err: Box::new(MockErrorImpl {
                    code: ErrorCode::Io("test_io_error".to_string()),
                    line,
                    column,
                }),
            }
        }

        pub fn classify(&self) -> Category {
            match self.err.code {
                ErrorCode::Io(_) => Category::Io,
                _ => Category::Data, // Default for test logic
            }
        }
        
        pub fn is_io(&self) -> bool {
            self.classify() == Category::Io
        }
    }

    let error = MockError::new_io_error(1, 5);
    assert!(error.is_io());
}

#[test]
fn test_is_io_with_non_io_error() {
    struct MockErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }
    
    struct MockError {
        err: Box<MockErrorImpl>,
    }
    
    impl MockError {
        pub fn new_data_error(line: usize, column: usize) -> Self {
            Self {
                err: Box::new(MockErrorImpl {
                    code: ErrorCode::Message("test_message".to_string()),
                    line,
                    column,
                }),
            }
        }

        pub fn classify(&self) -> Category {
            match self.err.code {
                ErrorCode::Io(_) => Category::Io,
                _ => Category::Data, // Default for test logic
            }
        }
        
        pub fn is_io(&self) -> bool {
            self.classify() == Category::Io
        }
    }

    let error = MockError::new_data_error(2, 10);
    assert!(!error.is_io());
}

