// Answer 0

#[test]
#[should_panic(expected = "advance out of bounds: the len is 0 but advancing by 1")]
fn test_panic_advance_case1() {
    let error_info = TryGetError { requested: 1, available: 0 };
    panic_advance(&error_info);
}

#[test]
#[should_panic(expected = "advance out of bounds: the len is 1 but advancing by 2")]
fn test_panic_advance_case2() {
    let error_info = TryGetError { requested: 2, available: 1 };
    panic_advance(&error_info);
}

#[test]
#[should_panic(expected = "advance out of bounds: the len is 5 but advancing by 10")]
fn test_panic_advance_case3() {
    let error_info = TryGetError { requested: 10, available: 5 };
    panic_advance(&error_info);
}

#[test]
#[should_panic(expected = "advance out of bounds: the len is 999 but advancing by 1000")]
fn test_panic_advance_case4() {
    let error_info = TryGetError { requested: 1000, available: 999 };
    panic_advance(&error_info);
}

#[test]
#[should_panic(expected = "advance out of bounds: the len is 3 but advancing by 4")]
fn test_panic_advance_case5() {
    let error_info = TryGetError { requested: 4, available: 3 };
    panic_advance(&error_info);
}

#[test]
#[should_panic(expected = "advance out of bounds: the len is 999 but advancing by 1000")]
fn test_panic_advance_case6() {
    let error_info = TryGetError { requested: 1000, available: 999 };
    panic_advance(&error_info);
}

