// Answer 0

#[test]
fn test_find_at_with_literal_match() {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::sync::Arc;

    struct TestExecReadOnly {
        res: Vec<String>,
        nfa: Program,
        dfa: Program,
        dfa_reverse: Program,
        suffixes: LiteralSearcher,
        match_type: MatchType,
    }

    let suffixes = LiteralSearcher::new(&[]); // Initialize appropriately
    let ro = TestExecReadOnly {
        res: vec!["abc".to_string()],
        nfa: Program::default(), // Use your appropriate initialization here
        dfa: Program::default(), // Use your appropriate initialization here
        dfa_reverse: Program::default(), // Use your appropriate initialization here
        suffixes,
        match_type: MatchType::Literal(MatchLiteralType::Unanchored),
    };

    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync {
        ro: &Arc::new(ro),
        cache: &cache,
    };

    let text: &[u8] = b"abcde";
    let start = 0;

    let result = exec.find_at(text, start);
    assert_eq!(result, Some((0, 3))); // "abc" is found at position (0, 3)
}

#[test]
fn test_find_at_with_no_match() {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::sync::Arc;

    struct TestExecReadOnly {
        res: Vec<String>,
        nfa: Program,
        dfa: Program,
        dfa_reverse: Program,
        suffixes: LiteralSearcher,
        match_type: MatchType,
    }

    let suffixes = LiteralSearcher::new(&[]); // Initialize appropriately
    let ro = TestExecReadOnly {
        res: vec!["xyz".to_string()],
        nfa: Program::default(), // Use your appropriate initialization here
        dfa: Program::default(), // Use your appropriate initialization here
        dfa_reverse: Program::default(), // Use your appropriate initialization here
        suffixes,
        match_type: MatchType::Literal(MatchLiteralType::Unanchored),
    };

    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync {
        ro: &Arc::new(ro),
        cache: &cache,
    };

    let text: &[u8] = b"abcde";
    let start = 0;

    let result = exec.find_at(text, start);
    assert_eq!(result, None); // No match expected
}

#[test]
fn test_find_at_with_anchor_end_match() {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::sync::Arc;

    struct TestExecReadOnly {
        res: Vec<String>,
        nfa: Program,
        dfa: Program,
        dfa_reverse: Program,
        suffixes: LiteralSearcher,
        match_type: MatchType,
    }

    let suffixes = LiteralSearcher::new(&[]); // Initialize appropriately
    let ro = TestExecReadOnly {
        res: vec!["abc".to_string()],
        nfa: Program::default(), // Use your appropriate initialization here
        dfa: Program::default(), // Use your appropriate initialization here
        dfa_reverse: Program::default(), // Use your appropriate initialization here
        suffixes,
        match_type: MatchType::Literal(MatchLiteralType::AnchoredEnd),
    };

    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync {
        ro: &Arc::new(ro),
        cache: &cache,
    };

    let text: &[u8] = b"defabc"; // Ensure text length is more than necessary
    let start = 3; // Starting at 'a'

    let result = exec.find_at(text, start);
    assert_eq!(result, Some((3, 6))); // "abc" is found at position (3, 6)
}

