// Answer 0

#[test]
fn test_map_match() {
    let result: Result<i32> = Result::Match(5);
    let mapped_result = result.map(|x| x * 2);
    
    if let Result::Match(value) = mapped_result {
        assert_eq!(value, 10);
    } else {
        panic!("Expected a match result");
    }
}

#[test]
fn test_map_no_match() {
    let result: Result<i32> = Result::NoMatch(3);
    let mapped_result = result.map(|x| x * 2);
    
    if let Result::NoMatch(value) = mapped_result {
        assert_eq!(value, 3);
    } else {
        panic!("Expected a no-match result");
    }
}

#[test]
fn test_map_quit() {
    let result: Result<i32> = Result::Quit;
    let mapped_result = result.map(|x| x * 2);
    
    match mapped_result {
        Result::Quit => {},
        _ => panic!("Expected a quit result"),
    }
}

