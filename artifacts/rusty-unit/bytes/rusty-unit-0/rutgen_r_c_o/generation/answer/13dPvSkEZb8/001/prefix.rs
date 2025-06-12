// Answer 0

#[test]
#[should_panic]
fn test_panic_does_not_fit_zero_size_and_large_nbytes() {
    panic_does_not_fit(0, usize::MAX + 1);
}

#[test]
#[should_panic]
fn test_panic_does_not_fit_large_size_and_zero_nbytes() {
    panic_does_not_fit(usize::MAX, 0);
}

#[test]
#[should_panic]
fn test_panic_does_not_fit_large_size_and_large_nbytes() {
    panic_does_not_fit(usize::MAX, usize::MAX + 1);
}

#[test]
#[should_panic]
fn test_panic_does_not_fit_small_size_and_large_nbytes() {
    panic_does_not_fit(1, usize::MAX + 1);
}

#[test]
#[should_panic]
fn test_panic_does_not_fit_mid_size_and_large_nbytes() {
    panic_does_not_fit(12345, usize::MAX + 1);
}

#[test]
#[should_panic]
fn test_panic_does_not_fit_mid_size_and_zero_nbytes() {
    panic_does_not_fit(12345, 0);
}

