// Answer 0


#[cfg(test)]
mod tests {
    use super::*;
    use hir::{HirKind, Hir, HirInfo};

    #[derive(Debug)]
    struct MockVisitor {
        pre_visited: bool,
        post_visited: bool,
    }

    impl MockVisitor {
        fn new() -> Self {
            Self {
                pre_visited: false,
                post_visited: false,
            }
        }
    }

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}

        fn visit_pre(&mut self, _: &Hir) -> Result<(), Self::Err> {
            self.pre_visited = true;
            Ok(())
        }

        fn visit_post(&mut self, _: &Hir) -> Result<(), Self::Err> {
            self.post_visited = true;
            Err(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    #[test]
    fn test_visit_with_valid_induction() {
        let info = HirInfo::default();
        let mut visitor = MockVisitor::new();
        let repet = hir::Repetition::new(info.clone());
        let group = hir::Group::new(info.clone());
        let hir = Hir {
            kind: HirKind::Repetition(repet),
            info,
        };

        let mut heap_visitor = HeapVisitor::new();
        let result = heap_visitor.visit(&hir, visitor);

        assert!(result.is_err());
        assert!(visitor.pre_visited);
        assert!(!visitor.post_visited);
    }

    #[test]
    fn test_visit_with_concat() {
        let info = HirInfo::default();
        let mut visitor = MockVisitor::new();
        let child_hir = Hir {
            kind: HirKind::Group(hir::Group::new(info.clone())),
            info: HirInfo::default(),
        };
        let concat_hir = Hir {
            kind: HirKind::Concat(vec![child_hir]),
            info,
        };

        let mut heap_visitor = HeapVisitor::new();
        let result = heap_visitor.visit(&concat_hir, visitor);

        assert!(result.is_err());
        assert!(visitor.pre_visited);
        assert!(!visitor.post_visited);
    }

    #[test]
    fn test_visit_with_alternation() {
        let info = HirInfo::default();
        let mut visitor = MockVisitor::new();
        let child_hir = Hir {
            kind: HirKind::Group(hir::Group::new(info.clone())),
            info: HirInfo::default(),
        };
        let alternation_hir = Hir {
            kind: HirKind::Alternation(vec![child_hir]),
            info,
        };

        let mut heap_visitor = HeapVisitor::new();
        let result = heap_visitor.visit(&alternation_hir, visitor);

        assert!(result.is_err());
        assert!(visitor.pre_visited);
        assert!(!visitor.post_visited);
    }
}


