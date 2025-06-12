// Answer 0

#[cfg(test)]
mod tests {
    use super::*;

    struct MockError {
        category: Category,
    }

    impl MockError {
        fn classify(&self) -> Category {
            self.category.clone()
        }
    }

    #[test]
    fn test_is_syntax_valid_syntax_error() {
        let error = MockError {
            category: Category::Syntax,
        };
        assert!(error.is_syntax());
    }

    #[test]
    fn test_is_syntax_other_error() {
        let error = MockError {
            category: Category::Other,
        };
        assert!(!error.is_syntax());
    }

    #[test]
    fn test_is_syntax_io_error() {
        let error = MockError {
            category: Category::Io,
        };
        assert!(!error.is_syntax());
    }

    #[test]
    fn test_is_syntax_boundary_case() {
        let error = MockError {
            category: Category::Syntax,
        };
        let result = error.is_syntax();
        assert!(result);
    }
}

