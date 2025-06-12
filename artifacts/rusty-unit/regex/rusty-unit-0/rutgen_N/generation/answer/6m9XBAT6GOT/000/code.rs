// Answer 0

#[test]
fn test_repetition_non_empty_match() {
    struct DummyHir {
        always_utf8: bool,
        all_assertions: bool,
        anchored_start: bool,
        anchored_end: bool,
        any_anchored_start: bool,
        any_anchored_end: bool,
        match_empty: bool,
    }

    impl DummyHir {
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

    struct Repetition {
        hir: DummyHir,
    }

    impl Repetition {
        fn is_match_empty(&self) -> bool {
            self.hir.is_match_empty()
        }
    }

    let repetition = Repetition {
        hir: DummyHir {
            always_utf8: true,
            all_assertions: false,
            anchored_start: true,
            anchored_end: false,
            any_anchored_start: false,
            any_anchored_end: true,
            match_empty: false,
        },
    };

    let result = repetition(repetition);

    assert!(result.info.is_always_utf8());
    assert!(!result.info.is_all_assertions());
    assert!(result.info.is_anchored_start());
    assert!(!result.info.is_anchored_end());
    assert!(!result.info.is_any_anchored_start());
    assert!(result.info.is_any_anchored_end());
    assert!(!result.info.is_match_empty());
}

#[test]
fn test_repetition_empty_match() {
    struct DummyHir {
        always_utf8: bool,
        all_assertions: bool,
        anchored_start: bool,
        anchored_end: bool,
        any_anchored_start: bool,
        any_anchored_end: bool,
        match_empty: bool,
    }

    impl DummyHir {
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

    struct Repetition {
        hir: DummyHir,
    }

    impl Repetition {
        fn is_match_empty(&self) -> bool {
            self.hir.is_match_empty()
        }
    }

    let repetition = Repetition {
        hir: DummyHir {
            always_utf8: true,
            all_assertions: false,
            anchored_start: true,
            anchored_end: false,
            any_anchored_start: false,
            any_anchored_end: true,
            match_empty: true,
        },
    };

    let result = repetition(repetition);

    assert!(result.info.is_always_utf8());
    assert!(!result.info.is_all_assertions());
    assert!(!result.info.is_anchored_start());
    assert!(!result.info.is_anchored_end());
    assert!(!result.info.is_any_anchored_start());
    assert!(result.info.is_any_anchored_end());
    assert!(result.info.is_match_empty());
}

