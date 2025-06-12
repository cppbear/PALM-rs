// Answer 0

#[test]
fn test_fmt_capacity_overflow() {
    struct TryReserveError {
        kind: TryReserveErrorKind,
    }

    enum TryReserveErrorKind {
        Std(std::io::Error),
        CapacityOverflow,
        AllocError { details: String },
    }

    let error = TryReserveError {
        kind: TryReserveErrorKind::CapacityOverflow,
    };

    let mut output = Vec::new();
    let result = error.fmt(&mut core::fmt::Formatter::new(&mut output));
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "memory allocation failed because the computed capacity exceeded the collection's maximum");
}

#[test]
fn test_fmt_alloc_error() {
    struct TryReserveError {
        kind: TryReserveErrorKind,
    }

    enum TryReserveErrorKind {
        Std(std::io::Error),
        CapacityOverflow,
        AllocError { details: String },
    }

    let error = TryReserveError {
        kind: TryReserveErrorKind::AllocError { details: String::from("Out of memory") },
    };

    let mut output = Vec::new();
    let result = error.fmt(&mut core::fmt::Formatter::new(&mut output));

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "memory allocation failed because the memory allocator returned an error");
}

#[test]
fn test_fmt_std_error() {
    struct TryReserveError {
        kind: TryReserveErrorKind,
    }

    enum TryReserveErrorKind {
        Std(std::io::Error),
        CapacityOverflow,
        AllocError { details: String },
    }

    let error = TryReserveError {
        kind: TryReserveErrorKind::Std(std::io::Error::new(std::io::ErrorKind::Other, "Some standard error")),
    };

    let mut output = Vec::new();
    let result = error.fmt(&mut core::fmt::Formatter::new(&mut output));

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "Some standard error");
}

