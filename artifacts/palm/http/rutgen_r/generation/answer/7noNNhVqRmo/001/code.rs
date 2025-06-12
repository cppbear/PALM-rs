// Answer 0

#[test]
fn test_source_with_none() {
    struct MockError;

    impl std::error::Error for MockError {}

    struct MockSource {
        error: Option<Box<dyn std::error::Error + 'static>>,
    }

    impl MockSource {
        fn new(error: Option<Box<dyn std::error::Error + 'static>>) -> Self {
            Self { error }
        }

        fn get_ref(&self) -> &Self {
            self
        }

        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            self.error.as_deref()
        }
    }

    let source = MockSource::new(None);
    assert!(source.source().is_none());
}

#[test]
fn test_source_with_some() {
    struct MockError;

    impl std::error::Error for MockError {}

    struct MockSource {
        error: Option<Box<dyn std::error::Error + 'static>>,
    }

    impl MockSource {
        fn new(error: Option<Box<dyn std::error::Error + 'static>>) -> Self {
            Self { error }
        }

        fn get_ref(&self) -> &Self {
            self
        }

        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            self.error.as_deref()
        }
    }

    let inner_error = Box::new(MockError);
    let source = MockSource::new(Some(inner_error));
    assert!(source.source().is_some());
}

