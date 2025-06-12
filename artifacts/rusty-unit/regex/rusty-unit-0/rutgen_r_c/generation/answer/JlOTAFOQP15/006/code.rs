// Answer 0

fn test_visit_with_repetition() -> Result<(), Box<dyn std::error::Error>> {
    struct MockVisitor {
        calls: Vec<String>,
    }

    impl MockVisitor {
        fn new() -> Self {
            Self { calls: vec![] }
        }
    }

    impl Visitor for MockVisitor {
        type Output = Vec<String>;
        type Err = Box<dyn std::error::Error>;

        fn start(&mut self) {
            self.calls.push("start".to_string());
        }

        fn visit_pre(&mut self, _hir: &Hir) -> Result<(), Self::Err> {
            self.calls.push("visit_pre".to_string());
            Ok(())
        }

        fn visit_post(&mut self, _hir: &Hir) -> Result<(), Self::Err> {
            self.calls.push("visit_post".to_string());
            Ok(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            self.calls.push("visit_alternation_in".to_string());
            Err("error".into())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.calls)
        }
    }

    let repetition_hir = Hir {
        kind: HirKind::Repetition(ast::Repetition::new(/* appropriate initialization */)),
        info: HirInfo::new(/* appropriate initialization */),
    };

    let mut visitor = MockVisitor::new();
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.visit(&repetition_hir, visitor)?;
    assert_eq!(result, vec!["start", "visit_pre", "visit_post"]);
    Ok(())
}

fn test_visit_with_group() -> Result<(), Box<dyn std::error::Error>> {
    struct MockVisitor {
        calls: Vec<String>,
    }

    impl MockVisitor {
        fn new() -> Self {
            Self { calls: vec![] }
        }
    }

    impl Visitor for MockVisitor {
        type Output = Vec<String>;
        type Err = Box<dyn std::error::Error>;

        fn start(&mut self) {
            self.calls.push("start".to_string());
        }

        fn visit_pre(&mut self, _hir: &Hir) -> Result<(), Self::Err> {
            self.calls.push("visit_pre".to_string());
            Ok(())
        }

        fn visit_post(&mut self, _hir: &Hir) -> Result<(), Self::Err> {
            self.calls.push("visit_post".to_string());
            Ok(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            self.calls.push("visit_alternation_in".to_string());
            Err("error".into())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.calls)
        }
    }

    let group_hir = Hir {
        kind: HirKind::Group(ast::Group::new(/* appropriate initialization */)),
        info: HirInfo::new(/* appropriate initialization */),
    };

    let mut visitor = MockVisitor::new();
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.visit(&group_hir, visitor)?;
    assert_eq!(result, vec!["start", "visit_pre", "visit_post"]);
    Ok(())
}

fn test_visit_with_concat() -> Result<(), Box<dyn std::error::Error>> {
    struct MockVisitor {
        calls: Vec<String>,
    }

    impl MockVisitor {
        fn new() -> Self {
            Self { calls: vec![] }
        }
    }

    impl Visitor for MockVisitor {
        type Output = Vec<String>;
        type Err = Box<dyn std::error::Error>;

        fn start(&mut self) {
            self.calls.push("start".to_string());
        }

        fn visit_pre(&mut self, _hir: &Hir) -> Result<(), Self::Err> {
            self.calls.push("visit_pre".to_string());
            Ok(())
        }

        fn visit_post(&mut self, _hir: &Hir) -> Result<(), Self::Err> {
            self.calls.push("visit_post".to_string());
            Ok(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            self.calls.push("visit_alternation_in".to_string());
            Err("error".into())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.calls)
        }
    }

    let concat_hir = Hir {
        kind: HirKind::Concat(vec![/* fill with valid Hir instances */]),
        info: HirInfo::new(/* appropriate initialization */),
    };

    let mut visitor = MockVisitor::new();
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.visit(&concat_hir, visitor)?;
    assert_eq!(result, vec!["start", "visit_pre", "visit_post"]);
    Ok(())
}

fn test_visit_with_alternation() -> Result<(), Box<dyn std::error::Error>> {
    struct MockVisitor {
        calls: Vec<String>,
    }

    impl MockVisitor {
        fn new() -> Self {
            Self { calls: vec![] }
        }
    }

    impl Visitor for MockVisitor {
        type Output = Vec<String>;
        type Err = Box<dyn std::error::Error>;

        fn start(&mut self) {
            self.calls.push("start".to_string());
        }

        fn visit_pre(&mut self, _hir: &Hir) -> Result<(), Self::Err> {
            self.calls.push("visit_pre".to_string());
            Ok(())
        }

        fn visit_post(&mut self, _hir: &Hir) -> Result<(), Self::Err> {
            self.calls.push("visit_post".to_string());
            Ok(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            self.calls.push("visit_alternation_in".to_string());
            Err("error".into())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.calls)
        }
    }

    let alternation_hir = Hir {
        kind: HirKind::Alternation(vec![/* fill with valid Hir instances */]),
        info: HirInfo::new(/* appropriate initialization */),
    };

    let mut visitor = MockVisitor::new();
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.visit(&alternation_hir, visitor)?;
    assert_eq!(result, vec!["start", "visit_pre", "visit_post"]);
    Ok(())
}

#[test]
fn run_tests() {
    let _ = test_visit_with_repetition();
    let _ = test_visit_with_group();
    let _ = test_visit_with_concat();
    let _ = test_visit_with_alternation();
}

