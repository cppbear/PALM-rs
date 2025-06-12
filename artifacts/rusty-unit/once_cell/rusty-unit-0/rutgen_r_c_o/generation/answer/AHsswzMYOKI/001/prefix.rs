// Answer 0

#[test]
fn test_init_success() {
    let once_nz = OnceNonZeroUsize::new();
    let result = once_nz.init(|| Ok(NonZeroUsize::new(1).unwrap()));
}

#[test]
fn test_init_success_large_value() {
    let once_nz = OnceNonZeroUsize::new();
    let result = once_nz.init(|| Ok(NonZeroUsize::new(1000).unwrap()));
}

#[test]
fn test_init_err_zero() {
    let once_nz = OnceNonZeroUsize::new();
    let result = once_nz.init(|| Err(()));
}

#[test]
fn test_init_err_negative() {
    let once_nz = OnceNonZeroUsize::new();
    let result = once_nz.init(|| Err(()));
}

#[test]
fn test_init_second_call_is_succeeds() {
    let once_nz = OnceNonZeroUsize::new();
    let result_first = once_nz.init(|| Ok(NonZeroUsize::new(1).unwrap()));
    let result_second = once_nz.init(|| Ok(NonZeroUsize::new(2).unwrap()));
}

#[test]
fn test_init_duplicate_value() {
    let once_nz = OnceNonZeroUsize::new();
    let result_first = once_nz.init(|| Ok(NonZeroUsize::new(1).unwrap()));
    let result_second = once_nz.init(|| Ok(NonZeroUsize::new(1).unwrap()));
}

#[test]
fn test_init_err_in_f() {
    let once_nz = OnceNonZeroUsize::new();
    let result = once_nz.init(|| {
        if true { Err(()) } else { Ok(NonZeroUsize::new(1).unwrap()) }
    });
}

