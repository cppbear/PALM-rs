// Answer 0

#[test]
#[should_panic(expected = "advance out of bounds: the len is 5 but advancing by 10")]
fn test_panic_advance_exceeds_available_bytes() {
    let error_info = TryGetError {
        requested: 10,
        available: 5,
    };
    panic_advance(&error_info);
}

#[test]
#[should_panic(expected = "advance out of bounds: the len is 0 but advancing by 1")]
fn test_panic_advance_zero_available_bytes() {
    let error_info = TryGetError {
        requested: 1,
        available: 0,
    };
    panic_advance(&error_info);
}

#[test]
#[should_panic(expected = "advance out of bounds: the len is 3 but advancing by 3")]
fn test_panic_advance_exact_available_bytes() {
    let error_info = TryGetError {
        requested: 3,
        available: 3,
    };
    panic_advance(&error_info);
}

