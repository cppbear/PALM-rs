// Answer 0

#[test]
fn test_fmt_std_error() {
    struct MockError;
    impl core::fmt::Display for MockError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("mocked standard error")
        }
    }

    enum TryReserveErrorKind {
        Std(MockError),
        CapacityOverflow,
        AllocError,
    }

    struct TryReserveError {
        kind: TryReserveErrorKind,
    }

    impl TryReserveError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let reason = match &self.kind {
                TryReserveErrorKind::Std(e) => return core::fmt::Display::fmt(e, f),
                TryReserveErrorKind::CapacityOverflow => {
                    " because the computed capacity exceeded the collection's maximum"
                }
                TryReserveErrorKind::AllocError => {
                    " because the memory allocator returned an error"
                }
            };
            f.write_str("memory allocation failed")?;
            f.write_str(reason)
        }
    }

    let error = TryReserveError {
        kind: TryReserveErrorKind::Std(MockError),
    };

    let result = format!("{}", error);
    assert_eq!(result, "mocked standard error");
}

#[test]
fn test_fmt_capacity_overflow() {
    enum TryReserveErrorKind {
        Std(MockError),
        CapacityOverflow,
        AllocError,
    }

    struct TryReserveError {
        kind: TryReserveErrorKind,
    }

    impl TryReserveError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let reason = match &self.kind {
                TryReserveErrorKind::Std(e) => return core::fmt::Display::fmt(e, f),
                TryReserveErrorKind::CapacityOverflow => {
                    " because the computed capacity exceeded the collection's maximum"
                }
                TryReserveErrorKind::AllocError => {
                    " because the memory allocator returned an error"
                }
            };
            f.write_str("memory allocation failed")?;
            f.write_str(reason)
        }
    }

    let error = TryReserveError {
        kind: TryReserveErrorKind::CapacityOverflow,
    };

    let result = format!("{}", error);
    assert_eq!(result, "memory allocation failed because the computed capacity exceeded the collection's maximum");
}

#[test]
fn test_fmt_alloc_error() {
    enum TryReserveErrorKind {
        Std(MockError),
        CapacityOverflow,
        AllocError,
    }

    struct TryReserveError {
        kind: TryReserveErrorKind,
    }

    impl TryReserveError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let reason = match &self.kind {
                TryReserveErrorKind::Std(e) => return core::fmt::Display::fmt(e, f),
                TryReserveErrorKind::CapacityOverflow => {
                    " because the computed capacity exceeded the collection's maximum"
                }
                TryReserveErrorKind::AllocError => {
                    " because the memory allocator returned an error"
                }
            };
            f.write_str("memory allocation failed")?;
            f.write_str(reason)
        }
    }

    let error = TryReserveError {
        kind: TryReserveErrorKind::AllocError,
    };

    let result = format!("{}", error);
    assert_eq!(result, "memory allocation failed because the memory allocator returned an error");
}

