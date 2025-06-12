// Answer 0

#[test]
fn test_read_captures_at_zero_slots() {
    struct Regex {
        ro: RegexOptions,
    }

    struct RegexOptions {
        match_type: MatchType,
    }

    enum MatchType {
        Nothing,
    }

    impl Regex {
        fn is_anchor_end_match(&self, _text: &[u8]) -> bool {
            true
        }

        fn find_at(&self, _text: &[u8], _start: usize) -> Option<(usize, usize)> {
            None
        }
    }

    let regex = Regex {
        ro: RegexOptions {
            match_type: MatchType::Nothing,
        },
    };
    let mut locs = vec![None; 0];
    let result = regex.read_captures_at(&mut locs, b"test", 0);
    assert!(result.is_none());
}

#[test]
fn test_read_captures_at_two_slots() {
    struct Regex {
        ro: RegexOptions,
    }

    struct RegexOptions {
        match_type: MatchType,
    }

    enum MatchType {
        DfaSuffix,
    }

    mod dfa {
        pub enum Result {
            Match((usize, usize)),
            NoMatch(usize),
            Quit,
        }
    }

    impl Regex {
        fn is_anchor_end_match(&self, _text: &[u8]) -> bool {
            true
        }

        fn find_at(&self, _text: &[u8], _start: usize) -> Option<(usize, usize)> {
            Some((0, 4))
        }

        fn find_dfa_reverse_suffix(&self, _text: &[u8], _start: usize) -> dfa::Result {
            dfa::Result::Match((0, 4))
        }

        fn captures_nfa_with_match(&self, slots: &mut [Option<usize>], s: usize, e: usize) -> Option<(usize, usize)> {
            slots[0] = Some(s);
            slots[1] = Some(e);
            Some((s, e))
        }
    }

    let regex = Regex {
        ro: RegexOptions {
            match_type: MatchType::DfaSuffix,
        },
    };
    let mut locs = vec![None; 2];
    let result = regex.read_captures_at(&mut locs, b"test", 0);
    assert_eq!(result, Some((0, 4)));
    assert_eq!(locs, vec![Some(0), Some(4)]);
}

#[test]
fn test_read_captures_at_generic_match() {
    struct Regex {
        ro: RegexOptions,
    }

    struct RegexOptions {
        match_type: MatchType,
    }

    enum MatchType {
        DfaSuffix,
    }

    mod dfa {
        pub enum Result {
            Match((usize, usize)),
            NoMatch(usize),
            Quit,
        }
    }

    impl Regex {
        fn is_anchor_end_match(&self, _text: &[u8]) -> bool {
            true
        }

        fn find_dfa_reverse_suffix(&self, _text: &[u8], _start: usize) -> dfa::Result {
            dfa::Result::Match((2, 6))
        }

        fn captures_nfa_with_match(&self, slots: &mut [Option<usize>], s: usize, e: usize) -> Option<(usize, usize)> {
            slots[0] = Some(s);
            slots[1] = Some(e);
            Some((s, e))
        }
    }

    let regex = Regex {
        ro: RegexOptions {
            match_type: MatchType::DfaSuffix,
        },
    };
    let mut locs = vec![None; 2];
    let result = regex.read_captures_at(&mut locs, b"abcdefg", 0);
    assert_eq!(result, Some((2, 6)));
    assert_eq!(locs, vec![Some(2), Some(6)]);
}

