// Answer 0

#[derive(Debug)]
struct Unsupported;

mod ser {
    #[derive(Debug)]
    pub struct Error {
        message: String,
    }

    impl Error {
        pub fn custom(args: std::fmt::Arguments) -> Self {
            Error {
                message: args.to_string(),
            }
        }
    }
}

#[test]
fn test_bad_type() {
    let unsupported = Unsupported;

    let error = bad_type(unsupported);
    let expected_message = "can only flatten structs and maps (got Unsupported)";

    assert_eq!(error.message, expected_message);
}

