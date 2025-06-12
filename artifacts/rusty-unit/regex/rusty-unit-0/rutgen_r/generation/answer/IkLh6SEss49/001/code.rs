// Answer 0

#[test]
fn test_read_captures_at_with_valid_input() {
    struct SimpleSearcher;
    
    impl SimpleSearcher {
        fn searcher(&self) -> SimpleSearcher {
            SimpleSearcher
        }

        fn read_captures_at(&self, locs: &mut Locations, text: &[u8], start: usize) -> Option<(usize, usize)> {
            if start >= text.len() {
                return None;
            }
            let search_result = text[start..].iter().position(|&b| b == b'a').map(|index| (index + start, index + start + 1));
            search_result
        }
    }
    
    struct SimpleRegex(SimpleSearcher);
    
    impl SimpleRegex {
        fn searcher(&self) -> SimpleSearcher {
            self.0.searcher()
        }
    }

    struct Locations {
        start: usize,
        end: usize,
    }

    let mut locs = Locations { start: 0, end: 0 };
    let text = b"abcabcabc";
    let regex = SimpleRegex(SimpleSearcher);

    let matched = regex.read_captures_at(&mut locs, text, 0);
    assert!(matched.is_some());
    assert_eq!(locs.start, 0);
    assert_eq!(locs.end, 1);
}

#[test]
fn test_read_captures_at_with_start_out_of_bounds() {
    struct SimpleSearcher;
    
    impl SimpleSearcher {
        fn searcher(&self) -> SimpleSearcher {
            SimpleSearcher
        }

        fn read_captures_at(&self, locs: &mut Locations, text: &[u8], start: usize) -> Option<(usize, usize)> {
            if start >= text.len() {
                return None;
            }
            let search_result = text[start..].iter().position(|&b| b == b'a').map(|index| (index + start, index + start + 1));
            search_result
        }
    }
    
    struct SimpleRegex(SimpleSearcher);
    
    impl SimpleRegex {
        fn searcher(&self) -> SimpleSearcher {
            self.0.searcher()
        }
    }

    struct Locations {
        start: usize,
        end: usize,
    }

    let mut locs = Locations { start: 0, end: 0 };
    let text = b"abcabcabc";
    let regex = SimpleRegex(SimpleSearcher);

    let matched = regex.read_captures_at(&mut locs, text, 10);
    assert!(matched.is_none());
}

#[test]
fn test_read_captures_at_with_start_at_zero() {
    struct SimpleSearcher;
    
    impl SimpleSearcher {
        fn searcher(&self) -> SimpleSearcher {
            SimpleSearcher
        }

        fn read_captures_at(&self, locs: &mut Locations, text: &[u8], start: usize) -> Option<(usize, usize)> {
            if start >= text.len() {
                return None;
            }
            let search_result = text[start..].iter().position(|&b| b == b'a').map(|index| (index + start, index + start + 1));
            search_result
        }
    }
    
    struct SimpleRegex(SimpleSearcher);
    
    impl SimpleRegex {
        fn searcher(&self) -> SimpleSearcher {
            self.0.searcher()
        }
    }

    struct Locations {
        start: usize,
        end: usize,
    }

    let mut locs = Locations { start: 0, end: 0 };
    let text = b"abcabcabc";
    let regex = SimpleRegex(SimpleSearcher);

    let matched = regex.read_captures_at(&mut locs, text, 0);
    assert!(matched.is_some());
    assert_eq!(locs.start, 0);
    assert_eq!(locs.end, 1);
}

