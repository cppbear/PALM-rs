// Answer 0

#[derive(Debug)]
enum TryReserveErrorKind {
    Std(std::io::Error),
    CapacityOverflow,
    AllocError,
}

struct ReserveError {
    kind: TryReserveErrorKind,
}

impl ReserveError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let reason = match &self.kind {
            TryReserveErrorKind::Std(e) => return core::fmt::Display::fmt(e, f),
            TryReserveErrorKind::CapacityOverflow => {
                " because the computed capacity exceeded the collection's maximum"
            }
            TryReserveErrorKind::AllocError { .. } => {
                " because the memory allocator returned an error"
            }
        };
        f.write_str("memory allocation failed")?;
        f.write_str(reason)
    }
}

#[test]
fn test_capacity_overflow_error() {
    let error = ReserveError {
        kind: TryReserveErrorKind::CapacityOverflow,
    };
    let mut buffer = core::fmt::Formatter::new();
    let result = error.fmt(&mut buffer);
    assert!(result.is_ok());
    let output = format!("{}", error);
    assert_eq!(output, "memory allocation failed because the computed capacity exceeded the collection's maximum");
}

#[test]
fn test_std_error() {
    let std_error = std::io::Error::new(std::io::ErrorKind::Other, "error");
    let error = ReserveError {
        kind: TryReserveErrorKind::Std(std_error),
    };
    let mut buffer = core::fmt::Formatter::new();
    let result = error.fmt(&mut buffer);
    assert!(result.is_ok());
    let output = format!("{}", error);
    assert_eq!(output, "error");
}

#[test]
fn test_alloc_error() {
    let error = ReserveError {
        kind: TryReserveErrorKind::AllocError {},
    };
    let mut buffer = core::fmt::Formatter::new();
    let result = error.fmt(&mut buffer);
    assert!(result.is_ok());
    let output = format!("{}", error);
    assert_eq!(output, "memory allocation failed because the memory allocator returned an error");
}

