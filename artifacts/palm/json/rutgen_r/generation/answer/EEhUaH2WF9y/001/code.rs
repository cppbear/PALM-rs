// Answer 0

#[test]
fn test_is_data_with_data_category() {
    struct TestError {
        category: Category,
    }

    impl TestError {
        fn classify(&self) -> Category {
            self.category.clone()
        }
        
        fn is_data(&self) -> bool {
            self.classify() == Category::Data
        }
    }

    let error = TestError { category: Category::Data };
    assert_eq!(error.is_data(), true);
}

#[test]
fn test_is_data_with_non_data_category() {
    struct TestError {
        category: Category,
    }

    impl TestError {
        fn classify(&self) -> Category {
            self.category.clone()
        }
        
        fn is_data(&self) -> bool {
            self.classify() == Category::Data
        }
    }

    let error = TestError { category: Category::Other }; // Assuming there's a non-data category
    assert_eq!(error.is_data(), false);
}

#[test]
fn test_is_data_with_default_category() {
    struct TestError {
        category: Category,
    }

    impl TestError {
        fn classify(&self) -> Category {
            self.category.clone()
        }
        
        fn is_data(&self) -> bool {
            self.classify() == Category::Data
        }
    }

    let error = TestError { category: Category::Default }; // Assuming a default category is available
    assert_eq!(error.is_data(), false);
}

