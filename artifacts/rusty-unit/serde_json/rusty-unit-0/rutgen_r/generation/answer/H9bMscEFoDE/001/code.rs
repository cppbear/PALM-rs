// Answer 0

#[test]
fn test_is_syntax_valid_syntax_error() {
    struct TestError {
        category: Category,
    }
    
    impl TestError {
        fn classify(&self) -> Category {
            self.category.clone()
        }
        
        fn is_syntax(&self) -> bool {
            self.classify() == Category::Syntax
        }
    }

    let error = TestError { category: Category::Syntax };
    assert!(error.is_syntax());
}

#[test]
fn test_is_syntax_invalid_category() {
    struct TestError {
        category: Category,
    }
    
    impl TestError {
        fn classify(&self) -> Category {
            self.category.clone()
        }
        
        fn is_syntax(&self) -> bool {
            self.classify() == Category::Syntax
        }
    }

    let error = TestError { category: Category::Other };
    assert!(!error.is_syntax());
}

#[test]
fn test_is_syntax_edge_case() {
    struct TestError {
        category: Category,
    }
    
    impl TestError {
        fn classify(&self) -> Category {
            self.category.clone()
        }
        
        fn is_syntax(&self) -> bool {
            self.classify() == Category::Syntax
        }
    }

    let error = TestError { category: Category::Syntax };
    assert!(error.is_syntax());

    let another_error = TestError { category: Category::Syntax };
    assert!(another_error.is_syntax());
}

