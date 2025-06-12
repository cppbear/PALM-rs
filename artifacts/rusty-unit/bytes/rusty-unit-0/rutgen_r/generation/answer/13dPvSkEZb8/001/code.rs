// Answer 0

#[test]
#[should_panic(expected = "size too large: the integer type can fit 0 bytes, but nbytes is 1")]
fn test_panic_does_not_fit_zero_size() {
    let size = 0;
    let nbytes = 1;
    panic_does_not_fit(size, nbytes);
}

#[test]
#[should_panic(expected = "size too large: the integer type can fit 1 bytes, but nbytes is 2")]
fn test_panic_does_not_fit_one_size() {
    let size = 1;
    let nbytes = 2;
    panic_does_not_fit(size, nbytes);
}

#[test]
#[should_panic(expected = "size too large: the integer type can fit 2 bytes, but nbytes is 3")]
fn test_panic_does_not_fit_two_size() {
    let size = 2;
    let nbytes = 3;
    panic_does_not_fit(size, nbytes);
}

#[test]
#[should_panic(expected = "size too large: the integer type can fit 4 bytes, but nbytes is 5")]
fn test_panic_does_not_fit_four_size() {
    let size = 4;
    let nbytes = 5;
    panic_does_not_fit(size, nbytes);
}

#[test]
#[should_panic(expected = "size too large: the integer type can fit 8 bytes, but nbytes is 9")]
fn test_panic_does_not_fit_eight_size() {
    let size = 8;
    let nbytes = 9;
    panic_does_not_fit(size, nbytes);
}

