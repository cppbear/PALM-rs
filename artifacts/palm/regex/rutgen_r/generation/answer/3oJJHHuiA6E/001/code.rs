// Answer 0

#[derive(Debug)]
struct Class {
    always_utf8: bool,
}

impl Class {
    fn new(always_utf8: bool) -> Self {
        Class { always_utf8 }
    }

    fn is_always_utf8(&self) -> bool {
        self.always_utf8
    }
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
enum HirKind {
    Class(Class),
}

#[derive(Debug)]
struct Hir {
    kind: HirKind,
    info: HirInfo,
}

pub fn class(class: Class) -> Hir {
    let mut info = HirInfo::new();
    info.set_always_utf8(class.is_always_utf8());
    info.set_all_assertions(false);
    info.set_anchored_start(false);
    info.set_anchored_end(false);
    info.set_any_anchored_start(false);
    info.set_any_anchored_end(false);
    info.set_match_empty(false);
    Hir {
        kind: HirKind::Class(class),
        info: info,
    }
}

#[test]
fn test_class_with_utf8_true() {
    let class_instance = Class::new(true);
    let result = class(class_instance);
    match result.kind {
        HirKind::Class(ref c) => assert!(c.is_always_utf8()),
        _ => panic!("Unexpected HirKind"),
    }
}

#[test]
fn test_class_with_utf8_false() {
    let class_instance = Class::new(false);
    let result = class(class_instance);
    match result.kind {
        HirKind::Class(ref c) => assert!(!c.is_always_utf8()),
        _ => panic!("Unexpected HirKind"),
    }
}

#[test]
fn test_class_panic_conditions() {
    let class_instance = Class::new(true);
    let result = class(class_instance);
    
    assert_eq!(result.info.always_utf8, true);
    assert_eq!(result.info.all_assertions, false);
    assert_eq!(result.info.anchored_start, false);
    assert_eq!(result.info.anchored_end, false);
    assert_eq!(result.info.any_anchored_start, false);
    assert_eq!(result.info.any_anchored_end, false);
    assert_eq!(result.info.match_empty, false);
}

