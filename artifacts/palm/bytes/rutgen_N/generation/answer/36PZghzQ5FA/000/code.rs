// Answer 0

#[test]
#[should_panic(expected = "advance out of bounds: the len is 5 but advancing by 10")]
fn test_panic_advance() {
    struct TryGetError {
        available: usize,
        requested: usize,
    }

    let error_info = TryGetError {
        available: 5,
        requested: 10,
    };

    panic_advance(&error_info);
}

