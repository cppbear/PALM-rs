// Answer 0

#[test]
#[should_panic(expected = "size too large: the integer type can fit 8 bytes, but nbytes is 16")]
fn test_panic_does_not_fit_large() {
    panic_does_not_fit(8, 16);
}

#[test]
#[should_panic(expected = "size too large: the integer type can fit 4 bytes, but nbytes is 8")]
fn test_panic_does_not_fit_large_4() {
    panic_does_not_fit(4, 8);
}

#[test]
#[should_panic(expected = "size too large: the integer type can fit 16 bytes, but nbytes is 32")]
fn test_panic_does_not_fit_large_16() {
    panic_does_not_fit(16, 32);
}

#[test]
#[should_panic(expected = "size too large: the integer type can fit 32 bytes, but nbytes is 64")]
fn test_panic_does_not_fit_large_32() {
    panic_does_not_fit(32, 64);
}

