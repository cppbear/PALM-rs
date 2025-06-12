// Answer 0

#[test]
fn test_description_compiled_too_big() {
    #[derive(Debug)]
    enum Error {
        Syntax(String),
        CompiledTooBig(usize),
        __Nonexhaustive,
    }

    impl Error {
        fn description(&self) -> &str {
            match *self {
                Error::Syntax(ref err) => err,
                Error::CompiledTooBig(_) => "compiled program too big",
                Error::__Nonexhaustive => unreachable!(),
            }
        }
    }

    let error = Error::CompiledTooBig(1024);
    let result = error.description();
    assert_eq!(result, "compiled program too big");
}

#[test]
fn test_description_compiled_too_big_large_value() {
    #[derive(Debug)]
    enum Error {
        Syntax(String),
        CompiledTooBig(usize),
        __Nonexhaustive,
    }

    impl Error {
        fn description(&self) -> &str {
            match *self {
                Error::Syntax(ref err) => err,
                Error::CompiledTooBig(_) => "compiled program too big",
                Error::__Nonexhaustive => unreachable!(),
            }
        }
    }

    let error = Error::CompiledTooBig(usize::MAX);
    let result = error.description();
    assert_eq!(result, "compiled program too big");
}

