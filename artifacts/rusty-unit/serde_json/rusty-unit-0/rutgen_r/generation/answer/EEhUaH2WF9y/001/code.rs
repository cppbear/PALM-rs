// Answer 0

#[derive(Debug)]
enum Category {
    Data,
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
fn test_is_data_true() {
    let err = Error { category: Category::Data };
    assert!(err.is_data());
}

#[test]
fn test_is_data_false() {
    let err = Error { category: Category::Other };
    assert!(!err.is_data());
}

#[test]
fn test_is_data_multiple_categories() {
    let err_data = Error { category: Category::Data };
    let err_other = Error { category: Category::Other };
    
    assert!(err_data.is_data());
    assert!(!err_other.is_data());
}

