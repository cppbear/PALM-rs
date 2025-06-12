// Answer 0

#[test]
#[should_panic(expected = "advance out of bounds: the len is 0 but advancing by 1")]
fn test_panic_advance_zero_available() {
    let error_info = TryGetError { requested: 1, available: 0 };
    panic_advance(&error_info);
}

#[test]
#[should_panic(expected = "advance out of bounds: the len is 5 but advancing by 10")]
fn test_panic_advance_exceeding_requested() {
    let error_info = TryGetError { requested: 10, available: 5 };
    panic_advance(&error_info);
}

#[test]
#[should_panic(expected = "advance out of bounds: the len is 100 but advancing by 101")]
fn test_panic_advance_boundary_exceed() {
    let error_info = TryGetError { requested: 101, available: 100 };
    panic_advance(&error_info);
}

#[test]
#[should_panic(expected = "advance out of bounds: the len is 1 but advancing by 2")]
fn test_panic_advance_single_available() {
    let error_info = TryGetError { requested: 2, available: 1 };
    panic_advance(&error_info);
}

#[test]
#[should_panic(expected = "advance out of bounds: the len is 3 but advancing by 4")]
fn test_panic_advance_small_buffer() {
    let error_info = TryGetError { requested: 4, available: 3 };
    panic_advance(&error_info);
}

