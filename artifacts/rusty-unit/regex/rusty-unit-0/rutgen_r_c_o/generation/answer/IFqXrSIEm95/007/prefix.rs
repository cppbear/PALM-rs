// Answer 0

#[test]
fn test_description_nest_limit_exceeded_min() {
    let error = Error {
        kind: ErrorKind::NestLimitExceeded(1),
    };
    let _ = error.description();
}

#[test]
fn test_description_nest_limit_exceeded_middle() {
    let error = Error {
        kind: ErrorKind::NestLimitExceeded(5),
    };
    let _ = error.description();
}

#[test]
fn test_description_nest_limit_exceeded_max() {
    let error = Error {
        kind: ErrorKind::NestLimitExceeded(10),
    };
    let _ = error.description();
}

