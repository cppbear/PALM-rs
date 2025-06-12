// Answer 0

#[test]
fn test_map_no_match() {
    // Setting up the NoMatch variant with a sample index.
    let no_match_result: Result<i32> = Result::NoMatch(5);
    
    // Mapping a function that should not affect NoMatch.
    let mapped_result = no_match_result.map(|x| x + 1);
    
    // Ensure the mapped result remains NoMatch with the same index.
    match mapped_result {
        Result::NoMatch(x) => assert_eq!(x, 5),
        _ => panic!("Expected NoMatch, found a different variant"),
    }
}

#[test]
fn test_map_quit() {
    // Setting up the Quit variant.
    let quit_result: Result<i32> = Result::Quit;
    
    // Mapping a function on Quit should result in Quit.
    let mapped_result = quit_result.map(|x| x + 1);

    // Ensure the mapped result is still Quit.
    match mapped_result {
        Result::Quit => {},
        _ => panic!("Expected Quit, found a different variant"),
    }
}

#[test]
fn test_map_match() {
    // Setting up a Match variant.
    let match_result: Result<i32> = Result::Match(10);
    
    // Mapping a function that modifies the successful match.
    let mapped_result = match_result.map(|x| x + 5);

    // Ensure the mapped result is now a Match with the transformed value.
    match mapped_result {
        Result::Match(x) => assert_eq!(x, 15),
        _ => panic!("Expected Match, found a different variant"),
    }
}

