// Answer 0

#[derive(Debug)]
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
        Self {
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

#[derive(Debug)]
enum WordBoundary {
    AsciiNegate,
    Other,
}

#[derive(Debug)]
struct Hir {
    kind: HirKind,
    info: HirInfo,
}

#[derive(Debug)]
enum HirKind {
    WordBoundary(WordBoundary),
}

#[test]
fn test_word_boundary_normal() {
    let word_boundary = WordBoundary::Other;
    let result = word_boundary(word_boundary);
    assert_eq!(result.info.always_utf8, true);
    assert_eq!(result.info.match_empty, false);
}

#[test]
fn test_word_boundary_negated() {
    let word_boundary = WordBoundary::AsciiNegate;
    let result = word_boundary(word_boundary);
    assert_eq!(result.info.always_utf8, false);
    assert_eq!(result.info.match_empty, true);
}

