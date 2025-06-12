// Answer 0

#[test]
fn test_find_literals_anchored_start_found() {
    struct TestNfa {
        prefixes: LiteralSearcher,
        is_anchored_start: bool,
    }

    struct TestExecReadOnly {
        nfa: TestNfa,
    }

    struct TestExecNoSync<'c> {
        ro: &'c TestExecReadOnly,
    }

    let prefixes = LiteralSearcher::prefixes(Literals::empty());
    let test_nfa = TestNfa { prefixes, is_anchored_start: true };
    let test_exec_read_only = TestExecReadOnly { nfa: test_nfa };
    let exec = TestExecNoSync { ro: &test_exec_read_only };

    let result = exec.find_literals(MatchLiteralType::AnchoredStart, b"test text", 0);
    assert!(result.is_some());
}

#[test]
fn test_find_literals_anchored_start_not_found() {
    struct TestNfa {
        prefixes: LiteralSearcher,
        is_anchored_start: bool,
    }

    struct TestExecReadOnly {
        nfa: TestNfa,
    }

    struct TestExecNoSync<'c> {
        ro: &'c TestExecReadOnly,
    }

    let prefixes = LiteralSearcher::prefixes(Literals::empty());
    let test_nfa = TestNfa { prefixes, is_anchored_start: true };
    let test_exec_read_only = TestExecReadOnly { nfa: test_nfa };
    let exec = TestExecNoSync { ro: &test_exec_read_only };

    let result = exec.find_literals(MatchLiteralType::AnchoredStart, b"test text", 1);
    assert!(result.is_none());
}

#[test]
fn test_find_literals_anchored_start_is_false() {
    struct TestNfa {
        prefixes: LiteralSearcher,
        is_anchored_start: bool,
    }

    struct TestExecReadOnly {
        nfa: TestNfa,
    }

    struct TestExecNoSync<'c> {
        ro: &'c TestExecReadOnly,
    }

    let prefixes = LiteralSearcher::prefixes(Literals::empty());
    let test_nfa = TestNfa { prefixes, is_anchored_start: false };
    let test_exec_read_only = TestExecReadOnly { nfa: test_nfa };
    let exec = TestExecNoSync { ro: &test_exec_read_only };

    let result = exec.find_literals(MatchLiteralType::AnchoredStart, b"test text", 0);
    assert!(result.is_none());
}

