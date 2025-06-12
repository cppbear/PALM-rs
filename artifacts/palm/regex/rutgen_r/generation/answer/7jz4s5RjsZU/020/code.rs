// Answer 0

#[test]
fn test_read_captures_at_empty_slots() {
    struct TestRegex {
        ro: MatchType,
    }

    impl TestRegex {
        fn is_anchor_end_match(&self, _text: &[u8]) -> bool {
            true
        }
        
        fn find_at(&self, _text: &[u8], _start: usize) -> Option<(usize, usize)> {
            None
        }
    }

    #[derive(Debug)]
    enum MatchType {
        DfaMany,
        Nothing,
        Dfa,
        DfaAnchoredReverse,
        DfaSuffix,
        Nfa(u32),
        Literal(u32),
    }
 
    let regex = TestRegex { ro: MatchType::DfaMany };
    let mut locs: Vec<Option<usize>> = Vec::new();
    let text: &[u8] = b"sample text";
    
    let result = regex.read_captures_at(&mut locs, text, 0);
    
    assert!(result.is_none());
}

#[test]
fn test_read_captures_at_two_slots() {
    struct TestRegex {
        ro: MatchType,
    }

    impl TestRegex {
        fn is_anchor_end_match(&self, _text: &[u8]) -> bool {
            true
        }
        
        fn find_at(&self, _text: &[u8], _start: usize) -> Option<(usize, usize)> {
            Some((0, 4))
        }
    }

    #[derive(Debug)]
    enum MatchType {
        DfaMany,
        Nothing,
        Dfa,
        DfaAnchoredReverse,
        DfaSuffix,
        Nfa(u32),
        Literal(u32),
    }
    
    let regex = TestRegex { ro: MatchType::DfaMany };
    let mut locs = vec![None; 2];
    let text: &[u8] = b"sample text";
    
    let result = regex.read_captures_at(&mut locs, text, 0);
    
    assert_eq!(result, Some((0, 4)));
    assert_eq!(locs, vec![Some(0), Some(4)]);
}

#[test]
fn test_read_captures_at_anchored_false() {
    struct TestRegex {
        ro: MatchType,
    }

    impl TestRegex {
        fn is_anchor_end_match(&self, _text: &[u8]) -> bool {
            false
        }
        
        fn find_at(&self, _text: &[u8], _start: usize) -> Option<(usize, usize)> {
            Some((0, 4))
        }
    }

    #[derive(Debug)]
    enum MatchType {
        DfaMany,
        Nothing,
        Dfa,
        DfaAnchoredReverse,
        DfaSuffix,
        Nfa(u32),
        Literal(u32),
    }

    let regex = TestRegex { ro: MatchType::DfaMany };
    let mut locs = vec![None; 3];
    let text: &[u8] = b"sample text";
    
    let result = regex.read_captures_at(&mut locs, text, 0);
    
    assert!(result.is_none());
}

