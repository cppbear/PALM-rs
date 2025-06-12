// Answer 0

#[test]
fn test_fmt_capacity_overflow() {
    use core::fmt::Formatter;
    
    #[derive(Clone, PartialEq, Eq, Debug)]
    struct TryReserveError {
        kind: TryReserveErrorKind,
    }

    #[derive(Clone, PartialEq, Eq, Debug)]
    enum TryReserveErrorKind {
        Std(alloc::collections::TryReserveError),
        CapacityOverflow,
        AllocError { layout: alloc::alloc::Layout },
    }

    let error = TryReserveError {
        kind: TryReserveErrorKind::CapacityOverflow,
    };

    let mut output = core::fmt::Formatter::from(core::fmt::Write::write_str);
    let result = error.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "memory allocation failed because the computed capacity exceeded the collection's maximum");
}

#[test]
fn test_fmt_alloc_error() {
    use core::fmt::Formatter;
    
    #[derive(Clone, PartialEq, Eq, Debug)]
    struct TryReserveError {
        kind: TryReserveErrorKind,
    }

    #[derive(Clone, PartialEq, Eq, Debug)]
    enum TryReserveErrorKind {
        Std(alloc::collections::TryReserveError),
        CapacityOverflow,
        AllocError { layout: alloc::alloc::Layout },
    }

    let error = TryReserveError {
        kind: TryReserveErrorKind::AllocError { layout: alloc::alloc::Layout::from_size_align(1, 1).unwrap() },
    };

    let mut output = core::fmt::Formatter::from(core::fmt::Write::write_str);
    let result = error.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "memory allocation failed because the memory allocator returned an error");
}

