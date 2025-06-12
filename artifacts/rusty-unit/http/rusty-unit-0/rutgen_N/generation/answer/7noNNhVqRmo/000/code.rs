// Answer 0

#[derive(Debug)]
struct MyError {
    source: Option<Box<dyn std::error::Error>>,
}

impl std::error::Error for MyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_deref()
    }
}

struct MyErrorWrapper {
    error: MyError,
}

impl MyErrorWrapper {
    fn get_ref(&self) -> &MyError {
        &self.error
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.get_ref().source()
    }
}

#[test]
fn test_source_none() {
    let error_wrapper = MyErrorWrapper {
        error: MyError { source: None },
    };
    assert!(error_wrapper.source().is_none());
}

#[test]
fn test_source_some() {
    let inner_error = MyError {
        source: None,
    };
    let boxed_inner_error: Box<dyn std::error::Error> = Box::new(inner_error);
    let error_wrapper = MyErrorWrapper {
        error: MyError { source: Some(boxed_inner_error) },
    };
    assert!(error_wrapper.source().is_some());
}

