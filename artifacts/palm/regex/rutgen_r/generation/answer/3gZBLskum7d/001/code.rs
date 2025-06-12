// Answer 0

#[test]
fn test_new_valid_inputs() {
    let haystack = "Hello, world!";
    let start = 0;
    let end = 5;
    let result = new(haystack, start, end);
    assert_eq!(result.text, "Hello, world!");
    assert_eq!(result.start, 0);
    assert_eq!(result.end, 5);
}

#[test]
#[should_panic]
fn test_new_start_out_of_bounds() {
    let haystack = "Hello, world!";
    let start = 20;
    let end = 25;
    new(haystack, start, end);
}

#[test]
#[should_panic]
fn test_new_end_out_of_bounds() {
    let haystack = "Hello, world!";
    let start = 5;
    let end = 20;
    new(haystack, start, end);
}

#[test]
#[should_panic]
fn test_new_end_less_than_start() {
    let haystack = "Hello, world!";
    let start = 5;
    let end = 3;
    new(haystack, start, end);
}

