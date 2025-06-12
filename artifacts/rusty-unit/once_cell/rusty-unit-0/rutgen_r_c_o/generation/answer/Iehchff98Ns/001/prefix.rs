// Answer 0

#[test]
fn test_get_or_init_nonzero_success_1() {
    let cell = OnceNonZeroUsize::new();
    let result = cell.get_or_init(|| NonZeroUsize::new(1).unwrap());
}

#[test]
fn test_get_or_init_nonzero_success_2() {
    let cell = OnceNonZeroUsize::new();
    let result = cell.get_or_init(|| NonZeroUsize::new(2).unwrap());
}

#[test]
fn test_get_or_init_nonzero_success_3() {
    let cell = OnceNonZeroUsize::new();
    let result = cell.get_or_init(|| NonZeroUsize::new(3).unwrap());
}

#[test]
fn test_get_or_init_nonzero_success_4() {
    let cell = OnceNonZeroUsize::new();
    let result = cell.get_or_init(|| NonZeroUsize::new(4).unwrap());
}

#[test]
fn test_get_or_init_nonzero_success_5() {
    let cell = OnceNonZeroUsize::new();
    let result = cell.get_or_init(|| NonZeroUsize::new(5).unwrap());
}

#[should_panic]
fn test_get_or_init_invalid_zero() {
    let cell = OnceNonZeroUsize::new();
    let _ = cell.get_or_init(|| NonZeroUsize::new(0).unwrap());
}

#[should_panic]
fn test_get_or_init_invalid_negative() {
    let cell = OnceNonZeroUsize::new();
    let _ = cell.get_or_init(|| NonZeroUsize::new(-1i32 as usize).unwrap());
}

