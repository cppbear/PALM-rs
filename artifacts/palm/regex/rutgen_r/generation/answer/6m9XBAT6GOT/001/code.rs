// Answer 0

#[test]
fn test_repetition_with_empty_match() {
    struct TestHir {
        is_always_utf8: bool,
        is_all_assertions: bool,
        is_anchored_start: bool,
        is_anchored_end: bool,
        is_any_anchored_start: bool,
        is_any_anchored_end: bool,
        is_match_empty: bool,
    }

    impl TestHir {
        fn is_always_utf8(&self) -> bool {
            self.is_always_utf8
        }

        fn is_all_assertions(&self) -> bool {
            self.is_all_assertions
        }

        fn is_anchored_start(&self) -> bool {
            self.is_anchored_start
        }

        fn is_anchored_end(&self) -> bool {
            self.is_anchored_end
        }

        fn is_any_anchored_start(&self) -> bool {
            self.is_any_anchored_start
        }

        fn is_any_anchored_end(&self) -> bool {
            self.is_any_anchored_end
        }

        fn is_match_empty(&self) -> bool {
            self.is_match_empty
        }
    }

    struct Repetition {
        hir: TestHir,
    }

    impl Repetition {
        fn is_match_empty(&self) -> bool {
            self.hir.is_match_empty()
        }
    }

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

    struct Hir {
        kind: HirKind,
        info: HirInfo,
    }

    enum HirKind {
        Repetition(Repetition),
    }

    let rep = Repetition {
        hir: TestHir {
            is_always_utf8: true,
            is_all_assertions: false,
            is_anchored_start: false,
            is_anchored_end: true,
            is_any_anchored_start: true,
            is_any_anchored_end: false,
            is_match_empty: true,
        },
    };

    let result = repetition(rep);

    assert!(matches!(result.kind, HirKind::Repetition(_)));
    assert!(result.info.match_empty);
    assert!(!result.info.anchored_start);
    assert!(result.info.anchored_end);
}

