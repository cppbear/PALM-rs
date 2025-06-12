// Answer 0

#[test]
fn test_new_match_valid_inputs() {
    let haystack: &[u8] = b"hello world";
    let start: usize = 0;
    let end: usize = 5;
    new(haystack, start, end);
}

#[test]
fn test_new_match_start_equals_end() {
    let haystack: &[u8] = b"hello world";
    let start: usize = 5;
    let end: usize = 5;
    new(haystack, start, end);
}

#[test]
fn test_new_match_end_exceeds_haystack_length() {
    let haystack: &[u8] = b"hello world";
    let start: usize = 5;
    let end: usize = 12; // greater than haystack length
    new(haystack, start, end);
}

#[test]
fn test_new_match_start_out_of_bounds() {
    let haystack: &[u8] = b"hello world";
    let start: usize = 11; // greater than maximum index
    let end: usize = 11;
    new(haystack, start, end);
}

#[test]
fn test_new_match_multiple_cases() {
    let haystack: &[u8] = b"rust programming";
    let start: usize = 0;
    let end: usize = haystack.len();
    new(haystack, start, end);
    
    let start: usize = 5;
    let end: usize = 10;
    new(haystack, start, end);
    
    let start: usize = 3;
    let end: usize = 8; 
    new(haystack, start, end);
}

#[test]
fn test_new_match_empty_string() {
    let haystack: &[u8] = b"";
    let start: usize = 0;
    let end: usize = 0; 
    new(haystack, start, end);
}

#[test]
fn test_new_match_single_character() {
    let haystack: &[u8] = b"a";
    let start: usize = 0;
    let end: usize = 1;
    new(haystack, start, end);
}

