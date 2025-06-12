// Answer 0

#[derive(PartialEq)]
enum Category {
    Data,
    Syntax,
    Other,
}

struct Error {
    category: Category,
}

impl Error {
    pub fn classify(&self) -> Category {
        self.category.clone()
    }
    
    pub fn is_data(&self) -> bool {
        self.classify() == Category::Data
    }
}

#[test]
fn test_is_data_returns_true_for_data_category() {
    let error = Error { category: Category::Data };
    assert!(error.is_data());
}

#[test]
fn test_is_data_returns_false_for_syntax_category() {
    let error = Error { category: Category::Syntax };
    assert!(!error.is_data());
}

#[test]
fn test_is_data_returns_false_for_other_category() {
    let error = Error { category: Category::Other };
    assert!(!error.is_data());
}

