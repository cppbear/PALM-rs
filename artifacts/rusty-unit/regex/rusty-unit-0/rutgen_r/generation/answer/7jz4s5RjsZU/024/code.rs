// Answer 0

#[test]
fn test_read_captures_at_empty_slots() {
    struct DummyRegex {
        match_type: MatchType,
    }

    let regex = DummyRegex {
        match_type: MatchType::Nothing,
    };

    let mut locs = Locations::default();
    let text: &[u8] = b"test";
    let result = regex.read_captures_at(&mut locs, text, 0);
    
    assert_eq!(result, None);
}

#[test]
fn test_read_captures_at_two_slots() {
    struct DummyRegex {
        match_type: MatchType,
    }

    let regex = DummyRegex {
        match_type: MatchType::Literal(LiteralType::SomeType),
    };

    let mut locs = Locations::from(vec![None, None]);
    let text: &[u8] = b"match here";
    let start = 0;

    let result = regex.read_captures_at(&mut locs, text, start);
    
    assert!(result.is_some());
}

#[test]
fn test_read_captures_at_with_is_anchor_end_match() {
    struct DummyRegex {
        match_type: MatchType,
        anchored: bool,
    }

    impl DummyRegex {
        fn is_anchor_end_match(&self, _text: &[u8]) -> bool {
            self.anchored
        }

        fn find_dfa_anchored_reverse(&self, _text: &[u8], _start: usize) -> dfa::Result<(usize, usize)> {
            dfa::Result::Quit
        }
    }

    let regex = DummyRegex {
        match_type: MatchType::DfaAnchoredReverse,
        anchored: true,
    };

    let mut locs = Locations::default();
    let text: &[u8] = b"test";
    let start = 0;

    let result = regex.read_captures_at(&mut locs, text, start);

    assert!(result.is_none());
}

#[test]
fn test_read_captures_at_dfa_many() {
    struct DummyRegex {
        match_type: MatchType,
    }

    let regex = DummyRegex {
        match_type: MatchType::DfaMany,
    };

    let mut locs = Locations::from(vec![None, None]);
    let text: &[u8] = b"many matches";
    let start = 0;

    let result = regex.read_captures_at(&mut locs, text, start);

    assert!(result.is_none());
}

