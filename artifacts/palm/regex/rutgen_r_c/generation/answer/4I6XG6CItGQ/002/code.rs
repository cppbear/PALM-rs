// Answer 0

#[test]
fn test_iter_teddy_ssse3() {
    struct MockVectorBuilder;
    struct MockTeddy {
        pats: Vec<Vec<u8>>,
    }
    impl MockTeddy {
        fn new(patterns: Vec<&[u8]>) -> Self {
            MockTeddy {
                pats: patterns.into_iter().map(|p| p.to_vec()).collect(),
            }
        }
        
        fn patterns(&self) -> &[Vec<u8>] {
            &self.pats
        }
    }
    
    struct MockMatcher {
        matcher: Matcher,
    }
    
    impl MockMatcher {
        fn new() -> Self {
            let teddy = MockTeddy::new(vec![b"pattern1", b"pattern2"]);
            MockMatcher {
                matcher: Matcher::TeddySSSE3(teddy),
            }
        }
        
        fn iter(&self) -> LiteralIter {
            match &self.matcher {
                Matcher::TeddySSSE3(ted) => LiteralIter::TeddySSSE3(ted.patterns()),
                _ => LiteralIter::Empty,
            }
        }
    }
    
    let matcher = MockMatcher::new();
    if let LiteralIter::TeddySSSE3(patterns) = matcher.iter() {
        assert_eq!(patterns.len(), 2);
        assert_eq!(patterns[0], b"pattern1");
        assert_eq!(patterns[1], b"pattern2");
    } else {
        panic!("Expected TeddySSSE3 variant from iterator");
    }
}

