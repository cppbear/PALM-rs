// Answer 0

#[test]
#[should_panic(expected = "size too large: the integer type can fit 4 bytes, but nbytes is 8")]
fn test_panic_does_not_fit_large_nbytes() {
    panic_does_not_fit(4, 8);
}

#[test]
#[should_panic(expected = "size too large: the integer type can fit 2 bytes, but nbytes is 4")]
fn test_panic_does_not_fit_small_size() {
    panic_does_not_fit(2, 4);
}

#[test]
#[should_panic(expected = "size too large: the integer type can fit 8 bytes, but nbytes is 16")]
fn test_panic_does_not_fit_exact_fit() {
    panic_does_not_fit(8, 16);
}

#[test]
#[should_panic(expected = "size too large: the integer type can fit 8 bytes, but nbytes is 8")]
fn test_panic_does_not_fit_exact_size() {
    panic_does_not_fit(8, 8);
}

