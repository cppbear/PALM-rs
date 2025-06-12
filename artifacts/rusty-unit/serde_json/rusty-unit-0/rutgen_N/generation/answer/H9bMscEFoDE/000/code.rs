// Answer 0

#[derive(Debug)]
struct Error {
    category: Category,
}

#[derive(Debug, PartialEq)]
enum Category {
    Syntax,
    Other,
}

impl Error {
    pub fn classify(&self) -> Category {
        self.category.clone()
    }

    pub fn is_syntax(&self) -> bool {
        self.classify() == Category::Syntax
    }
}

#[test]
fn test_is_syntax_true() {
    let error = Error {
        category: Category::Syntax,
    };
    assert_eq!(error.is_syntax(), true);
}

#[test]
fn test_is_syntax_false() {
    let error = Error {
        category: Category::Other,
    };
    assert_eq!(error.is_syntax(), false);
}

