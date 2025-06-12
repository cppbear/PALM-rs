// Answer 0

#[test]
fn test_find_bound_haystack_equal_pattern_length() {
    let pattern = vec![b'a'];
    let haystack = vec![b'b']; // haystack.len() == self.pattern.len()
    let bms = BoyerMooreSearch::new(pattern);
    bms.find(&haystack);
}

#[test]
fn test_find_haystack_length_equal_short_circuit() {
    let pattern = vec![b'a', b'b'];
    let haystack = vec![b'c'; 20]; // haystack.len() == short_circut
    let bms = BoyerMooreSearch::new(pattern);
    bms.find(&haystack);
}

#[test]
fn test_find_window_end_at_haystack_length() {
    let pattern = vec![b'a', b'b'];
    let haystack = vec![b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x']; // haystack.len() = 20
    let bms = BoyerMooreSearch::new(pattern);
    let _ = bms.find(&haystack);
}

#[test]
fn test_find_skip_table_value_not_zero() {
    let pattern = vec![b'a', b'b'];
    let haystack = vec![b'c', b'a', b'd', b'b', b'e', b'f', b'g', b'b', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's'];
    let bms = BoyerMooreSearch::new(pattern);
    let _ = bms.find(&haystack);
}

#[test]
fn test_find_check_match_false() {
    let pattern = vec![b'a', b'b'];
    let haystack = vec![b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's'];
    let bms = BoyerMooreSearch::new(pattern);
    let _ = bms.find(&haystack);
}

