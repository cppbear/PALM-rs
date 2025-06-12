// Answer 0

#[derive(Default)]
struct HIR {
    info: Info,
}

#[derive(Default)]
struct Info {
    always_utf8: bool,
}

impl Info {
    fn is_always_utf8(&self) -> bool {
        self.always_utf8
    }
}

impl HIR {
    pub fn is_always_utf8(&self) -> bool {
        self.info.is_always_utf8()
    }
}

#[test]
fn test_is_always_utf8_true() {
    let hir = HIR {
        info: Info { always_utf8: true },
    };
    assert!(hir.is_always_utf8());
}

#[test]
fn test_is_always_utf8_false() {
    let hir = HIR {
        info: Info { always_utf8: false },
    };
    assert!(!hir.is_always_utf8());
}

