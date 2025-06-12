// Answer 0

#[test]
fn test_new_with_valid_pattern() {
    let pattern = vec![b'a', b'b', b'c'];
    
    let searcher = new(pattern.clone());
    
    assert_eq!(searcher.pattern, pattern);
}

#[test]
#[should_panic]
fn test_new_with_empty_pattern() {
    let pattern: Vec<u8> = vec![];
    
    let _searcher = new(pattern);
}

