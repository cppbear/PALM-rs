// Answer 0

#[test]
fn test_read_captures_at_zero_slots() {
    struct RegexMock {
        ro: RoMock,
    }

    struct RoMock {
        match_type: MatchType,
    }
    
    impl RegexMock {
        fn find_at(&self, _text: &[u8], _start: usize) -> Option<(usize, usize)> {
            None
        }
        
        fn is_anchor_end_match(&self, _text: &[u8]) -> bool {
            false
        }
    }

    let regex = RegexMock {
        ro: RoMock {
            match_type: MatchType::Nothing,
        },
    };

    let mut locs = Locations::new();
    let text = b"sample text";
    let result = regex.read_captures_at(&mut locs, text, 0);
    assert!(result.is_none());
}

#[test]
fn test_read_captures_at_two_slots() {
    struct RegexMock {
        ro: RoMock,
    }

    struct RoMock {
        match_type: MatchType,
    }

    impl RegexMock {
        fn find_at(&self, _text: &[u8], _start: usize) -> Option<(usize, usize)> {
            Some((0, 12))
        }

        fn is_anchor_end_match(&self, _text: &[u8]) -> bool {
            true
        }
    }

    let regex = RegexMock {
        ro: RoMock {
            match_type: MatchType::Literal(LiteralType::Normal),
        },
    };

    let mut locs = Locations::new_with_slots(2);
    let text = b"sample text";
    let result = regex.read_captures_at(&mut locs, text, 0);
    assert_eq!(result, Some((0, 12)));
    assert_eq!(locs.slots[0], Some(0));
    assert_eq!(locs.slots[1], Some(12));
}

#[test]
fn test_captures_nfa_with_match() {
    struct RegexMock {
        ro: RoMock,
    }

    struct RoMock {
        match_type: MatchType,
    }

    impl RegexMock {
        fn find_at(&self, _text: &[u8], _start: usize) -> Option<(usize, usize)> {
            Some((5, 9))
        }

        fn captures_nfa_with_match(&self, slots: &mut Vec<Option<usize>>, _text: &[u8], s: usize, e: usize) {
            slots[0] = Some(s);
            slots[1] = Some(e);
        }

        fn is_anchor_end_match(&self, _text: &[u8]) -> bool {
            true
        }
    }

    let regex = RegexMock {
        ro: RoMock {
            match_type: MatchType::Literal(LiteralType::Normal),
        },
    };

    let mut locs = Locations::new_with_slots(2);
    let text = b"sample text";
    let result = regex.read_captures_at(&mut locs, text, 0);
    assert_eq!(result, Some((5, 9)));
    assert_eq!(locs.slots[0], Some(5));
    assert_eq!(locs.slots[1], Some(9));
}

#[test]
fn test_multiple_matches_with_nfa() {
    struct RegexMock {
        ro: RoMock,
    }

    struct RoMock {
        match_type: MatchType,
    }

    impl RegexMock {
        fn find_at(&self, _text: &[u8], _start: usize) -> Option<(usize, usize)> {
            None
        }

        fn captures_nfa(&self, _slots: &mut Vec<Option<usize>>, _text: &[u8], _start: usize) -> Option<(usize, usize)> {
            Some((2, 6))
        }

        fn is_anchor_end_match(&self, _text: &[u8]) -> bool {
            true
        }
    }

    let regex = RegexMock {
        ro: RoMock {
            match_type: MatchType::Nfa(NfaType::Simple),
        },
    };

    let mut locs = Locations::new_with_slots(2);
    let text = b"another sample text";
    let result = regex.read_captures_at(&mut locs, text, 0);
    assert_eq!(result, Some((2, 6)));
    assert_eq!(locs.slots[0], Some(2));
    assert_eq!(locs.slots[1], Some(6));
}

