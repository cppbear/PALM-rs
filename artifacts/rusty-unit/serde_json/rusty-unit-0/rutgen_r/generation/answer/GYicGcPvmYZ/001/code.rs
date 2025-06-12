// Answer 0

#[derive(Debug)]
enum Category {
    Io,
    Parse,
    Other,
}

struct Error {
    category: Category,
}

impl Error {
    fn classify(&self) -> Category {
        self.category.clone()
    }

    pub fn is_io(&self) -> bool {
        self.classify() == Category::Io
    }
}

#[test]
fn test_is_io_when_category_is_io() {
    let error = Error { category: Category::Io };
    assert!(error.is_io());
}

#[test]
fn test_is_io_when_category_is_parse() {
    let error = Error { category: Category::Parse };
    assert!(!error.is_io());
}

#[test]
fn test_is_io_when_category_is_other() {
    let error = Error { category: Category::Other };
    assert!(!error.is_io());
}

#[test]
fn test_is_io_with_empty_struct() {
    struct EmptyError;
    
    impl EmptyError {
        fn classify(&self) -> Category {
            Category::Other
        }

        pub fn is_io(&self) -> bool {
            self.classify() == Category::Io
        }
    }

    let error = EmptyError;
    assert!(!error.is_io());
}

#[test]
#[should_panic]
fn test_is_io_should_panic_on_unexpected_category() {
    struct UninitializedError;

    impl UninitializedError {
        fn classify(&self) -> Category {
            panic!("Unexpected category access!");
        }

        pub fn is_io(&self) -> bool {
            self.classify() == Category::Io
        }
    }

    let error = UninitializedError;
    let _ = error.is_io();
}

