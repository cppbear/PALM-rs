// Answer 0

#[test]
fn test_find_exact_length_no_match() {
    let pattern = b"abc".to_vec();
    let haystack = b"def".to_vec();
    
    let bms = BoyerMooreSearch::new(pattern);
    assert_eq!(bms.find(&haystack), None);
}

#[test]
fn test_find_short_circuit_length_no_match() {
    let pattern = b"abc".to_vec();
    let haystack = b"defghijklmnopqrst".to_vec(); // length equals short circuit threshold
    
    let bms = BoyerMooreSearch::new(pattern);
    assert_eq!(bms.find(&haystack), None);
}

#[test]
fn test_find_exact_length_match() {
    let pattern = b"abc".to_vec();
    let haystack = b"abc".to_vec(); // exact match
    
    let bms = BoyerMooreSearch::new(pattern);
    assert_eq!(bms.find(&haystack), Some(0));
}

#[test]
fn test_find_short_circuit_length_with_match() {
    let pattern = b"abc".to_vec();
    let haystack = b"defabcghijklmnop".to_vec(); // length equal to short circuit but contains the pattern
    
    let bms = BoyerMooreSearch::new(pattern);
    assert_eq!(bms.find(&haystack), Some(3));
}

