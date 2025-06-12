// Answer 0

#[test]
fn test_anchor_start_text() {
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

    enum Anchor {
        StartText,
        EndText,
    }

    enum HirKind {
        Anchor(Anchor),
    }

    struct Hir {
        kind: HirKind,
        info: HirInfo,
    }

    fn anchor(anchor: Anchor) -> Hir {
        let mut info = HirInfo::new();
        info.set_always_utf8(true);
        info.set_all_assertions(true);
        info.set_anchored_start(false);
        info.set_anchored_end(false);
        info.set_any_anchored_start(false);
        info.set_any_anchored_end(false);
        info.set_match_empty(true);
        if let Anchor::StartText = anchor {
            info.set_anchored_start(true);
            info.set_any_anchored_start(true);
        }
        if let Anchor::EndText = anchor {
            info.set_anchored_end(true);
            info.set_any_anchored_end(true);
        }
        Hir {
            kind: HirKind::Anchor(anchor),
            info: info,
        }
    }

    let result = anchor(Anchor::StartText);
    assert!(match result.info.anchored_start {
        true => true,
        false => false,
    });
    assert!(result.info.all_assertions);
}

#[test]
fn test_anchor_end_text() {
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

    enum Anchor {
        StartText,
        EndText,
    }

    enum HirKind {
        Anchor(Anchor),
    }

    struct Hir {
        kind: HirKind,
        info: HirInfo,
    }

    fn anchor(anchor: Anchor) -> Hir {
        let mut info = HirInfo::new();
        info.set_always_utf8(true);
        info.set_all_assertions(true);
        info.set_anchored_start(false);
        info.set_anchored_end(false);
        info.set_any_anchored_start(false);
        info.set_any_anchored_end(false);
        info.set_match_empty(true);
        if let Anchor::StartText = anchor {
            info.set_anchored_start(true);
            info.set_any_anchored_start(true);
        }
        if let Anchor::EndText = anchor {
            info.set_anchored_end(true);
            info.set_any_anchored_end(true);
        }
        Hir {
            kind: HirKind::Anchor(anchor),
            info: info,
        }
    }

    let result = anchor(Anchor::EndText);
    assert!(match result.info.anchored_end {
        true => true,
        false => false,
    });
    assert!(result.info.all_assertions);
}

