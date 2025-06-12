// Answer 0

#[derive(Debug)]
struct HIR {
    info: Info,
}

#[derive(Debug)]
struct Info {
    anchored_end: bool,
}

impl Info {
    fn is_any_anchored_end(&self) -> bool {
        self.anchored_end
    }
}

impl HIR {
    pub fn is_any_anchored_end(&self) -> bool {
        self.info.is_any_anchored_end()
    }
}

#[test]
fn test_is_any_anchored_end_with_anchored_end() {
    let hir = HIR {
        info: Info { anchored_end: true },
    };
    assert!(hir.is_any_anchored_end());
}

#[test]
fn test_is_any_anchored_end_without_anchored_end() {
    let hir = HIR {
        info: Info { anchored_end: false },
    };
    assert!(!hir.is_any_anchored_end());
}

#[test]
fn test_is_any_anchored_end_with_uninitialized_value() {
    let hir = HIR {
        info: Info { anchored_end: false },
    };
    assert!(!hir.is_any_anchored_end());
}

