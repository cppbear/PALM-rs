// Answer 0

#[test]
fn test_word_boundary_ascii_negate() {
    struct HirInfo {
        always_utf8: bool,
        all_assertions: bool,
        anchored_start: bool,
        anchored_end: bool,
        any_anchored_start: bool,
        any_anchored_end: bool,
        match_empty: bool,
    }

    impl HirInfo {
        fn new() -> Self {
            HirInfo {
                always_utf8: true,
                all_assertions: false,
                anchored_start: false,
                anchored_end: false,
                any_anchored_start: false,
                any_anchored_end: false,
                match_empty: false,
            }
        }
        
        fn set_always_utf8(&mut self, value: bool) {
            self.always_utf8 = value;
        }

        fn set_all_assertions(&mut self, value: bool) {
            self.all_assertions = value;
        }

        fn set_anchored_start(&mut self, value: bool) {
            self.anchored_start = value;
        }

        fn set_anchored_end(&mut self, value: bool) {
            self.anchored_end = value;
        }

        fn set_any_anchored_start(&mut self, value: bool) {
            self.any_anchored_start = value;
        }

        fn set_any_anchored_end(&mut self, value: bool) {
            self.any_anchored_end = value;
        }

        fn set_match_empty(&mut self, value: bool) {
            self.match_empty = value;
        }
    }

    enum WordBoundary {
        AsciiNegate,
    }

    struct Hir {
        kind: HirKind,
        info: HirInfo,
    }

    enum HirKind {
        WordBoundary(WordBoundary),
    }

    let word_boundary = WordBoundary::AsciiNegate;
    let mut info = HirInfo::new();
    info.set_always_utf8(false);
    info.set_all_assertions(true);
    info.set_match_empty(true);
    
    let result = word_boundary(word_boundary);

    assert_eq!(result.kind, HirKind::WordBoundary(word_boundary));
    assert_eq!(result.info.always_utf8, false);
    assert_eq!(result.info.all_assertions, true);
    assert_eq!(result.info.match_empty, true);
}

fn word_boundary(word_boundary: WordBoundary) -> Hir {
    let mut info = HirInfo::new();
    info.set_always_utf8(true);
    info.set_all_assertions(true);
    info.set_anchored_start(false);
    info.set_anchored_end(false);
    info.set_any_anchored_start(false);
    info.set_any_anchored_end(false);
    info.set_match_empty(word_boundary.is_negated());
    if let WordBoundary::AsciiNegate = word_boundary {
        info.set_always_utf8(false);
    }
    Hir {
        kind: HirKind::WordBoundary(word_boundary),
        info: info,
    }
}

impl WordBoundary {
    fn is_negated(&self) -> bool {
        matches!(self, WordBoundary::AsciiNegate)
    }
}

