// Answer 0

#[test]
fn test_find_at_nothing_match_type() {
    struct TestExecReadOnly {
        match_type: MatchType,
    }

    struct TestExecNoSync<'c> {
        ro: &'c TestExecReadOnly,
    }

    let ro = TestExecReadOnly {
        match_type: MatchType::Nothing,
    };

    let exec_no_sync = TestExecNoSync {
        ro: &ro,
    };

    let result = exec_no_sync.find_at(b"example text", 0);
    assert_eq!(result, None);
}

#[test]
fn test_find_at_anchor_end_match() {
    struct TestExecReadOnly {
        match_type: MatchType,
        is_anchored_end: bool,
    }

    struct TestExecNoSync<'c> {
        ro: &'c TestExecReadOnly,
    }

    impl<'c> TestExecNoSync<'c> {
        fn is_anchor_end_match(&self, text: &[u8]) -> bool {
            self.ro.is_anchored_end
        }

        fn find_at(&self, text: &[u8], start: usize) -> Option<(usize, usize)> {
            if !self.is_anchor_end_match(text) {
                return None;
            }

            match self.ro.match_type {
                MatchType::Nothing => None,
                _ => Some((start, start + 1)), // Placeholder for other match types
            }
        }
    }

    let ro = TestExecReadOnly {
        match_type: MatchType::Nothing,
        is_anchored_end: true,
    };

    let exec_no_sync = TestExecNoSync {
        ro: &ro,
    };

    let result = exec_no_sync.find_at(b"example text", 0);
    assert_eq!(result, None);
}

