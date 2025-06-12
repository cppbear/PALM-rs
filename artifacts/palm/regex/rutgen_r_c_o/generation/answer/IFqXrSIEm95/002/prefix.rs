// Answer 0

#[test]
fn test_description_unsupported_look_around() {
    use std::error::Error;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestError {
        kind: ErrorKind,
    }

    impl TestError {
        fn description(&self) -> &str {
            use self::ErrorKind::*;
            match self.kind {
                UnsupportedLookAround => "look-around is not supported",
                _ => unreachable!(),
            }
        }
    }

    let error = TestError {
        kind: ErrorKind::UnsupportedLookAround,
    };

    let _ = error.description();
}

