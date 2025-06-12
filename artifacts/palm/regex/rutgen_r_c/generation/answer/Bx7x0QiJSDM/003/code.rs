// Answer 0

#[test]
fn test_map_with_match() {
    let result: Result<i32> = Result::Match(5);
    let mapped_result = result.map(|x| x * 2);
    match mapped_result {
        Result::Match(value) => assert_eq!(value, 10),
        _ => panic!("Expected Result::Match(10), but got a different result"),
    }
}

#[test]
fn test_map_with_no_match() {
    let result: Result<i32> = Result::NoMatch(0);
    let mapped_result = result.map(|x| x * 2);
    match mapped_result {
        Result::NoMatch(x) => assert_eq!(x, 0),
        _ => panic!("Expected Result::NoMatch(0), but got a different result"),
    }
}

#[test]
fn test_map_with_quit() {
    let result: Result<i32> = Result::Quit;
    let mapped_result = result.map(|x| x * 2);
    match mapped_result {
        Result::Quit => {},
        _ => panic!("Expected Result::Quit, but got a different result"),
    }
}

