// Answer 0

#[test]
fn test_match_end_zero() {
    let m = Match::new("example", 0, 0);
    m.end();
}

#[test]
fn test_match_end_small_value() {
    let m = Match::new("example", 0, 1);
    m.end();
}

#[test]
fn test_match_end_mid_value() {
    let m = Match::new("example", 3, 5);
    m.end();
}

#[test]
fn test_match_end_large_value() {
    let haystack = "example";
    let start = 0;
    let end = usize::MAX; 
    let m = Match::new(haystack, start, end);
    m.end();
}

#[test]
fn test_match_end_full_length() {
    let m = Match::new("example", 0, 7);
    m.end();
}

#[test]
fn test_match_end_after_haystack() {
    let m = Match::new("example", 0, 8);
    m.end();
}

