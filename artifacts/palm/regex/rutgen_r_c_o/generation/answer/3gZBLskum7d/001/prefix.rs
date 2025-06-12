// Answer 0

#[test]
fn test_new_match_valid_case() {
    let haystack = "Hello, world!";
    let start = 0;
    let end = 5;
    new(haystack, start, end);
}

#[test]
fn test_new_match_full_string() {
    let haystack = "The quick brown fox jumps over the lazy dog.";
    let start = 0;
    let end = 43;
    new(haystack, start, end);
}

#[test]
fn test_new_match_mid_string() {
    let haystack = "Rust programming language.";
    let start = 5;
    let end = 16;
    new(haystack, start, end);
}

#[test]
fn test_new_match_edge_case_zero_length() {
    let haystack = "A";
    let start = 0;
    let end = 1;
    new(haystack, start, end);
}

#[test]
fn test_new_match_start_equals_end() {
    let haystack = "Edge case testing.";
    let start = 10;
    let end = 10;
    new(haystack, start, end);
}

#[test]
fn test_new_match_large_string() {
    let haystack = "a".repeat(1000);
    let start = 0;
    let end = 1000;
    new(&haystack, start, end);
}

#[should_panic]
fn test_new_match_panic_start_out_of_bounds() {
    let haystack = "Out of bounds.";
    let start = 15;
    let end = 20;
    new(haystack, start, end);
}

#[should_panic]
fn test_new_match_panic_end_out_of_bounds() {
    let haystack = "Another example.";
    let start = 5;
    let end = 20;
    new(haystack, start, end);
}

#[should_panic]
fn test_new_match_panic_start_greater_than_end() {
    let haystack = "Invalid range.";
    let start = 5;
    let end = 3;
    new(haystack, start, end);
}

