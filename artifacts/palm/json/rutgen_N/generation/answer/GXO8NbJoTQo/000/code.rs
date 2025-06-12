// Answer 0

#[derive(Debug)]
struct Error {
    category: Category,
}

#[derive(Debug, PartialEq)]
enum Category {
    Eof,
    Other,
}

impl Error {
    pub fn classify(&self) -> Category {
        self.category.clone()
    }

    pub fn is_eof(&self) -> bool {
        self.classify() == Category::Eof
    }
}

#[test]
fn test_is_eof_with_eof_category() {
    let error = Error { category: Category::Eof };
    assert!(error.is_eof());
}

#[test]
fn test_is_eof_with_other_category() {
    let error = Error { category: Category::Other };
    assert!(!error.is_eof());
}

