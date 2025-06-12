// Answer 0

#[derive(Debug)]
enum Category {
    Io,
    Other,
}

#[derive(Debug)]
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
fn test_is_io_with_io_category() {
    let error = Error {
        category: Category::Io,
    };
    assert!(error.is_io());
}

#[test]
fn test_is_io_with_other_category() {
    let error = Error {
        category: Category::Other,
    };
    assert!(!error.is_io());
}

