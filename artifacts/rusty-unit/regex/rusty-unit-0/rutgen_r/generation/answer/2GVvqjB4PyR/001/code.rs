// Answer 0

#[test]
fn test_is_always_utf8_valid() {
    struct HIR {
        info: Info,
    }

    struct Info {
        always_utf8: bool,
    }

    impl Info {
        fn is_always_utf8(&self) -> bool {
            self.always_utf8
        }
    }

    let hir = HIR {
        info: Info { always_utf8: true },
    };

    assert_eq!(hir.is_always_utf8(), true);
}

#[test]
fn test_is_always_utf8_invalid() {
    struct HIR {
        info: Info,
    }

    struct Info {
        always_utf8: bool,
    }

    impl Info {
        fn is_always_utf8(&self) -> bool {
            self.always_utf8
        }
    }

    let hir = HIR {
        info: Info { always_utf8: false },
    };

    assert_eq!(hir.is_always_utf8(), false);
}

