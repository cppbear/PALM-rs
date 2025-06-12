// Answer 0

fn test_visit_with_valid_hir() {
    struct TestVisitor {
        called_pre: bool,
        called_post: bool,
    }
    
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        
        fn start(&mut self) {}
        fn visit_pre(&mut self, _: &Hir) -> Result<(), Self::Err> {
            self.called_pre = true;
            Ok(())
        }
        fn visit_post(&mut self, _: &Hir) -> Result<(), Self::Err> {
            self.called_post = true;
            Ok(())
        }
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let mut visitor = TestVisitor {
        called_pre: false,
        called_post: false,
    };

    let repetition_hir = Hir {
        kind: HirKind::Repetition(Repetition { /* init fields */ }),
        info: HirInfo { /* init fields */ },
    };
    
    let mut heap_visitor = HeapVisitor::new();
    
    let result = heap_visitor.visit(&repetition_hir, visitor);
    
    assert!(result.is_ok());
    assert!(visitor.called_pre);
    assert!(visitor.called_post);
}

fn test_visit_with_empty_concat_hir() {
    struct TestVisitor {
        called_pre: bool,
        called_post: bool,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        
        fn start(&mut self) {}
        fn visit_pre(&mut self, _: &Hir) -> Result<(), Self::Err> {
            self.called_pre = true;
            Ok(())
        }
        fn visit_post(&mut self, _: &Hir) -> Result<(), Self::Err> {
            self.called_post = false; // Forcing it to not be called
            Err(())
        }
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let mut visitor = TestVisitor {
        called_pre: false,
        called_post: false,
    };

    let concat_hir = Hir {
        kind: HirKind::Concat(vec![]),
        info: HirInfo { /* init fields */ },
    };
    
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.visit(&concat_hir, visitor);

    assert!(result.is_err());
    assert!(visitor.called_pre);
    assert!(!visitor.called_post);
}

fn test_visit_with_alternation_hir() {
    struct TestVisitor {
        called_pre: bool,
        called_post: bool,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        
        fn start(&mut self) {}
        fn visit_pre(&mut self, _: &Hir) -> Result<(), Self::Err> {
            self.called_pre = true;
            Ok(())
        }
        fn visit_post(&mut self, _: &Hir) -> Result<(), Self::Err> {
            self.called_post = false; // Forcing it to not be called
            Err(())
        }
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let mut visitor = TestVisitor {
        called_pre: false,
        called_post: false,
    };

    let alternation_hir = Hir {
        kind: HirKind::Alternation(vec![ /* fill with dummy Hir objects */ ]),
        info: HirInfo { /* init fields */ },
    };
    
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.visit(&alternation_hir, visitor);

    assert!(result.is_err());
    assert!(visitor.called_pre);
    assert!(!visitor.called_post);
}

