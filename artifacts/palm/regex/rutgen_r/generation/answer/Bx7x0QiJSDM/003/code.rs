// Answer 0

#[derive(Debug)]
enum Result<T> {
    Match(T),
    NoMatch(String),
    Quit,
}

#[test]
fn test_map_match() {
    let result = Result::Match(5);
    let mapped_result = result.map(|x| x * 2);
    match mapped_result {
        Result::Match(value) => assert_eq!(value, 10),
        _ => panic!("Expected Result::Match(10), got {:?}", mapped_result),
    };
}

#[test]
fn test_map_no_match() {
    let result = Result::NoMatch(String::from("Not a match"));
    let mapped_result = result.map(|x: i32| x * 2);
    match mapped_result {
        Result::NoMatch(msg) => assert_eq!(msg, "Not a match"),
        _ => panic!("Expected Result::NoMatch, got {:?}", mapped_result),
    };
}

#[test]
fn test_map_quit() {
    let result = Result::Quit;
    let mapped_result = result.map(|x: i32| x * 2);
    match mapped_result {
        Result::Quit => assert!(true),
        _ => panic!("Expected Result::Quit, got {:?}", mapped_result),
    };
}

