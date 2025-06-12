// Answer 0

#[test]
fn test_suffixes_repetition_range_exactly() {
    struct TestHir {
        kind: HirKind,
    }

    struct TestLiterals {
        data: Vec<u8>,
    }

    impl TestLiterals {
        fn new() -> Self {
            TestLiterals { data: Vec::new() }
        }

        fn cross_add(&mut self, buf: &[u8]) {
            self.data.extend_from_slice(buf);
        }

        fn cut(&mut self) {
            self.data.clear();
        }

        fn to_empty(&self) -> Self {
            TestLiterals::new()
        }

        fn cross_product(&mut self, other: &Self) -> bool {
            !self.data.is_empty() && !other.data.is_empty()
        }

        fn any_complete(&self) -> bool {
            !self.data.is_empty()
        }
    }

    use regex_syntax::hir::{self, RepetitionKind, RepetitionRange, Group, Literal, Class};

    let repetitions = RepetitionKind::Range(RepetitionRange::Exactly(3));
    let group = Group { hir: Box::new(TestHir { kind: HirKind::Literal(hir::Literal::Byte(b'a')) }) };
    let expr = TestHir { kind: HirKind::Repetition(hir::Repetition { kind: repetitions, hir: Box::new(group), greedy: true }) };
    
    let mut lits = TestLiterals::new();

    suffixes(&expr, &mut lits);

    assert!(!lits.data.is_empty());
}

