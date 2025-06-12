// Answer 0

#[test]
fn test_find_at_valid_case() {
    struct Tester(Vec<u8>);
    
    let tester = Tester(b"Hello, world!".to_vec());
    let result = tester.find_at("Hello, world!", 0);
    assert_eq!(result, Some((0, 5))); // Match "Hello"
}

#[test]
fn test_find_at_start_non_zero() {
    struct Tester(Vec<u8>);
    
    let tester = Tester(b"Hello, world!".to_vec());
    let result = tester.find_at("Hello, world!", 7);
    assert_eq!(result, Some((7, 12))); // Match "world"
}

#[test]
fn test_find_at_out_of_bounds_start() {
    struct Tester(Vec<u8>);
    
    let tester = Tester(b"Hello, world!".to_vec());
    let result = tester.find_at("Hello, world!", 20);
    assert_eq!(result, None); // Starting index out of bounds
}

#[test]
fn test_find_at_empty_string() {
    struct Tester(Vec<u8>);
    
    let tester = Tester(b"".to_vec());
    let result = tester.find_at("", 0);
    assert_eq!(result, None); // Nothing to find in an empty string
}

#[test]
fn test_find_at_pattern_not_found() {
    struct Tester(Vec<u8>);
    
    let tester = Tester(b"Hello, world!".to_vec());
    let result = tester.find_at("notfound", 0);
    assert_eq!(result, None); // Pattern that does not exist
}

#[test]
fn test_find_at_pattern_at_end() {
    struct Tester(Vec<u8>);
    
    let tester = Tester(b"Hello, world!".to_vec());
    let result = tester.find_at("!", 12);
    assert_eq!(result, Some((12, 13))); // Match "!" at the end
}

