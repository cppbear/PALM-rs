// Answer 0

#[test]
fn test_many_matches_at_literal_match() {
    struct Regex {
        ro: RegexData,
        cache: CacheType,
    }

    struct RegexData {
        match_type: MatchType,
        dfa: DfaType,
    }

    enum MatchType {
        Literal(LiteralType),
        Dfa,
        DfaAnchoredReverse,
        DfaSuffix,
        DfaMany,
        Nfa(NfaType),
        Nothing,
    }

    struct CacheType;
    struct DfaType;
    struct LiteralType;
    struct NfaType;

    impl Regex {
        fn is_anchor_end_match(&self, _text: &[u8]) -> bool {
            true
        }

        fn find_literals(&self, _ty: LiteralType, _text: &[u8], _start: usize) -> Option<()> {
            Some(())
        }

        fn exec_nfa(&self, _ty: NfaType, _matches: &mut [bool], _context: &mut [()], _flag: bool, _text: &[u8], _start: usize) -> bool {
            true
        }
    }

    let regex = Regex {
        ro: RegexData {
            match_type: MatchType::Literal(LiteralType),
            dfa: DfaType,
        },
        cache: CacheType,
    };

    let mut matches = vec![false];
    let text = b"test";
    let start = 0;

    assert!(regex.many_matches_at(&mut matches, text, start));
    assert!(matches[0]);
}

#[test]
fn test_many_matches_at_nfa_match() {
    struct Regex {
        ro: RegexData,
        cache: CacheType,
    }

    struct RegexData {
        match_type: MatchType,
        dfa: DfaType,
    }

    enum MatchType {
        Literal(LiteralType),
        Dfa,
        DfaAnchoredReverse,
        DfaSuffix,
        DfaMany,
        Nfa(NfaType),
        Nothing,
    }

    struct CacheType;
    struct DfaType;
    struct LiteralType;
    struct NfaType;

    impl Regex {
        fn is_anchor_end_match(&self, _text: &[u8]) -> bool {
            true
        }

        fn exec_nfa(&self, _ty: NfaType, _matches: &mut [bool], _context: &mut [()], _flag: bool, _text: &[u8], _start: usize) -> bool {
            matches[0] = true;
            true
        }
    }

    let regex = Regex {
        ro: RegexData {
            match_type: MatchType::Nfa(NfaType),
            dfa: DfaType,
        },
        cache: CacheType,
    };

    let mut matches = vec![false];
    let text = b"test";
    let start = 0;

    assert!(regex.many_matches_at(&mut matches, text, start));
    assert!(matches[0]);
}

#[test]
fn test_many_matches_at_no_match() {
    struct Regex {
        ro: RegexData,
        cache: CacheType,
    }

    struct RegexData {
        match_type: MatchType,
        dfa: DfaType,
    }

    enum MatchType {
        Literal(LiteralType),
        Dfa,
        DfaAnchoredReverse,
        DfaSuffix,
        DfaMany,
        Nfa(NfaType),
        Nothing,
    }

    struct CacheType;
    struct DfaType;
    struct LiteralType;
    struct NfaType;

    impl Regex {
        fn is_anchor_end_match(&self, _text: &[u8]) -> bool {
            true
        }

        fn exec_nfa(&self, _ty: NfaType, _matches: &mut [bool], _context: &mut [()], _flag: bool, _text: &[u8], _start: usize) -> bool {
            false
        }
    }

    let regex = Regex {
        ro: RegexData {
            match_type: MatchType::Nfa(NfaType),
            dfa: DfaType,
        },
        cache: CacheType,
    };

    let mut matches = vec![false];
    let text = b"test";
    let start = 0;

    assert!(!regex.many_matches_at(&mut matches, text, start));
    assert!(!matches[0]);
}

