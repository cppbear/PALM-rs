// Answer 0

#[test]
fn test_fmt_capacity_overflow() {
    let error = TryReserveError {
        kind: TryReserveErrorKind::CapacityOverflow,
    };
    let mut buffer = alloc::string::String::new();
    let result = core::fmt::write(&mut buffer, |f| error.fmt(f));
}

#[test]
fn test_fmt_alloc_error() {
    let error = TryReserveError {
        kind: TryReserveErrorKind::AllocError { layout: alloc::alloc::Layout::from_size_align(1, 1).unwrap() },
    };
    let mut buffer = alloc::string::String::new();
    let result = core::fmt::write(&mut buffer, |f| error.fmt(f));
}

#[test]
fn test_fmt_standard_error() {
    let std_error = alloc::collections::TryReserveError::CapacityOverflow;
    let error = TryReserveError {
        kind: TryReserveErrorKind::Std(std_error),
    };
    let mut buffer = alloc::string::String::new();
    let result = core::fmt::write(&mut buffer, |f| error.fmt(f));
}

