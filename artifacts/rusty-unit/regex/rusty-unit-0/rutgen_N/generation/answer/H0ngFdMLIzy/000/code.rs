// Answer 0

#[derive(Debug)]
struct Hir {
    kind: HirKind,
    info: HirInfo,
}

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
        HirInfo {
            always_utf8: false,
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
struct Group {
    hir: HirContent,
}

#[derive(Debug)]
struct HirContent {
    always_utf8: bool,
    all_assertions: bool,
    anchored_start: bool,
    anchored_end: bool,
    any_anchored_start: bool,
    any_anchored_end: bool,
    match_empty: bool,
}

impl HirContent {
    fn is_always_utf8(&self) -> bool {
        self.always_utf8
    }

    fn is_all_assertions(&self) -> bool {
        self.all_assertions
    }

    fn is_anchored_start(&self) -> bool {
        self.anchored_start
    }

    fn is_anchored_end(&self) -> bool {
        self.anchored_end
    }

    fn is_any_anchored_start(&self) -> bool {
        self.any_anchored_start
    }

    fn is_any_anchored_end(&self) -> bool {
        self.any_anchored_end
    }

    fn is_match_empty(&self) -> bool {
        self.match_empty
    }
}

#[derive(Debug)]
enum HirKind {
    Group(Group),
}

#[test]
fn test_group_function() {
    let hir_content = HirContent {
        always_utf8: true,
        all_assertions: false,
        anchored_start: true,
        anchored_end: false,
        any_anchored_start: true,
        any_anchored_end: false,
        match_empty: true,
    };

    let group = Group {
        hir: hir_content,
    };

    let result = group(group);

    assert!(result.info.always_utf8);
    assert!(!result.info.all_assertions);
    assert!(result.info.anchored_start);
    assert!(!result.info.anchored_end);
    assert!(result.info.any_anchored_start);
    assert!(!result.info.any_anchored_end);
    assert!(result.info.match_empty);
}

#[test]
fn test_group_function_with_false_values() {
    let hir_content = HirContent {
        always_utf8: false,
        all_assertions: true,
        anchored_start: false,
        anchored_end: true,
        any_anchored_start: false,
        any_anchored_end: true,
        match_empty: false,
    };

    let group = Group {
        hir: hir_content,
    };

    let result = group(group);

    assert!(!result.info.always_utf8);
    assert!(result.info.all_assertions);
    assert!(!result.info.anchored_start);
    assert!(result.info.anchored_end);
    assert!(!result.info.any_anchored_start);
    assert!(result.info.any_anchored_end);
    assert!(!result.info.match_empty);
}

