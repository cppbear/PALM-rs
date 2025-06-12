// Answer 0

#[test]
fn test_concat_empty() {
    let exprs: Vec<Hir> = Vec::new();
    let result = concat(exprs);
    assert_eq!(result, Hir::empty());
}

#[test]
fn test_concat_single() {
    struct TestHir {
        always_utf8: bool,
        all_assertions: bool,
        anchored_start: bool,
        anchored_end: bool,
        match_empty: bool,
    }
    
    impl Hir for TestHir {
        fn is_always_utf8(&self) -> bool { self.always_utf8 }
        fn is_all_assertions(&self) -> bool { self.all_assertions }
        fn is_any_anchored_start(&self) -> bool { self.anchored_start }
        fn is_any_anchored_end(&self) -> bool { self.anchored_end }
        fn is_match_empty(&self) -> bool { self.match_empty }
    }
    
    let exprs = vec![TestHir { always_utf8: false, all_assertions: false, anchored_start: false, anchored_end: false, match_empty: false }];
    let result = concat(exprs);
    
    assert_eq!(result.kind, HirKind::Concat(exprs));
    assert!(result.info.is_always_utf8());
    assert!(result.info.is_all_assertions());
    assert!(result.info.is_any_anchored_start());
    assert!(result.info.is_any_anchored_end());
    assert!(result.info.is_match_empty());
}

#[test]
fn test_concat_multiple() {
    struct TestHir {
        always_utf8: bool,
        all_assertions: bool,
        anchored_start: bool,
        anchored_end: bool,
        match_empty: bool,
    }
    
    impl Hir for TestHir {
        fn is_always_utf8(&self) -> bool { self.always_utf8 }
        fn is_all_assertions(&self) -> bool { self.all_assertions }
        fn is_any_anchored_start(&self) -> bool { self.anchored_start }
        fn is_any_anchored_end(&self) -> bool { self.anchored_end }
        fn is_match_empty(&self) -> bool { self.match_empty }
    }
    
    let exprs = vec![
        TestHir { always_utf8: false, all_assertions: false, anchored_start: false, anchored_end: false, match_empty: false },
        TestHir { always_utf8: false, all_assertions: false, anchored_start: false, anchored_end: false, match_empty: false }
    ];
    
    let result = concat(exprs);
    
    assert_eq!(result.kind, HirKind::Concat(exprs));
    assert!(!result.info.is_always_utf8());
    assert!(!result.info.is_all_assertions());
    assert!(!result.info.is_any_anchored_start());
    assert!(!result.info.is_any_anchored_end());
    assert!(!result.info.is_match_empty());
}

