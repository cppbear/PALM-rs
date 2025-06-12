// Answer 0

#[test]
fn test_visit_empty_concat() {
    struct TestVisitor {
        pub output: Vec<u32>,
    }

    impl TestVisitor {
        fn new() -> Self {
            Self { output: Vec::new() }
        }
    }

    impl Visitor for TestVisitor {
        type Output = Vec<u32>;
        type Err = ();

        fn start(&mut self) {}
        fn visit_pre(&mut self, _hir: &Hir) -> Result<(), Self::Err> {
            Ok(())
        }

        fn visit_post(&mut self, _hir: &Hir) -> Result<(), Self::Err> {
            self.output.push(1);
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }
    }

    let empty_concat_hir = Hir {
        kind: HirKind::Concat(vec![]),
        info: HirInfo {},
    };

    let mut visitor = TestVisitor::new();
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.visit(&empty_concat_hir, visitor).unwrap();

    assert_eq!(result, vec![1]);
}

#[test]
fn test_visit_single_repetition() {
    struct TestVisitor {
        pub output: Vec<u32>,
    }

    impl TestVisitor {
        fn new() -> Self {
            Self { output: Vec::new() }
        }
    }

    impl Visitor for TestVisitor {
        type Output = Vec<u32>;
        type Err = ();

        fn start(&mut self) {}
        fn visit_pre(&mut self, _hir: &Hir) -> Result<(), Self::Err> {
            Ok(())
        }

        fn visit_post(&mut self, _hir: &Hir) -> Result<(), Self::Err> {
            self.output.push(2);
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }
    }

    let repetition = Hir {
        kind: HirKind::Repetition(/* construct appropriate repetition object here */),
        info: HirInfo {},
    };

    let mut visitor = TestVisitor::new();
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.visit(&repetition, visitor).unwrap();

    assert_eq!(result, vec![2]);
}

#[test]
fn test_visit_alternation() {
    struct TestVisitor {
        pub output: Vec<u32>,
    }

    impl TestVisitor {
        fn new() -> Self {
            Self { output: Vec::new() }
        }
    }

    impl Visitor for TestVisitor {
        type Output = Vec<u32>;
        type Err = ();

        fn start(&mut self) {}
        fn visit_pre(&mut self, _hir: &Hir) -> Result<(), Self::Err> {
            Ok(())
        }

        fn visit_post(&mut self, _hir: &Hir) -> Result<(), Self::Err> {
            self.output.push(3);
            Ok(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            self.output.push(4);
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }
    }

    let alternation_hir = Hir {
        kind: HirKind::Alternation(vec![
            Hir { kind: HirKind::Repetition(/* ... */), info: HirInfo {} },
            Hir { kind: HirKind::Group(/* ... */), info: HirInfo {} },
        ]),
        info: HirInfo {},
    };

    let mut visitor = TestVisitor::new();
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.visit(&alternation_hir, visitor).unwrap();

    assert_eq!(result, vec![4, 3, 3]);
}

