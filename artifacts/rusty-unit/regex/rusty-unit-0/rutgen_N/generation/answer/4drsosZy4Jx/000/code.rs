// Answer 0

#[test]
fn test_concat_empty() {
    let exprs: Vec<Hir> = vec![];
    let result = concat(exprs);
    assert_eq!(result, Hir::empty());
}

#[test]
fn test_concat_single_expression() {
    struct SimpleHir {
        always_utf8: bool,
        all_assertions: bool,
        any_anchored_start: bool,
        any_anchored_end: bool,
        match_empty: bool,
    }

    impl Hir for SimpleHir {
        fn is_always_utf8(&self) -> bool {
            self.always_utf8
        }
        
        fn is_all_assertions(&self) -> bool {
            self.all_assertions
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

    let exprs: Vec<Hir> = vec![SimpleHir {
        always_utf8: true,
        all_assertions: false,
        any_anchored_start: false,
        any_anchored_end: false,
        match_empty: true,
    }];
    
    let result = concat(exprs);
    assert_eq!(result.kind, HirKind::Concat(vec![SimpleHir { always_utf8: true, all_assertions: false, any_anchored_start: false, any_anchored_end: false, match_empty: true }]));
}

#[test]
fn test_concat_multiple_expressions() {
    struct SimpleHir {
        always_utf8: bool,
        all_assertions: bool,
        any_anchored_start: bool,
        any_anchored_end: bool,
        match_empty: bool,
    }

    impl Hir for SimpleHir {
        fn is_always_utf8(&self) -> bool {
            self.always_utf8
        }
        
        fn is_all_assertions(&self) -> bool {
            self.all_assertions
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

    let exprs: Vec<Hir> = vec![
        SimpleHir { always_utf8: true, all_assertions: true, any_anchored_start: false, any_anchored_end: false, match_empty: false },
        SimpleHir { always_utf8: true, all_assertions: false, any_anchored_start: true, any_anchored_end: false, match_empty: true },
        SimpleHir { always_utf8: false, all_assertions: true, any_anchored_start: false, any_anchored_end: true, match_empty: false },
    ];

    let result = concat(exprs);
    assert_eq!(result.kind, HirKind::Concat(vec![SimpleHir { always_utf8: true, all_assertions: false, any_anchored_start: true, any_anchored_end: true, match_empty: false }]));
}

