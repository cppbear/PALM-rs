// Answer 0

#[test]
fn test_as_str() {
    let haystack = "Hello, world!";
    let start = 7;
    let end = 12;
    
    let m = Match::new(haystack, start, end);
    
    assert_eq!(m.as_str(), "world");
}

#[test]
fn test_as_str_boundary() {
    let haystack = "Boundary test";
    let start = 0;
    let end = 8;
    
    let m = Match::new(haystack, start, end);
    
    assert_eq!(m.as_str(), "Boundary");
}

#[test]
fn test_as_str_empty_match() {
    let haystack = "Empty match";
    let start = 5;
    let end = 5;
    
    let m = Match::new(haystack, start, end);
    
    assert_eq!(m.as_str(), "");
}

#[test]
fn test_as_str_full_string() {
    let haystack = "Full string match";
    let start = 0;
    let end = 17;
    
    let m = Match::new(haystack, start, end);
    
    assert_eq!(m.as_str(), "Full string match");
}

