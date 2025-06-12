// Answer 0

#[test]
fn test_visit_with_err_in_visit_pre() {
    struct MockVisitor {
        pre_visit_fail: bool,
    }

    impl MockVisitor {
        fn new(pre_visit_fail: bool) -> Self {
            MockVisitor { pre_visit_fail }
        }
    }

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}

        fn visit_pre(&mut self, _: &Hir) -> Result<(), Self::Err> {
            if self.pre_visit_fail {
                Err(())
            } else {
                Ok(())
            }
        }

        fn visit_post(&mut self, _: &Hir) -> Result<(), Self::Err> {
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let hir = Hir { kind: HirKind::Group(Group { hir: HirKind::empty() }), info: HirInfo::default() };
    let mut visitor = MockVisitor::new(true);
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.visit(&hir, visitor);
    assert!(result.is_err());
}

#[test]
fn test_visit_no_induction_case() {
    struct MockVisitor;

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}

        fn visit_pre(&mut self, _: &Hir) -> Result<(), Self::Err> {
            Ok(())
        }

        fn visit_post(&mut self, _: &Hir) -> Result<(), Self::Err> {
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let hir = Hir { kind: HirKind::Concat(vec![]), info: HirInfo::default() };
    let mut visitor = MockVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.visit(&hir, visitor);
    assert!(result.is_ok());
}

#[test]
fn test_visit_with_empty_alternation() {
    struct MockVisitor {
        alternation_call: bool,
    }

    impl MockVisitor {
        fn new() -> Self {
            MockVisitor { alternation_call: false }
        }
    }

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}

        fn visit_pre(&mut self, _: &Hir) -> Result<(), Self::Err> {
            Ok(())
        }

        fn visit_post(&mut self, _: &Hir) -> Result<(), Self::Err> {
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            self.alternation_call = true;
            Ok(())
        }
    }

    let hir = Hir { kind: HirKind::Alternation(vec![]), info: HirInfo::default() };
    let mut visitor = MockVisitor::new();
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.visit(&hir, visitor);
    assert!(result.is_ok());
}

