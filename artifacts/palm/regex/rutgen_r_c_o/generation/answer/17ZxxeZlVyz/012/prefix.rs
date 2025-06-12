// Answer 0

#[test]
fn test_find_equal_length_haystack_pattern_no_match() {
    let pattern = b"abc";
    let search = BoyerMooreSearch::new(pattern.to_vec());
    let haystack = b"def";
    let result = search.find(haystack);
}

#[test]
fn test_find_short_circuit_length_haystack_pattern_no_match() {
    let pattern = b"abc";
    let search = BoyerMooreSearch::new(pattern.to_vec());
    let haystack = b"defghijklmno";
    let result = search.find(haystack);
}

#[test]
fn test_find_window_end_at_haystack_length() {
    let pattern = b"abc";
    let search = BoyerMooreSearch::new(pattern.to_vec());
    let haystack = b"defghabc";
    let mut result = search.find(haystack);
    while result.is_none() {
        result = search.find(haystack);
    }
}

#[test]
fn test_find_skip_zero_condition() {
    let pattern = b"apple";
    let search = BoyerMooreSearch::new(pattern.to_vec());
    let haystack = b"banana";
    let mut result = search.find(haystack);
}

#[test]
fn test_find_window_end_reached() {
    let pattern = b"xyz";
    let search = BoyerMooreSearch::new(pattern.to_vec());
    let haystack = b"abcdefgxyz";
    let mut result = search.find(haystack);
    while result.is_some() {
        result = search.find(haystack);
    }
}

