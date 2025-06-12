// Answer 0

#[test]
fn test_concat_single_expression() {
    struct DummyHir {
        always_utf8: bool,
        all_assertions: bool,
        anchored_start: bool,
        anchored_end: bool,
        match_empty: bool,
    }

    impl DummyHir {
        fn new(always_utf8: bool, all_assertions: bool, anchored_start: bool, anchored_end: bool, match_empty: bool) -> Self {
            Self {
                always_utf8,
                all_assertions,
                anchored_start,
                anchored_end,
                match_empty,
            }
        }

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

        fn is_match_empty(&self) -> bool {
            self.match_empty
        }
    }

    let exprs = vec![
        DummyHir::new(true, false, false, true, false),
    ];

    let result = concat(exprs);
    match result.kind {
        HirKind::Concat(inner) => {
            assert_eq!(inner.len(), 1);
        },
        _ => panic!("Expected a Concat kind"),
    }
}

#[test]
fn test_concat_empty_expressions() {
    let exprs: Vec<DummyHir> = vec![];
    let result = concat(exprs);
    match result.kind {
        HirKind::Empty => (),
        _ => panic!("Expected an Empty kind"),
    }
} 

#[test]
fn test_concat_multiple_expressions() {
    struct DummyHir {
        always_utf8: bool,
        all_assertions: bool,
        anchored_start: bool,
        anchored_end: bool,
        match_empty: bool,
    }

    impl DummyHir {
        fn new(always_utf8: bool, all_assertions: bool, anchored_start: bool, anchored_end: bool, match_empty: bool) -> Self {
            Self {
                always_utf8,
                all_assertions,
                anchored_start,
                anchored_end,
                match_empty,
            }
        }

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

        fn is_match_empty(&self) -> bool {
            self.match_empty
        }
    }

    let exprs = vec![
        DummyHir::new(true, true, false, false, true),
        DummyHir::new(true, false, true, false, false),
        DummyHir::new(false, true, false, true, false),
    ];

    let result = concat(exprs);
    match result.kind {
        HirKind::Concat(inner) => {
            assert_eq!(inner.len(), 3);
        },
        _ => panic!("Expected a Concat kind"),
    }
}

#[test]
fn test_concat_single_non_empty_expression() {
    struct DummyHir {
        always_utf8: bool,
        all_assertions: bool,
        anchored_start: bool,
        anchored_end: bool,
        match_empty: bool,
    }

    impl DummyHir {
        fn new(always_utf8: bool, all_assertions: bool, anchored_start: bool, anchored_end: bool, match_empty: bool) -> Self {
            Self {
                always_utf8,
                all_assertions,
                anchored_start,
                anchored_end,
                match_empty,
            }
        }

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

        fn is_match_empty(&self) -> bool {
            self.match_empty
        }
    }

    let exprs = vec![
        DummyHir::new(false, false, false, false, false),
    ];

    let result = concat(exprs);
    match result.kind {
        HirKind::Concat(inner) => {
            assert_eq!(inner.len(), 1);
        },
        _ => panic!("Expected a Concat kind"),
    }
}

