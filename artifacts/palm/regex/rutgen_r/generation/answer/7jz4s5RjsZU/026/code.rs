// Answer 0

#[test]
fn test_read_captures_at_empty_slots() {
    struct MockRegex {
        ro: MatchType,
    }

    impl MockRegex {
        fn find_at(&self, _text: &[u8], _start: usize) -> Option<(usize, usize)> {
            None
        }
        
        fn is_anchor_end_match(&self, _text: &[u8]) -> bool {
            true
        }
        
        fn find_dfa_anchored_reverse(&self, _text: &[u8], _start: usize) -> dfa::Result {
            dfa::Result::Match((0, 0))
        }
    }

    #[derive(Default)]
    struct Locations(Vec<Option<usize>>);

    let mut locs = Locations(vec![]);
    let regex = MockRegex {
        ro: MatchType::DfaAnchoredReverse,
    };
    let result = regex.read_captures_at(&mut locs, b"abc", 0);
    assert_eq!(result, None);
}

#[test]
fn test_read_captures_at_two_slots() {
    struct MockRegex {
        ro: MatchType,
    }

    impl MockRegex {
        fn find_at(&self, _text: &[u8], _start: usize) -> Option<(usize, usize)> {
            Some((0, 3))
        }
        
        fn is_anchor_end_match(&self, _text: &[u8]) -> bool {
            true
        }
        
        fn find_dfa_anchored_reverse(&self, _text: &[u8], _start: usize) -> dfa::Result {
            dfa::Result::Match((0, 3))
        }
    }

    #[derive(Default)]
    struct Locations(Vec<Option<usize>>);

    impl Locations {
        fn as_slots(&mut self) -> &mut Vec<Option<usize>> {
            &mut self.0
        }
    }

    let mut locs = Locations(vec![None; 2]);
    let regex = MockRegex {
        ro: MatchType::DfaAnchoredReverse,
    };
    let result = regex.read_captures_at(&mut locs, b"abc", 0);
    assert_eq!(result, Some((0, 3)));
    assert_eq!(locs.0, vec![Some(0), Some(3)]);
}

#[test]
fn test_read_captures_at_non_empty_slots() {
    struct MockRegex {
        ro: MatchType,
    }

    impl MockRegex {
        fn find_at(&self, _text: &[u8], _start: usize) -> Option<(usize, usize)> {
            Some((0, 3))
        }
        
        fn is_anchor_end_match(&self, _text: &[u8]) -> bool {
            true
        }

        fn find_dfa_anchored_reverse(&self, _text: &[u8], _start: usize) -> dfa::Result {
            dfa::Result::Match((0, 3))
        }
    }

    #[derive(Default)]
    struct Locations(Vec<Option<usize>>);

    impl Locations {
        fn as_slots(&mut self) -> &mut Vec<Option<usize>> {
            &mut self.0
        }
    }

    let mut locs = Locations(vec![Some(2), Some(5)]); // pre-filled slots
    let regex = MockRegex {
        ro: MatchType::DfaAnchoredReverse,
    };
    let result = regex.read_captures_at(&mut locs, b"abc", 0);
    assert_eq!(result, Some((0, 3)));
    assert_eq!(locs.0, vec![Some(0), Some(3)]); // should overwrite the previous slots
}

