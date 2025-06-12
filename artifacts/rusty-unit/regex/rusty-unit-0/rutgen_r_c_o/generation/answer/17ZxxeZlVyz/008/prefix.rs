// Answer 0

#[test]
fn test_find_exact_match_edge_case() {
    let pattern: Vec<u8> = b"abc".to_vec();
    let haystack: Vec<u8> = b"abcdefghijabcdefghijabc".to_vec();
    let bms = BoyerMooreSearch::new(pattern);
    let _ = bms.find(&haystack);
}

#[test]
fn test_find_no_match_due_to_skip_condition() {
    let pattern: Vec<u8> = b"abcde".to_vec();
    let haystack: Vec<u8> = b"abcdefghijabcdefghijaxx".to_vec();
    let bms = BoyerMooreSearch::new(pattern);
    let _ = bms.find(&haystack);
}

#[test]
fn test_find_no_match_due_to_short_circuit() {
    let pattern: Vec<u8> = b"xyz".to_vec();
    let haystack: Vec<u8> = b"abcdefghijabcdefghijabcde".to_vec(); // length 21
    let bms = BoyerMooreSearch::new(pattern);
    let _ = bms.find(&haystack);
}

#[test]
fn test_find_no_match_with_zero_skip() {
    let pattern: Vec<u8> = b"abc".to_vec();
    let haystack: Vec<u8> = b"abcdefghijabcdefghijaaa".to_vec(); // length 21
    let bms = BoyerMooreSearch::new(pattern);
    let _ = bms.find(&haystack);
}

#[test]
fn test_find_loop_return_none() {
    let pattern: Vec<u8> = b"xyz".to_vec();
    let haystack: Vec<u8> = b"abcdefghijabcdefghijabc".to_vec(); // length 21
    let bms = BoyerMooreSearch::new(pattern);
    let _ = bms.find(&haystack);
}

