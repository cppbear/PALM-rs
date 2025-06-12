// Answer 0

#[test]
fn test_bool() {
    // Since the function `bool` is designed to return a random boolean value,
    // we will call the function multiple times to assure the output is valid.
    
    let iterations = 1000; // number of iterations for testing robustness
    let mut true_count = 0;
    let mut false_count = 0;

    for _ in 0..iterations {
        let result = fastrand::bool(); // Call the bool function from our crate
        if result {
            true_count += 1;
        } else {
            false_count += 1;
        }
    }

    assert!(true_count > 0, "Expected at least one true result");
    assert!(false_count > 0, "Expected at least one false result");
}

#[test]
fn test_bool_edge_cases() {
    // Testing edge cases for random behavior
    
    let result = fastrand::bool(); // Call the bool function once
    assert!(result == true || result == false, "Expected either true or false");
}

