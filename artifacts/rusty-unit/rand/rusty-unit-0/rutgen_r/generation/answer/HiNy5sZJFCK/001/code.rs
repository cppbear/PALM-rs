// Answer 0

fn test_fmt_non_finite() {
    struct TestError;

    impl std::fmt::Display for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("Non-finite range in uniform distribution")
        }
    }

    impl std::fmt::Debug for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TestError")
        }
    }

    enum Error {
        EmptyRange,
        NonFinite,
    }

    impl Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(match self {
                Error::EmptyRange => "low > high (or equal if exclusive) in uniform distribution",
                Error::NonFinite => "Non-finite range in uniform distribution",
            })
        }
    }

    let error = Error::NonFinite;
    
    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new();
        let result = error.fmt(&mut formatter);
        output = formatter.into_inner().to_string();
    }
    
    assert_eq!(output, "Non-finite range in uniform distribution");
}

fn test_fmt_empty_range() {
    enum Error {
        EmptyRange,
        NonFinite,
    }

    impl Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(match self {
                Error::EmptyRange => "low > high (or equal if exclusive) in uniform distribution",
                Error::NonFinite => "Non-finite range in uniform distribution",
            })
        }
    }

    let error = Error::EmptyRange;
    
    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new();
        let result = error.fmt(&mut formatter);
        output = formatter.into_inner().to_string();
    }
    
    assert_eq!(output, "low > high (or equal if exclusive) in uniform distribution");
}

