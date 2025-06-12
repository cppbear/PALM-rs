// Answer 0

#[test]
fn test_map_result_quit() {
    // Create a Result::Quit variant
    let result: Result<i32> = Result::Quit;

    // Define a closure that would normally transform the value
    let transform = |x: i32| x + 1;

    // Call the map method with the Result::Quit, the expected output is also Result::Quit
    let mapped_result = result.map(transform);

    // Assert that the result is indeed Result::Quit
    match mapped_result {
        Result::Quit => (),
        _ => panic!("Expected Result::Quit but got {:?}", mapped_result),
    }
}

#[test]
fn test_map_result_no_match() {
    // Create a Result::NoMatch variant
    let result: Result<i32> = Result::NoMatch(42);

    // Define a closure that would normally transform the value
    let transform = |x: i32| x + 1;

    // Call the map method with the Result::NoMatch, the expected output is also Result::NoMatch
    let mapped_result = result.map(transform);

    // Assert that the result is indeed Result::NoMatch with the same index
    match mapped_result {
        Result::NoMatch(x) => assert_eq!(x, 42),
        _ => panic!("Expected Result::NoMatch but got {:?}", mapped_result),
    }
}

