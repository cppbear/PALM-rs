// Answer 0

#[test]
#[should_panic(expected = "advance out of bounds: the len is 0 but advancing by 1")]
fn test_panic_advance_out_of_bounds() {
    struct TryGetError {
        available: usize,
        requested: usize,
    }

    let error_info = TryGetError {
        available: 0,
        requested: 1,
    };
    panic_advance(&error_info);
}

#[test]
#[should_panic(expected = "advance out of bounds: the len is 5 but advancing by 10")]
fn test_panic_advance_exceeding_bounds() {
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

#[test]
#[should_panic(expected = "advance out of bounds: the len is 2 but advancing by 3")]
fn test_panic_advance_exactly_out_of_bounds() {
    struct TryGetError {
        available: usize,
        requested: usize,
    }

    let error_info = TryGetError {
        available: 2,
        requested: 3,
    };
    panic_advance(&error_info);
}

