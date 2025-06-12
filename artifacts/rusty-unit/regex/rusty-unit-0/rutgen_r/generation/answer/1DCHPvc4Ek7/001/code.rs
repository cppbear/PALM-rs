// Answer 0

#[test]
#[should_panic]
fn test_fmt_with_nonexhaustive_error() {
    struct Error {
        kind: ErrorKind,
    }

    enum ErrorKind {
        Syntax(String),
        CompiledTooBig(usize),
        __Nonexhaustive,
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self.kind {
                ErrorKind::Syntax(ref err) => write!(f, "Syntax error: {}", err),
                ErrorKind::CompiledTooBig(limit) => {
                    write!(f, "Compiled regex exceeds size limit of {} bytes.", limit)
                }
                ErrorKind::__Nonexhaustive => unreachable!(),
            }
        }
    }

    let error = Error { kind: ErrorKind::__Nonexhaustive };
    let _ = format!("{}", error);
}

