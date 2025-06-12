// Answer 0

#[test]
fn test_set_success_case() {
    let once = OnceNonZeroUsize::new();
    let value = NonZeroUsize::new(1).unwrap();
    once.set(value);
}

#[test]
fn test_set_success_case_large_value() {
    let once = OnceNonZeroUsize::new();
    let value = NonZeroUsize::new(usize::MAX).unwrap();
    once.set(value);
}

#[test]
fn test_set_success_case_mid_range() {
    let once = OnceNonZeroUsize::new();
    let value = NonZeroUsize::new(12345).unwrap();
    once.set(value);
}

#[test]
fn test_set_success_case_with_random_value() {
    let once = OnceNonZeroUsize::new();
    let value = NonZeroUsize::new(99999).unwrap();
    once.set(value);
}

