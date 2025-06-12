// Answer 0

#[test]
fn test_compare_exchange_success() {
    let once = OnceNonZeroUsize::new();
    let val = NonZeroUsize::new(1).unwrap();
    let result = once.compare_exchange(val);
}

#[test]
fn test_compare_exchange_success_high_value() {
    let once = OnceNonZeroUsize::new();
    let val = NonZeroUsize::new(10000).unwrap();
    let result = once.compare_exchange(val);
}

#[test]
fn test_compare_exchange_fail_on_non_zero() {
    let once = OnceNonZeroUsize::new();
    let val = NonZeroUsize::new(1).unwrap();
    let _ = once.compare_exchange(val);
    let fail_val = NonZeroUsize::new(2).unwrap();
    let result = once.compare_exchange(fail_val);
}

#[test]
fn test_compare_exchange_fail_on_two() {
    let once = OnceNonZeroUsize::new();
    let val1 = NonZeroUsize::new(1).unwrap();
    let _ = once.compare_exchange(val1);
    let val2 = NonZeroUsize::new(3).unwrap();
    let result = once.compare_exchange(val2);
}

