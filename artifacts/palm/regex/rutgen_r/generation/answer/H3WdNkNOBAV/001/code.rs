// Answer 0

#[test]
fn test_empty_hir() {
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
        fn new() -> HirInfo {
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
        Empty,
    }

    struct Hir {
        kind: HirKind,
        info: HirInfo,
    }

    fn empty() -> Hir {
        let mut info = HirInfo::new();
        info.set_always_utf8(true);
        info.set_all_assertions(true);
        info.set_anchored_start(false);
        info.set_anchored_end(false);
        info.set_any_anchored_start(false);
        info.set_any_anchored_end(false);
        info.set_match_empty(true);
        Hir {
            kind: HirKind::Empty,
            info: info,
        }
    }

    let hir = empty();
    
    assert_eq!(hir.kind, HirKind::Empty);
    assert_eq!(hir.info.always_utf8, true);
    assert_eq!(hir.info.all_assertions, true);
    assert_eq!(hir.info.anchored_start, false);
    assert_eq!(hir.info.anchored_end, false);
    assert_eq!(hir.info.any_anchored_start, false);
    assert_eq!(hir.info.any_anchored_end, false);
    assert_eq!(hir.info.match_empty, true);
}

