// Answer 0

#[test]
fn test_is_io_with_io_error() {
    struct MockError {
        category: Category,
    }

    impl MockError {
        fn classify(&self) -> Category {
            self.category.clone()
        }

        fn is_io(&self) -> bool {
            self.classify() == Category::Io
        }
    }

    #[derive(Clone, PartialEq)]
    enum Category {
        Io,
        Other,
    }

    let io_error = MockError {
        category: Category::Io,
    };

    assert!(io_error.is_io());
}

#[test]
fn test_is_io_with_non_io_error() {
    struct MockError {
        category: Category,
    }

    impl MockError {
        fn classify(&self) -> Category {
            self.category.clone()
        }

        fn is_io(&self) -> bool {
            self.classify() == Category::Io
        }
    }

    #[derive(Clone, PartialEq)]
    enum Category {
        Io,
        Other,
    }

    let non_io_error = MockError {
        category: Category::Other,
    };

    assert!(!non_io_error.is_io());
}

