// Answer 0

fn test_visit_with_concat() -> Result<(), Box<dyn std::error::Error>> {
    struct MockVisitor {
        should_pre_visit: bool,
        should_post_visit: bool,
    }

    impl MockVisitor {
        fn new(should_pre_visit: bool, should_post_visit: bool) -> Self {
            Self {
                should_pre_visit,
                should_post_visit,
            }
        }
    }

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = Box<dyn std::error::Error>;

        fn start(&mut self) {}
        fn visit_pre(&mut self, _: &Hir) -> Result<(), Self::Err> {
            if self.should_pre_visit {
                Ok(())
            } else {
                Err("visit_pre failed".into())
            }
        }
        fn visit_post(&mut self, _: &Hir) -> Result<(), Self::Err> {
            if self.should_post_visit {
                Ok(())
            } else {
                Err("visit_post failed".into())
            }
        }
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let repetition = hir::Repetition {
        hir: Hir {
            kind: HirKind::Repetition(Box::new([])),
            info: Default::default(),
        },
        ..Default::default()
    };

    let concat_hir = Hir {
        kind: HirKind::Concat(vec![Hir {
            kind: HirKind::Group(Box::new(HirKind::Repetition(Box::new([])))),
            info: Default::default(),
        }]),
        info: Default::default(),
    };

    let mut visitor = MockVisitor::new(true, true);

    let mut visitor_instance = HeapVisitor::new();
    visitor_instance.visit(&concat_hir, visitor)?;

    Ok(())
}

fn test_visit_with_alternation() -> Result<(), Box<dyn std::error::Error>> {
    struct MockVisitor {
        should_pre_visit: bool,
        should_post_visit: bool,
    }

    impl MockVisitor {
        fn new(should_pre_visit: bool, should_post_visit: bool) -> Self {
            Self {
                should_pre_visit,
                should_post_visit,
            }
        }
    }

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = Box<dyn std::error::Error>;

        fn start(&mut self) {}
        fn visit_pre(&mut self, _: &Hir) -> Result<(), Self::Err> {
            if self.should_pre_visit {
                Ok(())
            } else {
                Err("visit_pre failed".into())
            }
        }
        fn visit_post(&mut self, _: &Hir) -> Result<(), Self::Err> {
            if self.should_post_visit {
                Ok(())
            } else {
                Err("visit_post failed".into())
            }
        }
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let alternation_hir = Hir {
        kind: HirKind::Alternation(vec![
            Hir {
                kind: HirKind::Group(Box::new(HirKind::Repetition(Box::new([])))),
                info: Default::default(),
            },
            Hir {
                kind: HirKind::Group(Box::new(HirKind::Repetition(Box::new([])))),
                info: Default::default(),
            },
        ]),
        info: Default::default(),
    };

    let mut visitor = MockVisitor::new(true, true);

    let mut visitor_instance = HeapVisitor::new();
    visitor_instance.visit(&alternation_hir, visitor)?;

    Ok(())
}

#[test]
fn test_visit() {
    assert!(test_visit_with_concat().is_ok());
    assert!(test_visit_with_alternation().is_ok());
}

