// Answer 0

#[test]
fn test_shortest_match_at_valid_case() {
    struct MockExec;
    
    impl Exec {
        fn shortest_match_at(&self, text: &[u8], start: usize) -> Option<usize> {
            // Mock behavior: match from start if text is not empty
            if start < text.len() {
                Some(start)
            } else {
                None
            }
        }
    }
    
    let regex = Regex(MockExec);
    let text = b"Hello, world!";
    
    assert_eq!(regex.shortest_match_at(text, 0), Some(0));
    assert_eq!(regex.shortest_match_at(text, 5), Some(5));
    assert_eq!(regex.shortest_match_at(text, 20), None);
}

#[test]
fn test_shortest_match_at_empty_text() {
    struct MockExec;
    
    impl Exec {
        fn shortest_match_at(&self, text: &[u8], start: usize) -> Option<usize> {
            // Mock behavior: no match with empty text
            if text.is_empty() {
                None
            } else {
                Some(start)
            }
        }
    }
    
    let regex = Regex(MockExec);
    let text = b"";
    
    assert_eq!(regex.shortest_match_at(text, 0), None);
} 

#[test]
fn test_shortest_match_at_boundary_conditions() {
    struct MockExec;
    
    impl Exec {
        fn shortest_match_at(&self, text: &[u8], start: usize) -> Option<usize> {
            if start >= text.len() {
                return None;
            }
            Some(start)
        }
    }
    
    let regex = Regex(MockExec);
    let text = b"abc";
    
    assert_eq!(regex.shortest_match_at(text, 0), Some(0)); // start at beginning
    assert_eq!(regex.shortest_match_at(text, 1), Some(1)); // start at first character
    assert_eq!(regex.shortest_match_at(text, 2), Some(2)); // start at second character
    assert_eq!(regex.shortest_match_at(text, 3), None); // start out of bounds
}

